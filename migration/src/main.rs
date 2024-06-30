use sea_orm_migration::prelude::*;
use migration::Migrator;

#[async_std::main]
async fn main() {
   cli::run_cli(migration::Migrator).await;
}
