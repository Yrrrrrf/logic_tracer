These are the main file for the tokens mod:
```rust (mod.rs)
//! This module defines the types of tokens that can be recognized and validated by the
//! [`Lexer`](crate::components::lexer) and subsequently utilized by the [`Parser`](crate::components::parser).
//! 
//! It provides a foundational framework for building the lexical components of a language,
//! including primitive and potentially complex token types.
//! 
//! ## Modules
//! 
//! - `operators`: Includes various operator types like logical, mathematical, and relational operators.
//! - `numbers`: Defines numeric token types such as natural numbers, integers, and real numbers.
//!
//! ## Usage
//!
//! Token types are typically constructed via parsing strings and are extensively used throughout the
//! lexer and parser components to facilitate language processing.

// Consider re-enabling and documenting the `variables` module if relevant for future extensions.
// pub mod variables;  // Defines variable-related tokens like identifiers.

pub mod operators;  // Contains definitions for various operators.
pub mod numbers;    // Contains definitions for numeric types.
pub mod variables;  // Contains definitions for variable-related tokens.

pub use operators::Operator;
pub use numbers::Number;
pub use variables::Variable;  // variables generator

// pub use variables::Variable;
// pub use variables::Constant;


/// Represents a generic token within the language processing system.
/// 
/// All token types must implement this trait to ensure they can be debugged and
/// dynamically handled via polymorphism. Tokens are generally created from strings and
/// can be represented back as strings for debugging and logging purposes.
pub trait Token: std::fmt::Debug + 'static {
    /// Constructs an instance of a token from a string, if possible.
    fn from_str<S: Into<String>>(string: S) -> Option<Self> where Self: Sized;
    
    /// Returns a string representation of the token, typically used for debugging.
    fn to_string(&self) -> String {
        let mut result = std::any::type_name::<Self>().split("::").collect::<Vec<&str>>();
        result.reverse();

        let mut token_type = result.get(1).unwrap();
        let token_type = token_type.chars().next().unwrap().to_uppercase().collect::<String>() + &token_type[1..];

        // format!("{:>12} :: {self:?}", token_type)  // no additional formatting...
        let token_type = format!("\x1B[3m{}\x1B[0m", token_type);  // italic
        let token_type = format!("\x1B[1m{}\x1B[0m", token_type);  // bold
        
        format!("{:>28} :: {self:?}", token_type)
        // let token_type = format!("\x1B[36m{}\x1B[0m", token_type);  // cyan
        // let token_type = format!("\x1B[32m{}\x1B[0m", token_type);  // green
        // format!("{:>36} :: {self:?}", token_type)  (format with spacing according to the token_type length...)
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
```
```rust
// //! This file represents all the Number types that can be handled.
#![allow(unused)]

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Number;


pub trait NumberTrait: Token {
    // Create the into f64 method (according to std::convert::From)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Digit(u8);

impl Digit {
    pub fn from_char(c: char, base: u8) -> Option<Self> {
        c.to_digit(base.into()).map(|digit| Digit(digit as u8))
    }
}

/// Macro to define numeric types and their associated traits.
///
/// # Parameters
///
/// - `$token_type`: The type of the token (e.g., `Number`).
/// - `$trait_name`: The name of the trait that all numeric tokens will implement.
/// - `$name`: The name of the struct representing a specific numeric type (e.g., `Natural`, `Integer`, `Real`).
/// - `$native_type`: The native Rust type that the struct wraps (e.g., `usize`, `isize`, `f64`).
///
/// # Example
///
/// ```rust
/// define_numeric_type!(Number; NumberTrait;
///     Natural(usize),
///     Integer(isize),
///     Real(f64),
/// );
/// ```
macro_rules! define_numeric_type {
    ($token_type:ident; $trait_name:ident;
        $(
            $name:ident($native_type:ty)
        ),+
        $(,)?
    ) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name { value: $native_type }

            impl $name {
                /// Creates a new instance of the numeric type from the native Rust type.
                /// # Parameters
                /// - `value`: The native Rust type value to be wrapped.
                pub fn from_n(value: $native_type) -> Self {
                    Self { value }
                }
            }

            impl $trait_name for $name {}

            impl Token for $name {
                /// Parses a string into an instance of the numeric type.
                /// # Parameters
                /// - `string`: The string to be parsed.
                /// # Returns
                /// - `Option<Self>`: An instance of the numeric type if the parsing succeeds, `None` otherwise.
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    string.into().parse::<$native_type>().ok().map(|value| Self { value })
                }
            }
        )+
        // Once all the numeric types have been defined, implement the trait for the token type...
        crate::impl_token_trait!($token_type; $trait_name; $($name),+);
    };
}

