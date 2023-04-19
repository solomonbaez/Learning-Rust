

fn main() {
    // let mut list: [i32; 4] = [1, 2, 3, 4];

    // let result = largest(&list);

    let coordinate = Point { x:5.5, y:10 };
    let message = Point { x:"Hello", y:"World"};

    let mixup = coordinate.mixup(message);

    println!("mixup.x = {}, mixup.y = {}", mixup.x, mixup.y);
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}