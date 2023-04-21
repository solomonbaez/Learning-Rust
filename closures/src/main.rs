
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        // closure will call most_stocked to give away the largest inv
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut count_red = 0;
        let mut count_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => count_red += 1,
                ShirtColor::Blue => count_blue +=1,
            }
        }

        if count_red > count_blue {
            ShirtColor::Red
        }   
        else {
            ShirtColor::Blue
        }
    }
}

use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preference {:?} gets {:?}",
        user_pref1, giveaway1);

    // preference 2 is none, will activate the closure in giveaway
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preference {:?} gets {:?}",
        user_pref2, giveaway2);

    // extra practice
    let mut list: Vec<i32> = vec![1, 2, 3];

    let list_iter = list.iter();

    for val in list_iter {
        println!("Got: {}", val);
    }

    println!("Before closure, after map: {:?}", list);

    let mut borrows_mutably = || list.push(8);
    borrows_mutably();
    
    println!("After closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}