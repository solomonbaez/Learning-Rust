
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y, 
        }
    }
}

// newtypes
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Psyker {
    fn fly(&self);
}

struct BaselineHuman;

impl Pilot for BaselineHuman {
    fn fly(&self) {
        println!("Approaching target.")
    }
}

impl Psyker for BaselineHuman {
    fn fly(&self) {
        println!("For the God Emperor.")
    }
}

impl BaselineHuman {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*". repeat(len+4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// newtype 
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0} + Point {x: 2, y: 3},
        Point { x: 3, y: 3}
    );

    let p = Point { x: 1, y: 4};

    p.outline_print();

    let w = Wrapper(
        vec![String::from("Hello"), String::from(" world!")]
    );

    println!("w = {}", w);

    let mm = Millimeters(10);
    let m = Meters(20);

    let res = mm.add(m);

    println!("mm after adding M: {:?}", res);

    let person = BaselineHuman;

    Pilot::fly(&person);
    Psyker::fly(&person);
}