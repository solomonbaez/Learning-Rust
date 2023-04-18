fn main() {
    let mut counter = 0;

    // define the outer loop
    while counter != 2 {
        println!("count: {counter}");
        
        let mut remaining = 10;
        
        // define the inner loop
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            remaining -= 1;
            
        }
        counter += 1;
    }
    println!("The count broke at: {counter}");
}
