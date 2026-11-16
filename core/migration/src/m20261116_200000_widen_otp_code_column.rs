use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Sqlite {
            return Ok(());
        }

        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE "one_time_password" ALTER COLUMN "code" TYPE varchar(72)"#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Sqlite {
            return Ok(());
        }

        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE "one_time_password" ALTER COLUMN "code" TYPE char(6)"#,
            )
            .await?;

        Ok(())
    }
}