// Use the macro to define Natural, Integer, and Real
define_numeric_type!(Number; NumberTrait;
    Natural(usize),
    Integer(isize),
    Real(f64),
);


// todo: Improve define_numeric_type! macro to handle Imaginary and Complex numbers
// todo: Also modify the code above to make it able to use the Digit type to parse the numbers!
// todo: Using the Digit type will allow the input of numbers in ANY BASE!
// #[derive(Debug, Clone, PartialEq)]
// pub struct Imaginary {
//     value: f64,
// }

// impl Imaginary {
//     pub fn from_n(value: f64) -> Self { Self { value } }
// }

// impl NumberTrait for Imaginary {}

// impl Token for Imaginary {
//     fn from_str<S: Into<String>>(string: S) -> Option<Self> {
//         let string = string.into();
//         if string.ends_with('i') {
//             let real_part = &string[..string.len() - 1];
//             real_part.parse::<f64>().ok().map(|value| Self { value })
//         } else {
//             None
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Complex {
//     real: f64,
//     imag: f64,
// }

// impl Complex {
//     pub fn from_parts(real: f64, imag: f64) -> Self { Self { real, imag } }
// }

// impl NumberTrait for Complex {}

// impl Token for Complex {
//     fn from_str<S: Into<String>>(string: S) -> Option<Self> {
//         let string = string.into();
//         let parts: Vec<&str> = string.split(|c| c == '+' || c == '-').collect();
//         if parts.len() == 2 {
//             let real_part = parts[0];
//             let imag_part = &parts[1][..parts[1].len() - 1]; // Remove the trailing 'i'

//             if let (Ok(real), Ok(imag)) = (real_part.parse::<f64>(), imag_part.parse::<f64>()) {
//                 let sign = if string.contains('-') { -1.0 } else { 1.0 };
//                 return Some(Self { real, imag: sign * imag });
//             }
//         }
//         None
//     }
// }
```
```rust
use super::*;

use crate::impl_enum_token;
pub trait OperatorTrait: Token {
    // * Add properties and methods for the operator type...
    // todo: Add all the stuff below...
        // todo: Add `precedence`, `associativity`, and `arity` properties for the operator type...
        // todo: Add `is_unary`, `is_binary`, and `is_nary` methods for the operator type... (use `arity`)
        // todo: Add `is_left_associative`, `is_right_associative`, and `is_non_associative` methods for the operator type... (use `associativity`)
}
#[derive(Debug, Clone, PartialEq)]
pub struct Operator;

pub trait OperatorNegator: OperatorTrait {
    const NEGATOR: Self;
}

/// Macro to implement operator tokens and their associated traits.
/// 
/// # Parameters
/// 
/// - `$token_type`: The type of the token (e.g., `Operator`).
/// - `$trait_name`: The name of the trait that all operator tokens will implement.
/// - `$name`: The name of the enum representing a specific operator type (e.g., `MathOp`).
/// - `$negator`: An optional parameter representing the negator variant for the operator type.
/// - `$variant`: The variants of the operator enum, mapping strings to enum variants.
/// 
/// # Example
/// 
/// ```rust
/// impl_operator_token!(Operator; OperatorTrait;
///     MathOp (Subtract;  // this will be the negator of the operator type
///         Add => ("+"),
///         Subtract => ("-"),
///         Multiply => ("*"),
///         Divide => ("/"),
///         Modulo => ("%"),
///         Power => ("^"),
///         Root => ("√"),
///         Factorial => ("!"),
///     ),
///     LogicOp (Not;  // this will be the negator of the operator type
///         And => ("&", "^", "∧", "+"),
///         Or => ("|", "||", "*"),
///         Not => ("!", "~", "¬"),
///         XOr => ("^", "⊻", "⨁"),
///         XNOr => ("⊙", "⊽"),
///         NAnd => ("↑"),
///         NOr => ("↓"),
///     ),
///     RelationalOp (NotEqual;
///         Equal => ("==", "="),
///         NotEqual => ("!=", "≠"),
///         LessThan => ("<"),
///         LessThanOrEqual => ("<=", "≤"),
///         GreaterThan => (">"),
///         GreaterThanOrEqual => (">=", "≥"),
///     ),
///     SomeOtherOp (;  // this will not have a negator (don't implement OperatorNegator for it)
///         SomeOp => ("some_op"),
///         AnotherOp => ("another_op"),
///    ),
/// );
/// ```
macro_rules! impl_operator_token {
    ($token_type:ident; $trait_name:ident;
        $($name:ident (
            $( $negator:ident )?;
            $(
                $variant:ident => ($($str:expr),+)
                $(,)?
            )+
        )),+
        $(,)?
    ) => {
        impl_enum_token!($token_type; $trait_name;
            $($name (
                $(
                    $variant => ($($str),+)
                )+
            )),+
        );

        $(
            $(
                impl OperatorNegator for $name {
                    const NEGATOR: Self = $name::$negator;
                }
            )?
        )+
    };
}

impl_operator_token!(Operator; OperatorTrait;
    MathOp (Subtract;
        Add => ("+"),
        Subtract => ("-"),
        Multiply => ("*"),
        Divide => ("/"),
        Modulo => ("%"),
        Power => ("^"),
        Root => ("√"),
        Factorial => ("!"),
    ),
    LogicOp (Not;
        And => ("&", "^", "∧", "+"),
        Or => ("|", "||", "*"),
        Not => ("!", "~", "¬"),
        XOr => ("^", "⊻", "⨁"),
        XNOr => ("⊙", "⊽"),
        NAnd => ("↑"),
        NOr => ("↓"),
    ),
    // RelationalOp (NotEqual;
    //     Equal => ("==", "="),
    //     NotEqual => ("!=", "≠"),
    //     LessThan => ("<"),
    //     LessThanOrEqual => ("<=", "≤"),
    //     GreaterThan => (">"),
    //     GreaterThanOrEqual => (">=", "≥"),
    // ),
);
```
```rust
// Handle all the Token types that can be a Variable in the grammar! (Constants, etc...)

#![allow(unused)]

use super::*;
use crate::impl_enum_token;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable;


pub trait VariableTrait: Token {
    // * Add properties and methods for the variable type...
}


/// Macro to define constants with their associated subjects and declare the subject traits.
///
/// # Parameters
///
/// - `$subject`: The subject to which the constants belong (e.g., Math, Physics).
/// - `$name`: The name of the constant struct (e.g., Tau, ElectronMass).
/// - `$str`: The string representation of the constant (e.g., "τ", "\\me").
///
/// # Example
///
/// ```rust
/// define_constants!(
///     Math (
///         Tau => "τ",
///         Pi => "π",
///         Phi => "φ",
///         EulerGamma => "γ",
///         Infinity => "∞",
///         E => "e",
///     ),
///     Physics (
///         SpeedOfLight => "\\c",
///         PlanckConstant => "\\h",
///         Gravitational => "\\G",
///         MuonMass => "\\mμ",
///         ElectronMass => "\\me",
///         ProtonMass => "\\mp"
///     )
/// );
/// ```
// * Add this behavior to the macro (+ '\\const_name')
/// Macro to define constants and their associated traits.
macro_rules! define_constants {
    ($token_type:ident; $trait_name:ident;
        $($name:ident (
            $(
                $variant:ident => ($($str:expr),+)
                $(,)?
            )+
        )),+
        $(,)?
    ) => {
        impl_enum_token!($token_type; $trait_name;
            $($name (
                $(
                    $variant => ($($str),+)
                )+
            )),+
        );
    };
}


define_constants!(Variable; VariableTrait;
    MathConst (
        Tau => ("τ", "\\tau"),
        Pi => ("π"),
        Phi => ("φ"),
        EulerGamma => ("γ"),
        Infinity => ("∞"),
        E => ("e"),
    ),
    PhysicConst (
        SpeedOfLight => ("\\c"),
        PlanckConstant => ("\\h"),
        Gravitational => ("\\G"),
        MuonMass => ("\\mμ"),
        ElectronMass => ("\\me"),
        ProtonMass => ("\\mp"),
    ),
);
```