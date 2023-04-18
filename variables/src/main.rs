use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    if index != 0 {
        println!("Index is {index}")
    }

    let element = a[index];

    println!("The value you indexed at {index} is: {element}");
    another_func(5);
    
    let r = rand_number();
    println!("Random number is: {r}");

    let p = plus_one(5);
    println!("Plus one is: {p}");
}

fn another_func(x: i32) {
    println!("Value of another function is: {x}");
}

fn rand_number() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
