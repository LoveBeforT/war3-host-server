use std::io::{stdout, Write};
use log::info;

use crate::constants::GAME_VERSION;

pub struct Console {

}

impl Console {
    pub fn new() -> Console {
        Console {

        }
    }

    pub fn run(&self) {
        info!("console is running ...");
        info!("enter help for more infomations");

        loop {
            print!("server>");
            stdout().flush().expect("unable to flush console");
            
            // read command
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_n) => {
                    let cmd_line = line.trim_end().trim();
                    let cmds: Vec<&str> = cmd_line.split(" ").collect();
                    let cmd = cmds[0].trim();
                    match cmd {
                        "maplist" => {
                            self.maplist();
                        }
                        "map" => {
                            println!("unimplemented command")
                        }
                        "version" => {
                            self.version();
                        }
                        "help" => {
                            self.help();
                        }
                        "exit" => {
                            break
                        }
                        _ => { 
                            println!("unknown command")
                        }
                    }
                }
                Err(error) => println!("error: {error}"),
            }
        }

        info!("console exited")
    }

    pub fn maplist(&self) {
        println!("unimplemented command")
    }

    pub fn version(&self) {
        println!(
            "\nWarcraft III Host Server v{} by {}",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        );
        println!(
            "supported game version is {}\n",
            GAME_VERSION
        );
    }

    pub fn help(&self) {
        println!("\nnotes: the console input will blocked if the lobby was started");
        println!("Commands:\n");
        println!("\tmaplist\t\tshow contents in maps folder");
        println!("\tmap [mapname]\tcreate a lobby");
        println!("\tversion\t\tshow server version");
        println!("\thelp");
        println!("\texit\n");
    }
}
