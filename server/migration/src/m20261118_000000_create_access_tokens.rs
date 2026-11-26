use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccessTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccessTokens::Identifier)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AccessTokens::UserIdentifier)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccessTokens::Token).text().not_null())
                    .col(
                        ColumnDef::new(AccessTokens::ResourceIdentifier)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccessTokens::ResourceType).text().not_null())
                    .col(
                        ColumnDef::new(AccessTokens::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AccessTokens::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_access_tokens_user_identifier")
                            .from(AccessTokens::Table, AccessTokens::UserIdentifier)
                            .to(Users::Table, Users::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccessTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AccessTokens {
    Table,
    Identifier,
    UserIdentifier,
    Token,
    ResourceIdentifier,
    ResourceType,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Identifier,
}
