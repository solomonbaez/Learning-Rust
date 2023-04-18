// import libraries
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// main function: guessing game
fn main() {
    println!("Guess a number!");

    // generate a random integer
    let sec_num = rand::thread_rng().gen_range(1..=100);

    // accept user input until sec_num is guessed
    loop {
        println!("Please input your guess...");

        // mutable guess variable, user input
        let mut guess = String::new(); 
        
        // read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // change guess data-type to int
        // conditionally ignore non-int inputs
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // if Report = Ok(num), report user input
        println!("You guessed: {guess}");

        // match guesses to sec_num
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
