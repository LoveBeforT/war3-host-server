#[derive(Debug)]
pub struct Config{
    pub war3_path: String,
    pub listen_port: i16,
    pub listen_addr: String,
}

pub static mut CONFIG: Option<&mut Config> = None;
