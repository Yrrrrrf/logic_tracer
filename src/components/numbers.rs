// //! This file represents all the Number types that can be handled.
#![allow(unused)]


use super::lexer::*;
use crate::*;


pub trait NumberTrait: Token {
    // create the into f64 method (according to std::convert::From)
}

const  DEFAULT_BASE: u8 = 10;  // * 2..=36
// * it cover the range from 0..=9 U A..=Z (total of 36 characters...)
// todo: expand this to allow for more bases (up to 255)...


#[derive(Debug, Clone, PartialEq)]
pub struct Digit(u8);

impl Digit {
    pub fn from_char(c: char, base: u8) -> Option<Self> {
        c.to_digit(base.into()).map(|digit| Digit(digit as u8))
    }
}


macro_rules! define_numeric_type {
    ($token_type:ident; $trait_name:ident;
        $(
            $name:ident($native_type:ty)
        ),+
        $(,)?
    ) => {
        impl_token_type!($token_type; $trait_name; $($name),+);
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name { value: $native_type }
        
            impl $trait_name for $name {
            }

            impl Token for $name {
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    string.into().parse::<$native_type>().ok().map(|value| Self { value })
                // // &str2type:ident   // Function to convert string to type
                //     $str2type(string)  // Call the str2type function    
                }
            }
        )+
    };
}

// Use the macro to define Natural, Integer, and Real
define_numeric_type!(Number; NumberTrait;
    Natural(usize),
    Integer(isize),
    Real(f64)
);


// todo: str_2_i (r)  // mostly the same...
// todo: str_2_c (r + i)

