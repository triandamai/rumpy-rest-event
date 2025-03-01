use crate::common::env_config::EnvConfig;

pub mod delete;
pub mod get;
pub mod insert;
pub mod orm;
pub mod replace;
pub mod update;

pub fn get_db_name() -> String {
    let env = EnvConfig::init();
    format!("RUMPY-{}", env.mode).to_lowercase()
}
