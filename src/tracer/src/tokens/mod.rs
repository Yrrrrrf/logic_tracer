//! This module defines the types of tokens that can be recognized and validated by the
//! [`Lexer`](crate::components::lexer) and subsequently utilized by the [`Parser`](crate::components::parser).
//!
//! It provides a foundational framework for building the lexical components of a language,
//! including primitive and potentially complex token types.
//!
//! ## Usage
//!
//! Token types are typically constructed via parsing strings and are extensively used throughout the
//! lexer and parser components to facilitate language processing.

// Consider re-enabling and documenting the `variables` module if relevant for future extensions.
// pub mod variables;  // Defines variable-related tokens like identifiers.

pub mod numbers; // Contains definitions for numeric types.
pub mod operators; // Contains definitions for various operators.
pub mod variables; // Contains definitions for variable-related tokens.

pub use numbers::*;
pub use operators::*;
pub use variables::*; // variables generator (constants, alphabets, hiragana, etc...)

use std::fmt::Debug;

/// Represents a generic token within the language processing system.
///
/// All token types must implement this trait to ensure they can be debugged and
/// dynamically handled via polymorphism. Tokens are generally created from strings and
/// can be represented back as strings for debugging and logging purposes.
// * IN COMPUTER SCIENCE.
// * A token is a string of one or more characters, which are treated as a single unit by a program.
// * Tokens are the smallest elements of a program, and they are classified by the compiler according to their functionality.
pub trait Token: Debug {
    /// Constructs an instance of a token from a string, if possible.
    fn from_str<S: Into<String>>(string: S) -> Option<Self>
    where
        Self: Sized;

    /// Returns a string representation of the token, typically used for debugging.
    fn to_string(&self) -> String {
        let mut result = std::any::type_name::<Self>()
            .split("::")
            .collect::<Vec<&str>>();
        result.reverse();

        let mut token_type = result.get(0).unwrap();
        let token_type = token_type
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>()
            + &token_type[1..];

        // format!("{:>12} :: {self:?}", token_type)  // no additional formatting...
        let token_type = format!("\x1B[3m{}\x1B[0m", token_type); // italic
        let token_type = format!("\x1B[1m{}\x1B[0m", token_type); // bold

        format!("{token_type:>28} :: {self:?}")
        // let token_type = format!("\x1B[36m{}\x1B[0m", token_type);  // cyan
        // let token_type = format!("\x1B[32m{}\x1B[0m", token_type);  // green
        // format!("{:>36} :: {self:?}", token_type)  // (format with spacing according to the token_type length...)
        // todo: make this a bit more dynamic...
    }
}

#[macro_export]
/// Macro to implement specific token types for a given trait.
///
/// This macro simplifies the creation of token structures that implement a common trait,
/// allowing for polymorphic handling of different token types within the parsing logic.
macro_rules! impl_enum_token {
    ($token_type:ident; $trait_name:ident;  //
        $(
            $name:ident (
                $(
                    $variant:ident => ($($str:expr),+)
                    $(,)?
                )+
            )
        ),+ $(,)?
    ) => {
        crate::impl_token_trait!($token_type; $trait_name; $($name),+);
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub enum $name { $($variant,)+ }

            impl Token for $name {
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    match string.into().as_str() {
                        $($($str)|+ => Some($name::$variant),)+
                        _ => None
                    }
                }
            }

            impl $trait_name for $name {}  //
        )+
    };
}

#[macro_export]
/// Macro to implement specific token types for a given trait.
/// Implements the 'from' method for the token type to create a Box<dyn Trait> from a string.
///
/// This macro requires that al token types implement the specified trait and provides a `from_str` method to parse a string into a token.
macro_rules! impl_token_trait {
    ($token_type:ident; $trait_name:ident;
        $(
            $name:ident  // Name of the token type
        ),+ $(,)? // Supports multiple token types
    ) => {
        impl $token_type {
            /// Attempts to create a boxed token from a string if it matches any of the specified types.
            pub fn from<S: Into<String> + Copy>(string: S) -> Option<Box<dyn $trait_name>> {
                $(if let Some(value) = $name::from_str(string) {
                    Some(Box::new(value));  // Create some box with the token type
                })+
                None  // Return None if no types match
            }
        }
    };
}
