#![allow(unused)]

use std::str::CharIndices;
use std::iter::Peekable;
use log::debug;

use crate::{
    tracer::tokens::*,
};

pub trait TokenRecognizer {
    fn recognize_token<S: Into<String>>(input: S) -> Option<Box<dyn Token>>;
}

#[derive(Debug, Clone)]
/// A lexer for tokenizing source code.
pub struct Lexer<T: TokenRecognizer> {
    src_code: String,
    char_indices: Peekable<CharIndices<'static>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: TokenRecognizer> Lexer<T> {
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
    /// let code = String::from("25.1 * 42 - 13");
    /// let lexer = Lexer::new(code);
    /// 
    /// for token in lexer {
    ///     println!("{:?}", token);
    /// }
    /// ```
    pub fn new<S: Into<String>>(src_code: S) -> Self {
        let src_str: String = src_code.into();

        // Remove all whitespace and control characters from the source code 
        let trimmed_str: String = src_str.chars()
            .filter(|c| !c.is_whitespace())  // remove \t, \n, \r, \x20, etc.
            .filter(|c| !c.is_ascii_control())  // remove \x00 - \x1F, \x7F, etc.
            .collect();

        println!("\nNew {}:\t {}",
            std::any::type_name::<T>().split("::").last().unwrap(),
            format!("\x1B[1m\x1B[3m{}\x1B[0m\n", src_str)
            // format!("\x1B[1m\x1B[3m{}\x1B[0m\n", trimmed_str)
        );

        Self {
            src_code: trimmed_str.clone(),
            char_indices: Box::leak(trimmed_str.into_boxed_str()).char_indices().peekable(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Generates a table of tokens by tokenizing the entire source code.
    /// 
    /// This mehod resets the lexer and tokenizes the entire source code, returning a vector of boxed tokens
    /// # Returns
    ///
    /// Returns a vector of boxed tokens that represent the tokenized input.
    pub fn get_token_table(&mut self) -> Vec<Box<dyn Token>> {
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        // * restart the lexer (reset the char_indices iterator)
        self.char_indices = Box::leak(self.src_code.clone().into_boxed_str()).char_indices().peekable();
        while let Some(token) = self.next() {  // Tokenize the entire source code.
            tokens.push(token);  // Add the token to the table.
        }
        tokens  // Return the token table.
    }

}


impl<T: TokenRecognizer> Iterator for Lexer<T> {
    type Item = Box<dyn Token + 'static>;

    /// Advances the lexer to the next token, parsing and returning it.
    ///
    /// # Returns
    ///
    /// Returns Some(Box<dyn [`Token`]>) containing the next token if available, or `None` when no more tokens are available.
    fn next(&mut self) -> Option<Self::Item> {
        let mut c_string: String = String::new();
        let mut c_token: Option<Box<dyn Token>> = None;

        while let Some((_, c)) = self.char_indices.next() {
            c_string.push(c);

            // Attempt to match the current string as a token.
            c_token = T::recognize_token(&c_string);
            // If the current character is a single-character token, return it immediately.
            if c == '+' || c == '-' {return c_token}

            // Check the next character for multi-character tokens. (any: `dyn Token`)
            if let Some((_, next_char)) = self.char_indices.clone().peekable().peek().cloned() {
                let next_string = format!("{}{}", c_string, next_char);
                if let Some(token) = T::recognize_token(&next_string) {
                    c_token = Some(token);  // Return the multi-character token.
                    continue;  // Continue scanning for the next token.
                }
            }

            // Return the current token if a match is found or continue scanning.
            match c_token {
                Some(token) => return Some(token),
                None => debug!("Token not found for: {:?}", c_string),
            }
        }

        c_token
    }
}

/// Macro to implement a token recognizer for a given set of token types.
/// 
/// This macro generates a token recognizer struct that can be used to tokenize source code.
/// 
/// # Example
/// 
/// ```rust
/// impl_lexer_token_from!(MathLexer;  // recognizer for math tokens
///    MathOp,
///    Real,
///    // any other token types (dyn Token)...
/// );
#[macro_export]
macro_rules! impl_lexer_token_from {
    ($name:ident; $( $token_type:ty ),+ $(,)? ) => {
        pub struct $name;

        impl TokenRecognizer for $name {
            /// Attempts to convert a string to one of the specified token types.
            ///
            /// # Arguments
            ///
            /// * `string` - A string slice that represents the potential token.
            ///
            /// # Returns
            ///
            /// Returns `Some(Box<dyn Token>)` if the string matches a token type; otherwise, returns `None`.
            fn recognize_token<S: Into<String>>(string: S) -> Option<Box<dyn Token>> {
                let s = string.into();
                $(if let Some(value) = <$token_type>::from_str(&s) {
                    return Some(Box::new(value));
                })+
                None
            }
        }
    };
}

// Define different token recognizers
impl_lexer_token_from!(MathLexer;
    MathOp,
    Real,
);

impl_lexer_token_from!(LogicLexer;
    LogicOp,
    Natural,
);

impl_lexer_token_from!(PhysicLexer;
    PhysicConst,
    MathOp,
    Real,
);

impl_lexer_token_from!(CompleteLexer;
    MathOp,
    LogicOp,
    // RelOp,
    Natural,
    Integer,
    Real,
    GreekAlpha,
    GreekUpperAlpha,
    Alphabet,
    AlphaUpper,
    MathConst,
    PhysicConst,
);
