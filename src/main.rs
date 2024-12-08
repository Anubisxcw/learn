mod config;
//use duckdb::{params, Connection, Result};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::load_or_init_config;

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    
    let config = match load_or_init_config(){
        Ok(v)=>v,
        Err(err)=>{
            match err {
                config::ConfigError::IoError(err)=>{
                    tracing::error!("Error reading config file: {}", err)
                }
                config::ConfigError::InvalidConfig(err)=>{
                    tracing::error!("Invalid config file: {}", err);
                }
            }
            return;
        }
    };
    tracing::info!("Loaded config: {:#?}", config);
    
    
}


