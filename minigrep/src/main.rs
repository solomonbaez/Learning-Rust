
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // pull cl-args, collect specifies the collection
    let args: Vec<String> = env::args().collect();
    
    // build configuration item
    // unwrap result and pass error if not Ok
    let configs = Config::build(&args)
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