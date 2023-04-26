
use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

type Kilometers = i32;

fn add_one(x: i32) -> i32 {
    x + 1
}

// function implementing a function pointer
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    
    println!("x + y = {}", x + y);

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_nums = vec![1,2,3];
    let list_of_strs: Vec<String> = 
        list_of_nums.iter().map(ToString::to_string).collect();
    let list_of_stats: Vec<Status> =
        (0u32..20).map(Status::Value).collect();
}