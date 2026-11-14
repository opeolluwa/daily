mod m20251225_150349_add_countries;
mod m20261114_000000_sync_log;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> sea_orm::DynIden {
        "orchard_migrations".into_iden()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251225_150349_add_countries::Migration),
            Box::new(m20261114_000000_sync_log::Migration),
        ]
    }
}
