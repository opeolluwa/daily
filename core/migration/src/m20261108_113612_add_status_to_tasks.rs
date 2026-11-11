use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    sea_orm::DbBackend,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261108_113612_add_status_to_tasks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        if backend == DbBackend::Postgres {
            let has_type = manager
                .get_connection()
                .execute_unprepared("SELECT 1 FROM pg_type WHERE typname = 'todo_status'")
                .await?
                .rows_affected()
                > 0;

            if !has_type {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(TodoStatus::Type)
                            .values([
                                TodoStatus::Planned,
                                TodoStatus::InProgress,
                                TodoStatus::Done,
                            ])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        let mut column = ColumnDef::new(TodoStatus::Status);
        column
            .enumeration(
                TodoStatus::Type,
                [
                    TodoStatus::Planned,
                    TodoStatus::InProgress,
                    TodoStatus::Done,
                ],
            )
            .not_null()
            .default("planned");

        if backend == DbBackend::MySql {
            manager
                .alter_table(
                    Table::alter()
                        .table(Todo::Table)
                        .add_column(column)
                        .to_owned(),
                )
                .await?;
        } else {
            manager
                .alter_table(
                    Table::alter()
                        .table(Todo::Table)
                        .add_column_if_not_exists(column)
                        .to_owned(),
                )
                .await?;
        }

        manager
            .get_connection()
            .execute_unprepared("UPDATE todo SET status = 'done' WHERE done = TRUE")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        manager
            .alter_table(
                Table::alter()
                    .table(Todo::Table)
                    .drop_column(TodoStatus::Status)
                    .to_owned(),
            )
            .await?;

        if backend == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared("DROP TYPE IF EXISTS \"todo_status\"")
                .await?;
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Todo {
    Table,
}

#[derive(DeriveIden)]
pub enum TodoStatus {
    #[sea_orm(iden = "todo_status")]
    Type,
    Status,
    Planned,
    InProgress,
    Done,
}
