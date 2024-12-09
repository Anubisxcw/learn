mod config;
mod dbo;

use crate::config::load_or_init_config;
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::{params, Connection, Result};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::dbo::init_db;

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    tracing::info!("Server starting up");
    tracing::info!("Reading config file from config.toml");
    let config = match load_or_init_config() {
        Ok(v) => v,
        Err(err) => {
            match err {
                config::ConfigError::IoError(err) => {
                    tracing::error!("Error reading config file: {}", err)
                }
                config::ConfigError::InvalidConfig(err) => {
                    tracing::error!("Invalid config file: {}", err);
                }
            }
            return;
        }
    };
    tracing::info!("Loaded config: {:?}", config);
    let conn = Connection::open(config.get_db_file_name());
    init_db(conn.expect("init failed"));
}


