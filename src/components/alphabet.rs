//! The grammar defines the sequence of tokens that are valid for the language.
//!
//! This file contains the core components for tokenizing expressions, including
//! the definition of different token types and bracket types. It uses macros to
//! efficiently implement functionality across similar types.

use dev_utils::console::format::set_fg;
use crate::components::operators::*;

/// Represents the different types of tokens that can be recognized by the Lexer.
///
/// Each token type is associated with a specific character or set of characters
/// in the parsed text. These tokens are fundamental in the parsing and interpretation
/// of expressions in the language.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<T> where T: Operator {
    Variable(char),  // Any letter from A to Z (uppercase or lowercase)
    Number(char),  // Represent a sequence of digits in any base that count as one number
    UnderScore(),  // Used to add a "Subterm" to a term (e.g. A_1, A_2, A_3, ...)
    Dot(),  // Separate the integer part from the decimal part of a number
    Operator(T),  // Any type that implements the ['Operator'] trait
    OpenBracket(BracketType),  // Any type of bracket that opens another level of nesting
    CloseBracket(BracketType),  // Any type of bracket that closes a level of nesting
    Invalid(char),  //* Any other char that is not recognized by the 'Lexer'
}


// Macro implementation details...

/// The `impl_token` macro simplifies the implementation of common methods for
/// different token types.
///
/// This macro provides a generic way to implement methods like `from_char`,
/// `from_bracket`, and `to_char`, reducing the redundancy in the code.
macro_rules! impl_token {
    ($token_type:ident, $op_type:ty) => {
        impl $token_type<$op_type> {
            pub fn from(c: char) -> Self {
                // Moved logic from `from_char` here
                Self::from_bracket(c).unwrap_or_else(|| match c {
                    'A'..='Z' | 'a'..='z' => $token_type::Variable(c),
                    '0'..='9' => $token_type::Number(c),
                    '_' => $token_type::UnderScore(),
                    '.' => $token_type::Dot(),
                    _ => match <$op_type>::from(c) {
                        Some(op) => $token_type::Operator(op),
                        None => $token_type::Invalid(c),
                    },
                })
            }

            // Assuming `from_bracket` is part of the implementation
            fn from_bracket(c: char) -> Option<Self> {
                // Logic for brackets (remains the same as your original)
                match c {
                    '(' => Some($token_type::OpenBracket(BracketType::Parenthesis)),
                    ')' => Some($token_type::CloseBracket(BracketType::Parenthesis)),
                    '[' => Some($token_type::OpenBracket(BracketType::Square)),
                    ']' => Some($token_type::CloseBracket(BracketType::Square)),
                    '{' => Some($token_type::OpenBracket(BracketType::Curly)),
                    '}' => Some($token_type::CloseBracket(BracketType::Curly)),
                    _ => None,
                }
            }

            fn to_char(&self) -> char {
                match self {
                    $token_type::Variable(c) | $token_type::Number(c) | $token_type::Invalid(c) => *c,
                    $token_type::CloseBracket(bracket_type) => bracket_type.to_char().1,
                    $token_type::OpenBracket(bracket_type) => bracket_type.to_char().0,
                    $token_type::Operator(op) => op.to_char(),
                    $token_type::UnderScore() => '_',
                    $token_type::Dot() => '.',
                }
            }
        }
    };
}

impl_token!(Token, LogicOp);
impl_token!(Token, MathOp);



/// Represents the different types of brackets recognized by the Lexer.
///
/// This enum, along with the associated `bracket_type_and_to_char` macro,
/// provides a structured way to handle various bracket types, such as parentheses,
/// square brackets, and curly brackets.
macro_rules! bracket_type_and_to_char {
    ($($bracket_type:ident => ($open_char:expr, $close_char:expr)),*) => {
        #[derive(Clone, Debug, PartialEq, Copy)]
        pub enum BracketType {
            $($bracket_type),*,
        }

        impl BracketType {
            pub fn to_char(&self) -> (char, char) {
                match self {
                    $(BracketType::$bracket_type => ($open_char, $close_char),)*
                }
            }
        }
    };
}

bracket_type_and_to_char! {
    Parenthesis => ('(', ')'),
    Square => ('[', ']'),
    Curly => ('{', '}')
}
