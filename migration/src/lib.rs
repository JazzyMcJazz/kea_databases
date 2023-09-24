pub use sea_orm_migration::prelude::*;

mod m20230912_000001_create_tables;
mod m20230912_000002_add_triggers;
mod m20230912_000003_functions;
mod m20230912_000004_stored_procedures;
mod m20230912_000005_events;
mod m20230912_000006_views;
mod m20230912_000011_dummy_data;
mod m20230912_000012_dummy_data_2;
mod m20230912_000013_produce_item_drops;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230912_000001_create_tables::Migration),
            Box::new(m20230912_000002_add_triggers::Migration),
            Box::new(m20230912_000003_functions::Migration),
            Box::new(m20230912_000004_stored_procedures::Migration),
            Box::new(m20230912_000005_events::Migration),
            Box::new(m20230912_000006_views::Migration),
            Box::new(m20230912_000011_dummy_data::Migration),
            Box::new(m20230912_000012_dummy_data_2::Migration),
            Box::new(m20230912_000013_produce_item_drops::Migration),
        ]
    }
}
