use std::ops::Add;

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// こうすると、impl Iterator<String> for Counterとか複数の型に対する実装ができちゃう
pub trait Iteratorr<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millmeters(u32);
struct Meters(u32);

impl Add<Meters> for Millmeters {
    type Output = Millmeters;

    fn add(self, other: Meters) -> Millmeters {
        Millmeters(self.0 + (other.0 * 1000))
    }
}
