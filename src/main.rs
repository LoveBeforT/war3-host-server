mod bootstrap;
mod constants;

use log::{info, error, debug};
use bootstrap::init::init;

fn main() {
    println!(
        "Warcraft III Host Server v{} by {}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );

    init();
}
