use std::io;

fn main() {
    println!("Please input the measurement type you'd like to convert (type C to convert to F, and vice-versa): ");
    
    let mut measurement = String::new();

    io::stdin()
        .read_line(&mut measurement)
        .expect("Not a valid string.");

    println!("Please input the value you'd like to convert: ");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Not a valid number.");

    let temperature: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => unreachable!(),
    };

    let conversion = convert(measurement.trim(), temperature);

    println!("The value of {temperature} converted to {measurement} is: {conversion} {measurement}");
}

fn convert(m:&str, val:i32) -> i32 {
    let units: [&str; 2] = ["F", "C"];
    
    'conv_loop: loop {
        for idx in 0..2 {
            if units[idx] == m {
                let result = match m {
                    "F" => (val - 32) * 5 / 9,
                    "C" => val * 9 / 5 + 32,
                    _ => unreachable!(),
                };
                return result
            }
            else if idx == 1 {
                println!("Please enter a valid measurement system.");
                break 'conv_loop;
            }
        };
        
    }
    unreachable!()
}