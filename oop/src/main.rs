
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("This should construct a button!")
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub signature: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("This should be a select-box!")
    }
}

pub struct Screen {
    // hold trait object
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 10,
                signature: vec![
                    String::from("This"),
                    String::from("really"),
                    String::from("should"),
                    String::from("be"),
                    String::from("functional"),
                ],
            }),
            Box::new(Button {
                width: 2, 
                height: 2,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();
}