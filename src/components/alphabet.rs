//! The grammar defines the sequence of tokens that are valid for the language.
//!
//! This file contains the core components for tokenizing expressions, including
//! the definition of different token types and bracket types. It uses macros to
//! efficiently implement functionality across similar types.

use crate::components::operators::*;

// Represents the different types of tokens that can be recognized by the Lexer.
///
/// Each token type is associated with a specific character or set of characters
/// in the parsed text. These tokens are fundamental in the parsing and interpretation
/// of expressions in the language.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<T> where T: Operator {
    Variable(char),  // Any letter from A to Z (uppercase or lowercase)
    Number(char),  // Represent a sequence of digits in any base that count as one number
    UnderScore(),  // Used to add a "SubCompound" to a Compound (e.g. A_1, A_2, A_3, ...)
    Dot(),  // Separate the integer part from the decimal part of a number
    Operator(T),  // Any type that implements the ['Operator'] trait
    OpenBracket(BracketType),  // Any type of bracket that opens another level of nesting
    CloseBracket(BracketType),  // Any type of bracket that closes a level of nesting
    Invalid(char),  //* Any other char that is not recognized by the 'Lexer'
}


/// The `impl_token` macro simplifies the implementation of common methods for
/// different token types.
///
/// This macro provides a generic way to implement methods like `from_char`,
/// `from_bracket`, and `to_char`, reducing the redundancy in the code.
macro_rules! impl_token {
    ($op_type:ty) => {
        impl Token<$op_type> {
            /// Creates a new token from a character.
            ///
            /// This method attempts to create a token corresponding to the given character.
            /// It first checks if the character represents a bracket type. If not,
            /// it then matches the character against various token types including
            /// variables, numbers, underscores, dots, and operators.
            ///
            /// # Arguments
            /// - `c` - The character to convert into a token.
            ///
            /// # Returns
            /// Returns a new token of the corresponding type, or `Token::Invalid` if
            /// the character does not match any token type.
            pub fn from(c: char) -> Self {
                BracketType::from_char(c).unwrap_or_else(|| 
                    match c {
                    'A'..='Z' | 'a'..='z' => Token::Variable(c),
                    '0'..='9' => Token::Number(c),
                    '_' => Token::UnderScore(),
                    '.' => Token::Dot(),
                    _ => match <$op_type>::from(c) {
                        Some(op) => Token::Operator(op),
                        None => Token::Invalid(c),
                    },
                }
            )}

            /// Converts the token to its character representation.
            ///
            /// This method provides a way to retrieve the character associated with a token,
            /// useful for debugging and display purposes.
            ///
            /// # Returns
            /// Returns the character representation of the token.
            fn to_char(&self) -> char {
                match self {
                    Token::Variable(c) | Token::Number(c) | Token::Invalid(c) => *c,
                    // ^ This two lines below fix the 'return tuple' issue on the 'BracketType' enum
                    // ^ Just to return the actual char, not the tuple
                    Token::OpenBracket(bracket_type) => bracket_type.to_char().0,
                    Token::CloseBracket(bracket_type) => bracket_type.to_char().1,
                    Token::Operator(op) => op.to_char(),
                    Token::UnderScore() => '_',
                    Token::Dot() => '.',
                }
            }
        }
    };
}

impl_token!(LogicOp);
impl_token!(MathOp);


/// Represents the different types of brackets recognized by the Lexer.
///
/// This enum, along with the associated `bracket_type_and_to_char` macro,
/// provides a structured way to handle various bracket types, such as parentheses,
/// square brackets, and curly brackets.
macro_rules! bracket_type_and_to_char {
    ($($bracket_type:ident => ($open_char:expr, $close_char:expr)),*) => {
        /// Represents types of brackets recognized in expressions.
        ///
        /// `BracketType` enum defines various kinds of brackets used in the language's syntax.
        /// This includes parentheses, square brackets, curly brackets, and potentially others.
        /// Each bracket type is associated with its opening and closing characters.
        ///
        /// The enum is primarily used in tokenization to identify different bracket types
        /// and their roles in structuring expressions. The opening and closing characters
        /// for each bracket type are essential for parsing nested structures and ensuring
        /// the correct interpretation of expressions.
        ///
        /// The enum variants are defined using the `bracket_type_and_to_char` macro, which
        /// also implements methods for creating a `BracketType` from a character and retrieving
        /// the character pair for a given `BracketType`.
        #[derive(Clone, Debug, PartialEq, Copy)]
        pub enum BracketType {
            $($bracket_type),*,
        }

        impl BracketType {
            /// Creates a `BracketType` from a character.
            ///
            /// This method checks if the given character corresponds to any known
            /// opening or closing bracket type.
            ///
            /// # Arguments
            /// - `c` - The character to check.
            ///
            /// # Returns
            /// Returns `Some(BracketType)` if the character matches a known bracket type,
            /// otherwise returns `None`.
            pub fn from_char<T: Operator>(c: char) -> Option<Token<T>> {
                match c {
                    $($open_char => Some(Token::OpenBracket(BracketType::$bracket_type)),)*
                    $($close_char => Some(Token::CloseBracket(BracketType::$bracket_type)),)*
                    _ => None,
                }
            }

            /// Retrieves the opening and closing characters for a bracket type.
            ///
            /// This method is useful for matching bracket pairs and for display purposes.
            ///
            /// # Returns
            /// Returns a tuple containing the opening and closing characters for the bracket type.
            pub fn to_char(&self) -> (char, char) {
                match self {
                    // todo: Return only the actual char, not the tuple
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
    // Other => ('/', '\\')
}
