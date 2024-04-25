// //! This file represents all the Number types that can be handled.
#![allow(unused)]


use super::neo_grammar::*;
use crate::*;


pub trait NumberTrait: Token {
    // create the into f64 method (according to std::convert::From)
}

// todo: FIX THE DIGIT TYPE TO READ PROPERLY THE VALUE OF A U8
// * This because it currently reads the value as a char (which is not the intended behaviour)
// * I mean, probably is better to handle it using some king of "Alphabet" type (like the one in the alphabet.rs file)
// * Then the Digit can be from 0..=255 (u8) possible values allowing for a more flexible implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Digit(u8);  // * Using a u8 allows a digit to be from base 2..=255

macro_rules! from_digit {
    ($($ty:ty),+) => {
        $(impl From<$ty> for Digit {
            fn from(value: $ty) -> Self {Digit(value as u8)}
        })+
        
    };
}

from_digit!(u8, char);


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


// ? TESTING.....................................



// todo: Look for a way to implement the into f64 method for all number types
// * This enable the operations:
// * - `let num: f64 = number.into();`
// * - `let num: f64 = number as f64;`
// * This can be done by implementing the From trait for each number type

// impl From<Natural> for f64 {
//     fn from(value: Natural) -> Self {
//         let mut result = 0.0;
//         for (i, digit) in value.digits.iter().enumerate() {
//             result += (digit.0 as f64) * 10.0_f64.powi((value.digits.len() - i - 1) as i32);
//         }
//         println!("{:?}", result);
//         result
//     }
// }


// impl From<Integer> for f64 {
//     fn from(value: Integer) -> Self {
//         let mut result = 0.0;
//         for (i, digit) in value.value.digits.iter().enumerate() {
//             result += (digit.0 as f64) * 10.0_f64.powi((value.value.digits.len() - i - 1) as i32);
//         }
//         if value.sign {
//             -result
//         } else {
//             result
//         }
//     }
// }

// impl From<Real> for f64 {
//     fn from(value: Real) -> Self {
//         let mut result = 0.0;
//         for (i, digit) in value.integer_part.value.digits.iter().enumerate() {
//             result += (digit.0 as f64) * 10.0_f64.powi((value.integer_part.value.digits.len() - i - 1) as i32);
//         }
//         let mut frac_result = 0.0;
//         for (i, digit) in value.fractional_part.digits.iter().enumerate() {
//             frac_result += (digit.0 as f64) * 10.0_f64.powi(-(i as i32 + 1));
//         }
//         result + frac_result
//     }
// }



fn str_to_n<S: Into<String>>(string: S) -> Option<Natural> {
    let input = string.into();
    // * LIMITED TO BASE 10 (DECIMAL) to avoid using a base parameter... (for now)
    // * This can be extended to allow for different bases
    let digits: Vec<Digit> = input.chars().any(|c| !c.is_digit(10))
        .then(|| Vec::new()).unwrap_or_else(|| 
            input.chars().filter(|c| c.is_digit(10)).map(|c| Digit::from(c)).collect()
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
