use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("notification_settings")
                    .if_not_exists()
                    .col(pk_uuid("identifier"))
                    .col(
                        ColumnDef::new("tasks_due")
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new("reminders")
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new("sync_complete")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new("app_updates")
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new("user_identifier").char_len(36))
                    .col(timestamp_with_time_zone("created_at"))
                    .col(timestamp_with_time_zone("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("notification_settings").to_owned())
            .await
    }
}
