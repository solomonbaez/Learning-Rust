use ::std::f32::consts::PI;

enum Shape {
    Circle { r: f32 },
    Rectangle { l: f32, w: f32, h: f32 },
}
impl Shape {
    fn area(&self) -> f32 {
        match &self {
            Shape::Circle { r } => {
                return PI * r.powf(2.0);
            }
            Shape::Rectangle { l, w, h } => return l * w * h,
        };
    }
}

fn main() {
    let circle = Shape::Circle { r: 10.0 };
    let circle_area = circle.area();
    println!("{}", circle_area);

    let rectangle = Shape::Rectangle {
        l: 1.0,
        w: 2.0,
        h: 2.0,
    };
    let rectangle_area = rectangle.area();
    println!("{}", rectangle_area);
}
