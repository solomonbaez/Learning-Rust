use::std::io;
use::std::time::{SystemTime, UNIX_EPOCH};

//linear congruential generator
#[allow(non_snake_case)]
fn gen_LCG_rand() -> u32 {
    let now = SystemTime::now();
    // UNIX_EPOCH anchors systime
    let since_epoch = now.duration_since(UNIX_EPOCH).
        expect("failed to anchor epoch");

    let seed = since_epoch.as_secs();
    let a: u32 = 1103515245;
    let c: u32 = 12345;
    let m = 2_i64.pow(31) - 1;

    return 
        (a.wrapping_mul(seed as u32).wrapping_add(c)) % m as u32;
}

fn main() {
    let rand = gen_LCG_rand();

    println!("Welcome to GG3.0");
    println!("Please insert your first guess:");
    println!("pick a number between 10-40");
    let mut guess = String::new();

    //take ownership and mutate the guess value
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read guess!");

    let parsed_guess: u32 = guess.trim().parse().unwrap();
    println!("Your guess: {}", parsed_guess);

    if rand == parsed_guess {
        println!("Nice! You guessed correct :)");
    } else {
        println!("Sorry, try again!");
    }

    println!("The correct answer is: {}", rand);
}
