mod bootstrap;
mod constants;
mod server;
mod command;

use bootstrap::init::init;
use command::handler::Console;

fn main() {
    println!(
        "Warcraft III Host Server v{} by {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );

    init();

    Console::new().run();
}
