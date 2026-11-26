use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MinifiedLinks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MinifiedLinks::Identifier)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MinifiedLinks::OriginalUrl).text().not_null())
                    .col(ColumnDef::new(MinifiedLinks::MinifiedUrl).text().not_null())
                    .col(
                        ColumnDef::new(MinifiedLinks::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MinifiedLinks::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MinifiedLinks::ValidUntil)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_minified_links_minified_url")
                    .table(MinifiedLinks::Table)
                    .col(MinifiedLinks::MinifiedUrl)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MinifiedLinks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MinifiedLinks {
    Table,
    Identifier,
    OriginalUrl,
    MinifiedUrl,
    CreatedAt,
    UpdatedAt,
    ValidUntil,
}
