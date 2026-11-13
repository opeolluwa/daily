use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    sea_orm::DbBackend,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261113_000000_add_recurrence_type_to_reminder"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        if backend == DbBackend::Postgres {
            let has_type = manager
                .get_connection()
                .execute_unprepared(
                    "SELECT 1 FROM pg_type WHERE typname = 'reminder_recurrence'",
                )
                .await?
                .rows_affected()
                > 0;

            if !has_type {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(ReminderRecurrence::Type)
                            .values([
                                ReminderRecurrence::Daily,
                                ReminderRecurrence::Weekly,
                                ReminderRecurrence::Biweekly,
                                ReminderRecurrence::Monthly,
                                ReminderRecurrence::Yearly,
                                ReminderRecurrence::Weekdays,
                            ])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        let mut column = ColumnDef::new(ReminderRecurrence::RecurrenceType);
        column
            .enumeration(
                ReminderRecurrence::Type,
                [
                    ReminderRecurrence::Daily,
                    ReminderRecurrence::Weekly,
                    ReminderRecurrence::Biweekly,
                    ReminderRecurrence::Monthly,
                    ReminderRecurrence::Yearly,
                    ReminderRecurrence::Weekdays,
                ],
            )
            .null();

        if backend == DbBackend::MySql {
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
        let backend = manager.get_database_backend();

        manager
            .alter_table(
                Table::alter()
                    .table(Reminder::Table)
                    .drop_column(ReminderRecurrence::RecurrenceType)
                    .to_owned(),
            )
            .await?;

        if backend == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared("DROP TYPE IF EXISTS \"reminder_recurrence\"")
                .await?;
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Reminder {
    Table,
}

#[derive(DeriveIden)]
pub enum ReminderRecurrence {
    #[sea_orm(iden = "reminder_recurrence")]
    Type,
    RecurrenceType,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Yearly,
    Weekdays,
}
