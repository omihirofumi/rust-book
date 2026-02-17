use rust_book::{Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox ({} x {})", self.width, self.height);
        println!("Options:");
        for option in &self.options {
            println!("- {}", option);
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    on_click: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button ({} x {})", self.width, self.height);
        println!("{} fired", self.on_click);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 100,
                options: vec![String::from("Yes"), String::from("Wow")],
            }),
            Box::new(Button {
                width: 10,
                height: 10,
                on_click: String::from("submit"),
            }),
        ],
    };

    screen.run();
}
