use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261113_100000_add_end_date_to_reminder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        let mut column = ColumnDef::new(ReminderCol::EndDate);
        column.date().null();

        if backend == sea_orm::DbBackend::MySql {
            manager
                .alter_table(
                    Table::alter()
                        .table(Reminder::Table)
                        .add_column(column)
                        .to_owned(),
                )
                .await?;
        } else {
            manager
                .alter_table(
                    Table::alter()
                        .table(Reminder::Table)
                        .add_column_if_not_exists(column)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Reminder::Table)
                    .drop_column(ReminderCol::EndDate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Reminder {
    Table,
}

#[derive(DeriveIden)]
pub enum ReminderCol {
    EndDate,
}
