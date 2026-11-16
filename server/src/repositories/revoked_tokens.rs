use std::sync::Arc;

use chrono::DateTime;
use kernel::entities::revoked_token;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::errors::database_error::DatabaseError;
use crate::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct TokenBlacklistRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for TokenBlacklistRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

pub(crate) trait TokenBlacklistRepositoryTrait {
    async fn revoke_token(
        &self,
        jti: &str,
        user_identifier: &str,
        expires_at: DateTime<chrono::FixedOffset>,
    ) -> Result<(), DatabaseError>;

    async fn is_revoked(&self, jti: &str) -> Result<bool, DatabaseError>;
}

impl TokenBlacklistRepositoryTrait for TokenBlacklistRepository {
    async fn revoke_token(
        &self,
        jti: &str,
        user_identifier: &str,
        expires_at: DateTime<chrono::FixedOffset>,
    ) -> Result<(), DatabaseError> {
        let record = revoked_token::ActiveModel {
            identifier: Set(kernel::id::new_id()),
            jti: Set(jti.to_string()),
            user_identifier: Set(user_identifier.to_string()),
            expires_at: Set(expires_at),
            revoked_at: Set(chrono::Utc::now().fixed_offset()),
        };

        record
            .insert(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn is_revoked(&self, jti: &str) -> Result<bool, DatabaseError> {
        let record = revoked_token::Entity::find()
            .filter(revoked_token::Column::Jti.eq(jti))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;
        Ok(record.is_some())
    }
}
