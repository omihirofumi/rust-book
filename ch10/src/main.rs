use std::fmt::Display;

// orphan rule
// -> trait or typeの作者のどっちかがtrait実装に責任を持つ。
// -> つまり、二つとも外部だとできない。
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = largest_i32(&number_list);

    println!("The largest number is {largest}");

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you propably already know, people"),
        reply: false,
        repost: false,
    };

    println!("{}", post.summarize());
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
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

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        String::from("Hello")
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        String::from("Hello")
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// fn notify(item1: &impl Summary, item2: &impl Summary) {}
// fn notify<T: Summary>(item1: &T, item2: &T) {}
// fn notify(item: &(impl Summary + Display)) {}
// fn notify<T: Summary + Display>(item: &T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// fn returns_summarizable() -> impl Summary {}
