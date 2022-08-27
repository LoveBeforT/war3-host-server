use log4rs;

pub fn init_logger(){
    // set logger
    log4rs::init_file("logger.yml", Default::default()).unwrap();
}
