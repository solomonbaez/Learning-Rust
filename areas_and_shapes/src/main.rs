use ::std::f32::consts::PI;

enum Shape {
    Circle { r: f32 },
}
impl Shape {
    fn area(&self) -> f32 {
        match &self {
            Shape::Circle { r } => {
                return PI * r.powf(2.0);
            }
        };
    }
}

fn main() {
    let circle = Shape::Circle { r: 10.0 };
    let circle_area = circle.area();
    println!("{}", circle_area);
}
