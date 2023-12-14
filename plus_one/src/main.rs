use std::option::Option;

fn plus_one(value: Option<i32>) -> Option<i32> {
    // no reference -> we need control
    match value {
        Some(value) => Option::Some(value + 1),
        None => Option::None,
    }
}

fn main() {
    let x = Option::<i32>::Some(1);
    let y = plus_one(x);

    println!("{}", y.unwrap())
}
