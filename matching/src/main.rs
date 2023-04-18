#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
    Arkansas,
}

#[derive(Debug)]
enum Coin {
    Penny, 
    Nickel,
    Dime, 
    Quarter(UsState),
}

fn main() {
    let penny = Coin::Nickel;

    let res = value_cents(&penny);

    println!("You found a coin worth {} cents!", res);
}

fn value_cents(coin: &Coin) -> u32 {
    match coin {
        Coin:: Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}