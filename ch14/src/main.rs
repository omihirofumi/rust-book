use ch14::{PrimaryColor, add_one, mix};

fn main() {
    println!("Hello, world!");
    add_one(4);

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let _ = mix(red, yellow);
}
