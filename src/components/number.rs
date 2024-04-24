// //! This file represents all the Number types that can be handled.
#![allow(unused)]


use super::neo_grammar::*;
use crate::*;


pub trait NumberTrait: Token {
    // fn as_f64(&self) -> f64;

}


#[derive(Debug, Clone, PartialEq)]
pub struct Digit(u8);  // * Using a u8 allows a digit to be from base 2..=255

macro_rules! from_digit {
    ($($ty:ty),+) => {
        $(impl From<$ty> for Digit {
            fn from(value: $ty) -> Self {
                Digit(value as u8)
            }
        })+
        
    };
}

from_digit!(u8, char);


macro_rules! impl_number_token {
    ($token_type:ident;
        $(
            $name:ident  // Name of the number type
            ($str2type:ident)  // Function to convert string to type
            { $($field:ident: $ty:ty),+ }  // Field and type
        ),+  // Repeat the pattern for each number type...
        $(,)?  // Optional comma at the end
    ) => {
        declare_enum!(token $token_type; $($name),+);
    
        $(
            declare_struct!($name; $($field: $ty),+);

            impl NumberTrait for $name {
                // pub fn from
            }

            impl Token for $name {
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    let s = string.into();  // Convert the input to a string
                    assert!(s.len() > 0);  // Assert that the string is not empty
                    $str2type(s)  // Call the str2type function
                }
            }
        )+  // TO HERE (Repeat the pattern for each number type...)

        impl $token_type {
            pub fn from<S: Into<String> + Copy>(string: S) -> Option<Self> {
                $(if let Some(value) = $name::from_str(string) {
                        return Some($token_type::$name(value));
                })+
                None
            }
        }
    };
}

impl_number_token!(Number;
    Natural (str_to_n) { digits: Vec<Digit> },
    Integer (str_to_z) { sign: bool, value: Natural },
    Real    (str_to_r) { integer_part: Integer, fractional_part: Natural },
    // Imaginary { value: Real },
    // Complex { real_part: Real, imaginary_part: Imaginary },
    // Irrational { value: Real },
);


fn str_to_n<S: Into<String>>(string: S) -> Option<Natural> {
    let input = string.into();
    // * LIMITED TO BASE 10 (DECIMAL) to avoid using a base parameter... (for now)
    // * This can be extended to allow for different bases
    let digits: Vec<Digit> = input.chars().any(|c| !c.is_digit(10))
        .then(|| Vec::new()).unwrap_or_else(|| 
            input.chars().filter(|c| c.is_digit(10)).map(Digit::from).collect()
    );
    match digits.len() {
        0 => None,  // * If there are no digits, return None
        _ => Some(Natural { digits })
    }
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
    let input = string.into();
    let (int_p, frac_p) = input.split_once('.')?;

    let integer_part = str_to_z(int_p)?;
    let fractional_part = str_to_n(frac_p)?;

    Some(Real { integer_part,  fractional_part } )
}

// todo: str_2_i (r)  // mostly the same...
// todo: str_2_c (r + i)
