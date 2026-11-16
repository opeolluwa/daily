use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Postgres/PGlite: ALTER TYPE "item_type" RENAME VALUE 'todo' TO 'task'
        // already ran in m20261112, updating all rows by OID. The literal 'todo'
        // is no longer valid, so we must skip the UPDATE there.
        if manager.get_database_backend() == DbBackend::Sqlite {
            manager
                .get_connection()
                .execute_unprepared(
                    r#"UPDATE recycle_bin SET item_type = 'task' WHERE item_type = 'todo'"#,
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
