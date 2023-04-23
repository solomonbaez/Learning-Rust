
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