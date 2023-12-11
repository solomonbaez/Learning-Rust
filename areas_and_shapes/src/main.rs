use ::std::f32::consts::PI;

enum Shape {
    Circle { r: f32 },
    Triangle { h: f32, b: f32 },
    Rectangle { l: f32, w: f32, h: f32 },
}
impl Shape {
    fn area(&self) -> f32 {
        match &self {
            // deconstruct the variant in the match statement
            Shape::Circle { r } => {
                return PI * r.powf(2.0);
            }
            Shape::Triangle { h, b } => return 0.5 * h * b,
            Shape::Rectangle { l, w, h } => return l * w * h,
        };
    }
}

fn main() {
    let circle = Shape::Circle { r: 10.0 };
    println!("{}", circle.area());

    let triangle = Shape::Triangle { h: 10.0, b: 30.0 };
    println!("{}", triangle.area());

    let rectangle = Shape::Rectangle {
        l: 1.0,
        w: 2.0,
        h: 2.0,
    };
    println!("{}", rectangle.area());
}
