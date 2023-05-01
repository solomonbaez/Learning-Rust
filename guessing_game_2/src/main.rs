struct Guess<'a> {
    val: i32,
    truth: &'a i32,
}

impl<'a> Guess<'a> {
    fn new(guess: i32, value: &'a i32) -> Guess {
        Guess { val: guess, truth: value }
    }

    fn compare(&self) -> bool {
        if self.val == *self.truth {
            println!("Correct!");
            return true;
        }
        else if self.val < *self.truth {
            println!("Too low!");
            return false;
        }
        else if self.val > *self.truth {
            println!("Too high!");
            return false;
        }
        else { false }
    }
}

use std::io;
use rand::Rng;

fn main() {

    let ground_truth = rand::thread_rng().gen_range(1..100);
    println!("Guess a number between 1 and 100! ");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input = match input.trim().parse::<i32>() {
            Ok(num) => {
                if num <= 100 && num >= 1 { num }
                else {
                    println!("{} is not a valid number!", input);
                    continue;
                }
            }
            Err(_) => {
                println!("{} is not a valid number!", input);
                continue;
            },
        };

        let guess = Guess::new(input, &ground_truth);
            
        if guess.compare() { break };
    };
}