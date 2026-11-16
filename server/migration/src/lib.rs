mod m20251225_150349_add_countries;
mod m20261114_000000_sync_log;
mod m20261116_000000_switch_identifiers_to_ulid;

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
            Box::new(m20261116_000000_switch_identifiers_to_ulid::Migration),
        ]
    }
}
