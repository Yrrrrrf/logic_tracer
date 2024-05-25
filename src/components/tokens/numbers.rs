// //! This file represents all the Number types that can be handled.
#![allow(unused)]

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Number;


pub trait NumberTrait: Token {
    // Create the into f64 method (according to std::convert::From)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Digit(u8);

impl Digit {
    pub fn from_char(c: char, base: u8) -> Option<Self> {
        c.to_digit(base.into()).map(|digit| Digit(digit as u8))
    }
}

/// Macro to define numeric types and their associated traits.
///
/// # Parameters
///
/// - `$token_type`: The type of the token (e.g., `Number`).
/// - `$trait_name`: The name of the trait that all numeric tokens will implement.
/// - `$name`: The name of the struct representing a specific numeric type (e.g., `Natural`, `Integer`, `Real`).
/// - `$native_type`: The native Rust type that the struct wraps (e.g., `usize`, `isize`, `f64`).
///
/// # Example
///
/// ```rust
/// define_numeric_type!(Number; NumberTrait;
///     Natural(usize),
///     Integer(isize),
///     Real(f64),
/// );
/// ```
macro_rules! define_numeric_type {
    ($token_type:ident; $trait_name:ident;
        $(
            $name:ident($native_type:ty)
        ),+
        $(,)?
    ) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name { value: $native_type }

            impl $name {
                /// Creates a new instance of the numeric type from the native Rust type.
                /// # Parameters
                /// - `value`: The native Rust type value to be wrapped.
                pub fn from_n(value: $native_type) -> Self {
                    Self { value }
                }
            }

            impl $trait_name for $name {}

            impl Token for $name {
                /// Parses a string into an instance of the numeric type.
                /// # Parameters
                /// - `string`: The string to be parsed.
                /// # Returns
                /// - `Option<Self>`: An instance of the numeric type if the parsing succeeds, `None` otherwise.
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    string.into().parse::<$native_type>().ok().map(|value| Self { value })
                }
            }
        )+
        // Once all the numeric types have been defined, implement the trait for the token type...
        crate::impl_token_trait!($token_type; $trait_name; $($name),+);
    };
}

// Use the macro to define Natural, Integer, and Real
define_numeric_type!(Number; NumberTrait;
    Natural(usize),
    Integer(isize),
    Real(f64),
);


// todo: Improve define_numeric_type! macro to handle Imaginary and Complex numbers
// todo: Also modify the code above to make it able to use the Digit type to parse the numbers!
// todo: Using the Digit type will allow the input of numbers in ANY BASE!
// #[derive(Debug, Clone, PartialEq)]
// pub struct Imaginary {
//     value: f64,
// }

// impl Imaginary {
//     pub fn from_n(value: f64) -> Self { Self { value } }
// }

// impl NumberTrait for Imaginary {}

// impl Token for Imaginary {
//     fn from_str<S: Into<String>>(string: S) -> Option<Self> {
//         let string = string.into();
//         if string.ends_with('i') {
//             let real_part = &string[..string.len() - 1];
//             real_part.parse::<f64>().ok().map(|value| Self { value })
//         } else {
//             None
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Complex {
//     real: f64,
//     imag: f64,
// }

// impl Complex {
//     pub fn from_parts(real: f64, imag: f64) -> Self { Self { real, imag } }
// }

// impl NumberTrait for Complex {}

// impl Token for Complex {
//     fn from_str<S: Into<String>>(string: S) -> Option<Self> {
//         let string = string.into();
//         let parts: Vec<&str> = string.split(|c| c == '+' || c == '-').collect();
//         if parts.len() == 2 {
//             let real_part = parts[0];
//             let imag_part = &parts[1][..parts[1].len() - 1]; // Remove the trailing 'i'

//             if let (Ok(real), Ok(imag)) = (real_part.parse::<f64>(), imag_part.parse::<f64>()) {
//                 let sign = if string.contains('-') { -1.0 } else { 1.0 };
//                 return Some(Self { real, imag: sign * imag });
//             }
//         }
//         None
//     }
// }

// todo: Also 