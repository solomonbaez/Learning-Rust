// use std::io::ErrorKind;
use std::fs::File;
use std::error::Error;


// only match operations, no closures
fn main() -> Result<(), Box<dyn Error>> {
    //let greeting_file_res = File::open("hello.txt");

    // let greeting_file = match greeting_file_res {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    
    // let user = read_username_from_file();
    // println!("Username from {:?} is: {:?}", greeting_file, user);
    
    let username = read_username_from_file();
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    //let mut username = String::new();
    //File::open("hello.txt")?.read_to_string(&mut username)?;
    //Ok(username)

    fs::read_to_string("hello.txt")
}
