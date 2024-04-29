//! This is the neo grammar module, which contains the grammar for the Neo language.
//! 
//! The grammar is defined using the `Token` enum, which represents the different types of tokens
#![allow(unused)]



use std::str::CharIndices;

use log::debug;

use crate::components::{
    operators::*,
    numbers::*,
};


pub trait Token: 
    std::fmt::Debug  // implement the Debug trait for the Token
    +
    'static  // * make the Token type static
    // making the Token type static ensures that the type is not tied to any specific lifetime
    // so that it can be used in any context
    // https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
{
    fn from_str<S: Into<String>>(string: S) -> Option<Self> where Self: Sized;
    
    fn to_string(&self) -> String {
        
        let mut result = std::any::type_name::<Self>().split("::").collect::<Vec<&str>>();
        result.reverse();

        let mut token_type = result.get(1).unwrap();
        let token_type = token_type.chars().next().unwrap().to_uppercase().collect::<String>() + &token_type[1..];


        // format!("{:>12} :: {self:?}", token_type)  // no additional formatting...

        // * add some good looking colors to the token type...
        // https://en.wikipedia.org/wiki/ANSI_escape_code
        let token_type = format!("\x1B[3m{}\x1B[0m", token_type);  // italic
        let token_type = format!("\x1B[1m{}\x1B[0m", token_type);  // bold
        
        format!("{:>28} :: {self:?}", token_type)


        // * Now with cyan color add cyan color
        // let token_type = format!("\x1B[36m{}\x1B[0m", token_type);  // cyan
        // let token_type = format!("\x1B[32m{}\x1B[0m", token_type);  // green
        // format!("{:>36} :: {self:?}", token_type)  (format with spacing according to the token_type length...)
        // todo: make this a bit more dynamic...
    }

}


#[derive(Debug, Clone)]
/// A lexer for tokenizing source code.
pub struct Lexer {
    src_code: String,
    pos: usize,
}

impl Lexer {
    /// Creates a new `Lexer` instance with the given source code.
    ///
    /// # Arguments
    ///
    /// * `src_code` - The source code to be tokenized.
    ///
    /// # Example
    ///
    /// ```
    /// let code = String::from("let x = 42;");
    /// let lexer = Lexer::new(code);
    /// ```
    pub fn new<S: Into<String>>(src_code: S) -> Self {
        let src_str: String = src_code.into();

        // * Remove all whitespace and control characters from the source code 
        let trimmed_str: String = src_str.chars()  // for all characters in the source code
        .filter(|c| !c.is_whitespace())  // * without whitespace
        // todo: test if this filter is absolutely necessary...
        .filter(|c| !c.is_ascii_control())  // * without control characters
        .collect();
    
        println!("New Lexer src: {}", 
            format!("\x1B[1m\x1B[3m{}\x1B[0m\n", src_str)
            // format!("\x1B[1m\x1B[3m{}\x1B[0m\n", trimmed_str))
        );

        Self {
            src_code: trimmed_str,
            pos: 0,  // Initialize the position to 0
        }
    }

    // Return the tokens until it reaches the EOF or any invalid token
    pub fn get_token_table(&self) -> Vec<Box<dyn Token>> {
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        self.clone().for_each(|token| tokens.push(token));
        tokens
    }

}


impl Iterator for Lexer {
    type Item = Box<dyn Token + 'static>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iterating from the current cursor position
        let mut chars = self.src_code[self.pos..].chars().enumerate().peekable();
        // the string of the current token
        let mut c_string: String = String::new();
        // the current token
        let mut c_token: Option<Box<dyn Token>> = None;

        while let Some((i, c)) = chars.next() {
            // self.pos = i;  // Increment the cursor position
            self.pos += 1;  // Increment the cursor position
            c_string.push(c);  // Add the current character to the token string

            // * Take the next character and check if it is a valid token
            c_token = Lexer::token_from(&c_string);
            if c == '+' || c == '-' {return c_token}

            if let Some((_, next_char)) = chars.peek() {
                let c_string = &format!("{}{}", c_string, *next_char);
                if let Some(token) = Lexer::token_from(c_string) {
                    c_token = Some(token);  // return the token
                    continue; // Consume the next character and continue the loop
                }
            }

            match c_token {
                Some(token) => return Some(token),
                None => debug!("Token not found for: {:?}", c_string),
            }

            // return c_token;  // return the token
        }
       c_token
    }

}



// ^ If some Token have a repeated &str identifier, it will only MATCH the FIRST ONE!
#[macro_export]
macro_rules! define_token_types {
    ( $( $token_type:ty ),+ $(,)? ) => {
        impl Lexer {
            pub fn token_from<S: Into<String>>(string: S) -> Option<Box<dyn Token>> {
                let s = string.into();
                $(if let Some(value) = <$token_type>::from_str(&s) {
                    return Some(Box::new(value));
                })+
                None
            }
        }
    };
}


// * Define the token types that will be included in the grammar
define_token_types!(
    // * Also the order defines the priority of the token types!
    // ^ SO, USE THE SIMPLER TOKEN TYPES FIRST!
    // LogicOp, 
    MathOp,
    // RelOp,

    Natural, 
    Integer, 
    Real, 

    // Just for logic:
    // Natural, LogicOp
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

