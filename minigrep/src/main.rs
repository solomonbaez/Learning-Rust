
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // build configuration item
    // built iterator with env::args
    // unwrap result and pass error if not Ok
    let configs = Config::build(env::args())
        .unwrap_or_else(|err| {
            // stop the process
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    // if let is called to check whether run returns an Err
    // no need to unwrap as the Ok() is "null"
    if let Err(e) = minigrep::run(configs){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}