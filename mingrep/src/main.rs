use std::process;
use std::env;

use mingrep;
use mingrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });
    //cfg.show();
    if let Err(e) = mingrep::run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

