//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certail
//! calcuations more conveninet.

/// Adds one to the numerb given
// --snip--
/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = ch14::add_one(arg);;
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 3
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let _ = c1;
        let _ = c2;
        SecondaryColor::Orange
    }
}
