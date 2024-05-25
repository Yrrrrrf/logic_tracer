//! This is the neo grammar module, which contains the grammar for the Neo language.
//! 
//! The grammar is defined using the `Token` enum, which represents the different types of tokens
#![allow(unused)]

use std::str::CharIndices;
use std::iter::Peekable;


use log::debug;

use crate::{components::tokens::{
    Token,
    numbers::*,
    operators::*,
    variables::*,
    }
};


#[derive(Debug, Clone)]
/// A lexer for tokenizing source code.
pub struct Lexer {
    src_code: String,
    char_indices: CharIndices<'static>,
}

// todo: Modify the lexer to now recibe the tokens to be used in the grammar!
// * To allow the user to define the tokens to be used in the grammar!
// * Fow now I'm using some dynamic dispatch (Box<dyn Token>) to allow the use of multiple token types
// * I need to use some static dispatch to allow the user to select the tokens to be used in the grammar!
// * The differenc is that the user will have to define the tokens to be used in the grammar, and the lexer will use them to tokenize the source code!


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
    /// use logic_tracer::Lexer;
    /// use logic_tracer::tokens::Token;
    /// use logic_tracer::tokens::numbers::*;
    /// use logic_tracer::tokens::operators::*;
    /// 
    /// 
    /// let code = String::from("25.1 * 42 - 13");
    /// let lexer = Lexer::new(code);
    /// 
    // todo: add some comparion with the lexer...
    // todo: like iterate over the tokens and compare them with the expected tokens...
    ///
    /// for token in lexer {
    ///    println!("{:?}", token);
    /// // print the type of the token
    /// }
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

        let char_indices = Box::leak(trimmed_str.clone().into_boxed_str()).char_indices();

        Self {
            src_code: trimmed_str,
            char_indices,
        }
    }

    /// Generates a table of tokens by tokenizing the entire source code.
    ///
    /// # Returns
    ///
    /// Returns a vector of boxed tokens that represent the tokenized input.
    pub fn get_token_table(&self) -> Vec<Box<dyn Token>> {
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        self.clone().for_each(|token| tokens.push(token));
        tokens
    }

    // Explanation:
    // * .clone()    Clone the char_indices iterator.
    // * .peekable() Make the iterator peekable. (This consumes the iterator)
    // * .peek()     Peek at the next element without consuming it.
    // * .cloned()   Clone the peeked element. (This consumes the peekable iterator)
    //  ^ This fn is redundant because it's only used once, but it's useful to understand the process
    #[inline]  // replace the call with the actual code
    fn peek_char(&self) -> Option<(usize, char)> {
        self.char_indices.clone().peekable().peek().cloned()
    }


}

impl Iterator for Lexer {
    type Item = Box<dyn Token + 'static>;

    /// Advances the lexer to the next token, parsing and returning it.
    ///
    /// # Returns
    ///
    /// Returns Some(Box<dyn [`Token`]>) containing the next token if available, or `None` when no more tokens are available.
    fn next(&mut self) -> Option<Self::Item> {
        // // Start iterating from the current cursor position
        // let mut chars = self.src_code[self.char_pos..].chars().enumerate().peekable();
        // // the string of the current token
        // let mut c_string: String = String::new();
        // // the current token
        // let mut c_token: Option<Box<dyn Token>> = None;

        let mut c_string = String::new();
        let mut c_token = None;




        while let Some((i, c)) = self.char_indices.next() {  // Iterate over the characters in the source code.
            c_string.push(c);  // Accumulate characters into the current token string.

            // Attempt to match the current string as a token.
            c_token = Lexer::token_from(&c_string);
            if c == '+' || c == '-' {return c_token}

            // * Check the next character for multi-character tokens.
            // if let Some((_, next_char)) = chars.peek() {  // If there is a next character.
            if let Some((_, next_char)) = self.peek_char() {  // If there is a next character.
                let next_string = &format!("{}{}", c_string, next_char);  // Append the next character to the current string.
                if let Some(token) = Lexer::token_from(next_string) {  // * If the new string matches a token.
                    c_token = Some(token);  // * Update the current token (a token with more characters was found).
                    continue;  // * Continue to next iteration without advancing the main cursor.
                }
            }

            // * Return the current token if a match is found or continue scanning.
            match c_token {
                Some(token) => return Some(token),
                None => debug!("Token not found for: {:?}", c_string),
            }
        }
        c_token  // Return the last token found or None if no tokens match.
    }

}


// ^ If some Token have a repeated &str identifier, it will only MATCH the FIRST ONE!
/// Macro to implement the `token_from` function for various token types.
/// This allows the lexer to recognize and construct tokens from strings.
#[macro_export]
macro_rules! impl_lexer_token_from {
    ( $( $token_type:ty ),+ $(,)? ) => {
        impl Lexer {
            /// Attempts to convert a string to one of the specified token types.
            ///
            /// # Arguments
            ///
            /// * `string` - A string slice that represents the potential token.
            ///
            /// # Returns
            ///
            /// Returns `Some(Box<dyn Token>)` if the string matches a token type; otherwise, returns `None`.
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
impl_lexer_token_from!(
    // * Also the order defines the priority of the token types!
    // ^ SO, USE THE SIMPLER TOKEN TYPES FIRST!
    // LogicOp, 
    MathOp,
    // RelOp,

    // Natural, 
    Integer, 
    Real, 

    GreekAlpha,
    GreekUpperAlpha,

    PhysicConst,

    // Just for logic:
    // Natural, LogicOp
);
