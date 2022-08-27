use crate::bootstrap::config;

pub fn init() {
    println!("initializing server configuration...");

    config::initconfig();

    println!("initialized server configuration");
}
