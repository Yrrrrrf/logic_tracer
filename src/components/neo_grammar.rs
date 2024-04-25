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
pub enum PrimitiveToken<Op, Num>
// pub enum PrimitiveToken<Op>
// pub enum PrimitiveToken<Num>
    where 
        Op: OperatorTrait, 
        Num: NumberTrait 
{
    // * Done 
    Number(Num),
    Operator(Op),
    
    // ^ Testing
    Variable(char),  // Any letter from A to Z (uppercase or lowercase)

    // . Temporarily done (probably will be removed...)
    Invalid(),  // Any char thas is not included in the alphabet or any PrimitiveToken
}


impl PrimitiveToken<LogicOp, Natural> {

    // * A primitive token is a token that is not a composite token
    pub fn from<S: Into<String>>(string: S) -> Self {
        let s: String = string.into();

        // * Check if the string is a number
        if let Some(number) = Natural::from_str(&s) {
            return PrimitiveToken::Number(number);
        }

        // * Check if the string is an operator
        if let Some(operator) = LogicOp::from_str(&s) {
            return PrimitiveToken::Operator(operator);
        }

        // * Check if the string is a variable
        if let Some(variable) = s.chars().next().filter(|c| c.is_alphabetic()){
            return PrimitiveToken::Variable(variable);
        }

        // println!("Invalid token: {:?}", s);
        Self::Invalid()
    }
}



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



