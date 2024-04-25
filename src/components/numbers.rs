// //! This file represents all the Number types that can be handled.
#![allow(unused)]


use super::neo_grammar::*;
use crate::*;


pub trait NumberTrait: Token {
    // create the into f64 method (according to std::convert::From)
}

const  DEFAULT_BASE: u8 = 36;  // * 2..=36
// * it cover the range from 0..=9 U A..=Z (total of 36 characters...)
// todo: expand this to allow for more bases (up to 255)...


#[derive(Debug, Clone, PartialEq)]
pub struct Digit(u8);

impl Digit {
    pub fn from_char(c: char, base: u8) -> Option<Self> {
        c.to_digit(base.into()).map(|digit| Digit(digit as u8))
    }
}


macro_rules! impl_number_token {
    ($token_type:ident; $trait_name:ident;
        $(
            $name:ident  // Name of the number type
            ($str2type:ident)  // Function to convert string to type
            { $($field:ident: $ty:ty),+ }  // Field and type
        ),+  // Repeat the pattern for each number type...
        $(,)?  // Optional comma at the end
    ) => {
        impl_token_type!($token_type; $trait_name; $($name),+);    
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name { $($field: $ty),+ }


            impl $trait_name for $name {
                // fn as_f64(&self) -> f64 {
                //     0.0
                // }
            }

            impl Token for $name {
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    $str2type(string)  // Call the str2type function
                }
            }
        )+  // TO HERE (Repeat the pattern for each number type...)

    };
}

impl_number_token!(Number; NumberTrait;
    Natural (str_to_n) { digits: Vec<Digit> },
    Integer (str_to_z) { sign: bool, value: Natural },
    Real    (str_to_r) { integer_part: Integer, fractional_part: Natural },
    // Imaginary { value: Real },
    // Complex { real_part: Real, imaginary_part: Imaginary },
    // Irrational { value: Real },
);


fn str_to_n<S: Into<String>>(string: S) -> Option<Natural> {
    let digits: Vec<Digit> = string.into().chars()
        .map(|c| Digit::from_char(c, DEFAULT_BASE))
        .collect::<Option<Vec<Digit>>>()?;
    (!digits.is_empty()).then(|| Natural { digits })
}

fn str_to_z<S: Into<String>>(string: S) -> Option<Integer> {
    let input = string.into();
    let (sign, value) = match input.chars().next() {
        Some('-') => (true, str_to_n(input.chars().skip(1).collect::<String>())?),
        _ => (false, str_to_n(input)?),
    };
    Some(Integer { sign, value })
}

fn str_to_r<S: Into<String>>(string: S) -> Option<Real> {
    let input = string.into(); // No need to explicitly declare type
    let (int_p, frac_p) = input.split_once('.')?;

    Some(Real {
        integer_part: str_to_z(int_p)?,
        fractional_part: str_to_n(frac_p)?
    })
}

// todo: str_2_i (r)  // mostly the same...
// todo: str_2_c (r + i)

