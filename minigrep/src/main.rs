
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
            println!("Problem parsing arguments: {}", err);
            // stop the process
            process::exit(1);
        });

    println!("Searching for query: {}", configs.query);
    println!("In file {}", configs.file_path);

    // if let is called to check whether run returns an Err
    // no need to unwrap as the Ok() is "null"
    if let Err(e) = minigrep::run(configs){
        println!("Application error: {}", e);
        process::exit(1);
    }
}