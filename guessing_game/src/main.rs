pub struct Guess{
    value: i32, 
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}!", value)
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

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
        let mut input = String::new(); 
        
        // read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // change guess data-type to int
        // conditionally ignore non-int inputs
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(input);

        // if Report = Ok(num), report user input
        println!("You guessed: {}", guess.value);

        // match guesses to sec_num
        match guess.value.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
