use::std::io;

fn main() {
    let seed = String::from("22");

    println!("Welcome to GG3.0");
    println!("Please insert your first guess:");
    println!("pick a number between 10-40");
    let mut guess = String::new();

    //take ownership and mutate the guess value
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read guess!");

    let trimmed = guess.trim();
    println!("Your guess: {}", trimmed);
    if seed == trimmed {
        println!("Nice! You guessed correct :)");
    } else {
        println!("Sorry, try again!");
    }
    println!("The correct answer is: {}", seed);
}
