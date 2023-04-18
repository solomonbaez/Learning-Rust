// allow for print debugging
#[derive(Debug)]
// rectangle structure
struct Rectangle {
    width: u32,
    height: u32,
}
// rectangle methods
impl Rectangle {
    // check if object is a rectangle
    // width getter
    fn width(&self) -> bool {
        self.width > 0
    }

    // provide ownership to the impl
    // calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }    

    // check if rectangles are compatable
    fn can_hold(&self, fill: &Rectangle) -> bool {
        self.width > fill.width && self.height > fill.height
    }
}

fn main() {
    // define rectangles
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle2 = Rectangle {
        width: 10,
        height: 50,
    };
    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };

    if rectangle1.width() {
        // report dims
        println!("rectangle 1 is {:#?}", rectangle1);
        // report area
        println!(
            "The area of the rectangle is {} square pixels.",
            rectangle1.area()
        );
    }
    else {
        println!("The object is not a rectangle.")
    }

    println!("Can rectangle 1 hold rectangle 2? {}", rectangle1.can_hold(&rectangle2));
    println!("Can rectangle 1 hold rectangle 3? {}", rectangle1.can_hold(&rectangle3));
}
