use ini::Ini;
use log::{info, debug};

use crate::constants;
use crate::configuration::option;

pub fn init_config() {
    info!("reading {} ...", constants::CONFIG_FILENAME);

    let conf_file = Ini::load_from_file("server.ini").unwrap();
    let server_section = conf_file.section(Some("Server")).unwrap();
    let conf = Box::new(option::Config {
        war3_path: String::from(server_section.get("War3Path").unwrap()),
        listen_addr: String::from(server_section.get("ListenAddress").unwrap()),
        listen_port: server_section.get("ListenPort")
        .unwrap()
        .parse::<i16>()
        .unwrap()
    });
 
    unsafe {
        option::CONFIG = Some(Box::leak(conf));
        debug!("{:?}", option::CONFIG);
    }
}
