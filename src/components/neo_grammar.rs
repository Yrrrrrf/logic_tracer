//! This is the neo grammar module, which contains the grammar for the Neo language.
//! 
//! The grammar is defined using the `Token` enum, which represents the different types of tokens
#![allow(unused)]


use crate::components::{
    operators::*,
    alphabet::*,
    // num::*,
    number::*,
};


pub trait Token {
    // fn from_char(char: char) -> Self;
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
    where 
        Op: OperatorTrait, 
        Num: NumberTrait 
{
    Variable(char),  // Any letter from A to Z (uppercase or lowercase)
    Number(Num),
    Operator(Op),

    // & Probably is not needed...?
    Invalid(char),  // Any char thas is not included in the alphabet or any PrimitiveToken
}




#[macro_export]
macro_rules! declare_enum {
    ( token $name:ident; $($variant:ident),+ ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {$($variant($variant),)+}
    };
    ( $name:ident; $($variant:ident),+ ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name { $($variant,)+ }
    };
}

#[macro_export]
macro_rules! declare_struct {
    ($name:ident; $($field:ident: $ty:ty),+) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {$($field: $ty),+}
    };
}

