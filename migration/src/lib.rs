pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240219_065139_initial;
mod m20240219_070643_update_user_credential;
mod m20240219_072548_seeding_user_credential;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240219_065139_initial::Migration),
            Box::new(m20240219_070643_update_user_credential::Migration),
            Box::new(m20240219_072548_seeding_user_credential::Migration),
        ]
    }
}

