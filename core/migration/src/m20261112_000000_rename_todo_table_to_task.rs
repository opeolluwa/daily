use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(Table::rename().table("todo", "task").to_owned())
            .await?;

        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared(r#"ALTER TYPE "todo_status" RENAME TO "task_status""#)
                .await?;

            manager
                .get_connection()
                .execute_unprepared(r#"ALTER TYPE "item_type" RENAME VALUE 'todo' TO 'task'"#)
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared(r#"ALTER TYPE "item_type" RENAME VALUE 'task' TO 'todo'"#)
                .await?;

            manager
                .get_connection()
                .execute_unprepared(r#"ALTER TYPE "task_status" RENAME TO "todo_status""#)
                .await?;
        }

        manager
            .rename_table(Table::rename().table("task", "todo").to_owned())
            .await?;

        Ok(())
    }
}
