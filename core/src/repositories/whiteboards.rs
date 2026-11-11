use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::{meta::RequestMeta, whiteboards::{CreateWhiteboard, UpdateWhiteboard}},
    entities::whiteboards,
    error::KernelError,
    repositories::{
        prelude::WorkspaceRepositoryExt,
        workspace::WorkspaceRepository,
        workspace_manager::{RecordExistInWorkspace, TransferRecord},
    },
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct WhiteboardRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait WhiteboardRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateWhiteboard,
        meta: &Option<RequestMeta>,
    ) -> Result<whiteboards::Model, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<whiteboards::Model>, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<whiteboards::Model>, KernelError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<whiteboards::Model, KernelError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<whiteboards::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateWhiteboard,
        meta: &Option<RequestMeta>,
    ) -> Result<whiteboards::Model, KernelError>;
}

#[async_trait]
impl WhiteboardRepositoryExt for WhiteboardRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateWhiteboard,
        meta: &Option<RequestMeta>,
    ) -> Result<whiteboards::Model, KernelError> {
        let mut active_model: whiteboards::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier));
        } else {
            return Err(KernelError::DbOperationError(
                "workspace identifier is required".into(),
            ));
        };

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<whiteboards::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        whiteboards::Entity::find()
            .filter(whiteboards::Column::Identifier.eq(*identifier))
            .filter(whiteboards::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<whiteboards::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        whiteboards::Entity::find()
            .filter(whiteboards::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(whiteboards::Column::UpdatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<whiteboards::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = whiteboards::Entity::find()
            .filter(whiteboards::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(whiteboards::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::WhiteboardNotFound(identifier.to_string()))?;

        whiteboards::Entity::delete(whiteboards::ActiveModel {
            identifier: Set(model.identifier),
            ..Default::default()
        })
        .exec(self.conn.as_ref())
        .await
        .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(model)
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<whiteboards::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        whiteboards::Entity::find()
            .limit(10)
            .filter(whiteboards::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(whiteboards::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateWhiteboard,
        meta: &Option<RequestMeta>,
    ) -> Result<whiteboards::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = whiteboards::Entity::find()
            .filter(whiteboards::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(whiteboards::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::WhiteboardNotFound(identifier.to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(Some(title.clone()));
        }
        if let Some(payload) = &payload.payload {
            active_model.payload = Set(Some(payload.clone()));
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}

#[async_trait::async_trait]
impl TransferRecord for WhiteboardRepository {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(KernelError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(KernelError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        if !self
            .record_exists_in_workspace(record_identifier, previous_workspace_identifier)
            .await?
        {
            return Err(KernelError::WhiteboardNotFound(
                record_identifier.to_string(),
            ));
        }

        let Some(record) = whiteboards::Entity::find()
            .filter(whiteboards::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::WhiteboardNotFound(
                record_identifier.to_string(),
            ));
        };

        let mut active_model = record.into_active_model();

        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model.workspace_identifier = Set(Some(*target_workspace_identifier));

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl RecordExistInWorkspace for WhiteboardRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, KernelError> {
        let record = whiteboards::Entity::find()
            .filter(whiteboards::Column::Identifier.eq(*record_identifier))
            .filter(whiteboards::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}
