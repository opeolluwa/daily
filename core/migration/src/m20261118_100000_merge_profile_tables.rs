use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("user_profile")
                    .if_not_exists()
                    .col(pk_uuid("identifier"))
                    .col(ColumnDef::new("first_name").string().not_null().default(""))
                    .col(ColumnDef::new("last_name").string().not_null().default(""))
                    .col(ColumnDef::new("email").string().not_null().default(""))
                    .col(ColumnDef::new("profile_picture").string())
                    .col(timestamp_with_time_zone("created_at"))
                    .col(timestamp_with_time_zone("updated_at"))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db.execute_unprepared(
            "INSERT INTO user_profile (identifier, first_name, last_name, email, profile_picture, created_at, updated_at)
             SELECT
                 wp.identifier,
                 COALESCE(wp.first_name, ''),
                 COALESCE(wp.last_name, ''),
                 COALESCE(up.master_email, ''),
                 wp.profile_picture,
                 wp.created_at,
                 wp.updated_at
             FROM workspace_profiles wp
             LEFT JOIN user_preferences up ON 1=1
             WHERE wp.deleted_at IS NULL
             ORDER BY wp.updated_at DESC
             LIMIT 1",
        )
        .await
        .ok();

        manager
            .drop_table(Table::drop().table("workspace_profiles").if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("user_preferences").if_exists().to_owned())
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
