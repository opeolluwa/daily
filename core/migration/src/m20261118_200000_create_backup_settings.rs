use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::DbBackend,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        if backend == DbBackend::Postgres {
            let has_type = manager
                .get_connection()
                .execute_unprepared("SELECT 1 FROM pg_type WHERE typname = 'backup_mode'")
                .await?
                .rows_affected()
                > 0;

            if !has_type {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(BackupModeEnum::Type)
                            .values([
                                BackupModeEnum::LocalOnly,
                                BackupModeEnum::CloudOnly,
                                BackupModeEnum::LocalFirst,
                                BackupModeEnum::CloudFirst,
                            ])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table("backup_settings")
                    .if_not_exists()
                    .col(pk_uuid("identifier"))
                    .col(ColumnDef::new("user_identifier").char_len(36))
                    .col(ColumnDef::new("server_url").string().not_null().default(""))
                    .col(ColumnDef::new("last_sync").timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(BackupModeEnum::BackupMode)
                            .enumeration(
                                BackupModeEnum::Type,
                                [
                                    BackupModeEnum::LocalOnly,
                                    BackupModeEnum::CloudOnly,
                                    BackupModeEnum::LocalFirst,
                                    BackupModeEnum::CloudFirst,
                                ],
                            )
                            .not_null()
                            .default("local_only"),
                    )
                    .col(timestamp_with_time_zone("created_at"))
                    .col(timestamp_with_time_zone("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        manager
            .drop_table(Table::drop().table("backup_settings").to_owned())
            .await?;

        if backend == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared("DROP TYPE IF EXISTS \"backup_mode\"")
                .await?;
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum BackupModeEnum {
    #[sea_orm(iden = "backup_mode")]
    Type,
    BackupMode,
    LocalOnly,
    CloudOnly,
    LocalFirst,
    CloudFirst,
}
