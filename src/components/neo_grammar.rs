//! This is the neo grammar module, which contains the grammar for the Neo language.
//! 
//! The grammar is defined using the `Token` enum, which represents the different types of tokens
#![allow(unused)]


use crate::components::{
    operators::*,
    numbers::*,
    alphabet::*,
};


// pub trait Token {  // * without the Debug trait (isn't really needed)
// * just added because it's easier to debug the Token's
pub trait Token: std::fmt::Debug {
    fn from_str<S: Into<String>>(string: S) -> Option<Self> where Self: Sized;
    
    fn t_type(&self) -> &'static str {
        match std::any::type_name::<Self>().split("::").last() {
            Some(t) => t,
            None => "Unknown",
        }
    }

}


#[derive(Debug, PartialEq)]
pub struct PrimitiveToken{}


// #[derive(Debug, PartialEq)]
// pub enum PrimitiveToken<Op, Num>
//     where 
//         Op: OperatorTrait, 
//         Num: NumberTrait 
// {
//     // ^ Testing
//     Variable(char),  // Any letter from A to Z (uppercase or lowercase)
//     // . Temporarily done (probably will be removed...)
//     Invalid(),  // Any char thas is not included in the alphabet or any PrimitiveToken
// }


// // Create an invalid token
// #[derive(Debug, PartialEq)]
// pub struct InvalidToken {
//     pub value: String
// }

// impl Token for InvalidToken {
//     fn from_str<S: Into<String>>(string: S) -> Option<Self> {
//         Some(InvalidToken { value: string.into() })
//     }
// }


// * This will include the Token types in the PrimitiveToken...
// * This means that it are the basic types that can be used in the language
// ^ If some Token have a repeated &str identifier, it will only MATCH the FIRST ONE!
#[macro_export]
macro_rules! define_token_types {
    ( $( $token_type:ty ),+ $(,)? ) => {
        impl PrimitiveToken {
            pub fn from<S: Into<String>>(string: S) -> Option<Box<dyn Token>> {
                let s = string.into();
                $(if let Some(value) = <$token_type>::from_str(&s) {
                    return Some(Box::new(value));
                })+
                None
            }
        }
    };
}

define_token_types!(
    Natural, 
    // Integer, 
    // Real, 
    LogicOp, 
    // MathOp, 
    RelOp
);


#[macro_export]
macro_rules! impl_token_type {
    ($token_type:ident; $trait_name:ident;
        $(
            $name:ident  // Name of the token type
        ),+ $(,)? // Multiple token types
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $token_type {}
        impl $token_type {
            pub fn from<S: Into<String> + Copy>(string: S) -> Option<Box<dyn $trait_name>> {
                $(if let Some(value) = $name::from_str(string) {
                    return Some(Box::new(value));
                })+
                None  // Return None if none of the types match
            }
        
        }
    };
}

