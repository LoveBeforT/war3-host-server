use log::info;
use crate::bootstrap::{config, logger};

pub fn init() {
    logger::init_logger();

    info!("initializing server configuration...");
    config::init_config();
    info!("server configuration done");

}
