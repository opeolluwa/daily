use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::{
        meta::RequestMeta,
        paste_bins::{CreatePasteBin, UpdatePasteBin},
        recycle_bin::CreateRecycleBinEntry,
    },
    entities::{paste_bins, recycle_bin},
    error::LunarError,
    repositories::{
        prelude::WorkspaceRepositoryExt,
        recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
        workspace::WorkspaceRepository,
        workspace_manager::{DuplicateRecord, RecordExistInWorkspace, TransferRecord},
    },
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct PasteBinRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait PasteBinRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreatePasteBin,
        meta: &Option<RequestMeta>,
    ) -> Result<paste_bins::Model, LunarError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<paste_bins::Model>, LunarError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<paste_bins::Model>, LunarError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<paste_bins::Model>, LunarError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdatePasteBin,
        meta: &Option<RequestMeta>,
    ) -> Result<paste_bins::Model, LunarError>;
}

#[async_trait]
impl PasteBinRepositoryExt for PasteBinRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreatePasteBin,
        meta: &Option<RequestMeta>,
    ) -> Result<paste_bins::Model, LunarError> {
        let mut active_model: paste_bins::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier));
        } else {
            return Err(LunarError::DbOperationError(
                "workspace identifier is required".into(),
            ));
        };

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<paste_bins::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        paste_bins::Entity::find()
            .filter(paste_bins::Column::Identifier.eq(*identifier))
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<paste_bins::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        paste_bins::Entity::find()
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(paste_bins::Column::UpdatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = paste_bins::Entity::find()
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(paste_bins::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::PasteBinNotFound(identifier.to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        let bin = RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: crate::entities::sea_orm_active_enums::ItemType::PasteBin,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        paste_bins::Entity::delete_many()
            .filter(paste_bins::Column::Identifier.eq(*identifier))
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(bin)
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<paste_bins::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        paste_bins::Entity::find()
            .limit(10)
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(paste_bins::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdatePasteBin,
        meta: &Option<RequestMeta>,
    ) -> Result<paste_bins::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = paste_bins::Entity::find()
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(paste_bins::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::PasteBinNotFound(identifier.to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(Some(title.clone()));
        }
        if let Some(content) = &payload.content {
            active_model.content = Set(content.clone());
        }
        if let Some(is_pinned) = payload.is_pinned {
            active_model.is_pinned = Set(is_pinned);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }
}

#[async_trait::async_trait]
impl TransferRecord for PasteBinRepository {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), LunarError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(LunarError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(LunarError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        if !self
            .record_exists_in_workspace(record_identifier, previous_workspace_identifier)
            .await?
        {
            return Err(LunarError::PasteBinNotFound(record_identifier.to_string()));
        }

        let Some(record) = paste_bins::Entity::find()
            .filter(paste_bins::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::PasteBinNotFound(record_identifier.to_string()));
        };

        let mut active_model = record.into_active_model();

        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model.workspace_identifier = Set(Some(*target_workspace_identifier));

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl RecordExistInWorkspace for PasteBinRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, LunarError> {
        let record = paste_bins::Entity::find()
            .filter(paste_bins::Column::Identifier.eq(*record_identifier))
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}

#[async_trait::async_trait]
impl DuplicateRecord for PasteBinRepository {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<Uuid, LunarError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(LunarError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(LunarError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        let Some(record) = paste_bins::Entity::find()
            .filter(paste_bins::Column::Identifier.eq(*record_identifier))
            .filter(paste_bins::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::PasteBinNotFound(record_identifier.to_string()));
        };

        let mut new_record = record.into_active_model();

        let new_identifier = Uuid::new_v4();
        new_record.identifier = Set(new_identifier);
        new_record.workspace_identifier = Set(Some(*target_workspace_identifier));
        new_record.created_at = Set(Utc::now().fixed_offset());
        new_record.updated_at = Set(Utc::now().fixed_offset());

        new_record
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(new_identifier)
    }
}
