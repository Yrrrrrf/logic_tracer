//! # Logic Operators
//! 
//! This module contains the behavior of the operations that can be used in the application.
//! 
//! Rust itself implements some of the logic gates, but not all of them. This module
//! extends Rust's capabilities by providing additional logic and mathematical operators.

use core::fmt;
use dev_utils::console::format::set_fg;


/// A trait for operators that can be formatted and debugged.
///
/// This trait is used to implement the `to_string` method for operator enums,
/// allowing them to be easily converted to their string representation.
pub trait Operator: fmt::Debug + fmt::Display {
    fn to_string(&self) -> String;
}


/// Macro to implement the `Operator` and `Display` traits for enums.
///
/// This macro simplifies the process of implementing these traits for
/// different operator enums by automatically generating the necessary code.
macro_rules! impl_operator_traits {
    ($enum_name:ident, {$($variant:ident => $str:expr),* $(,)?}) => {
        // Implementation of the Operator trait
        impl Operator for $enum_name {
            fn to_string(&self) -> String {
                match self {
                    $( $enum_name::$variant => $str.to_string(), )*
                }
            }
        }

        // Implementation of the Display trait
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", set_fg(
                    match self {
                        $( $enum_name::$variant => $str, )*
                    }, "b")
                )
            }    
        }
    };
}


/// Macro to define enums and implement `Operator` and `Display` traits.
///
/// This macro facilitates the creation of enums representing operators,
/// and automatically implements the `Operator` and `Display` traits for them.
macro_rules! define_operator_enum {
    ($enum_name:ident, {$($variant:ident => $str:expr),* $(,)?}) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $enum_name {
            $($variant,)*
        }

        impl $enum_name {
            pub fn from(c: char) -> Option<$enum_name> {
                match c {
                    $( c => Some($enum_name::$variant), )*
                    _ => None,
                }
            }
        }

        impl_operator_traits!($enum_name, {$($variant => $str),*});
    };
}


// Enum representing various logic operators.
//
// This enum includes both basic and complex logic operators,
// each associated with a specific symbol for display.
define_operator_enum! {
    LogicOp, {
        And => "&",  // AND 0001
        Or => "|",  // OR 0111
        Not => "!",  // Negation 0->1, 1->0
        NAnd => "↑",  // Negated AND  1110
        NOr => "↓",  // Negated OR  1000
        XOr => "^",  // Exclusive OR  0110
        XNOr => "⊙",  // Exclusive NOR  1001 (Negated XOR)
        Implies => "→",  // Implication  0110  (A -> B)  (If A then B)
        IFf => "↔",  // If and only if  1001  (A <-> B)  (A is equivalent to B)
    }
}


// Enum representing various mathematical operators.
//
// This enum covers a range of common mathematical operations,
// such as addition, subtraction, and more, each with its own symbol.
define_operator_enum! {
    MathOp, {
        Add => "+",
        Subtract => "-",
        Multiply => "*",
        Divide => "/",
        Modulo => "%",
        Power => "^",
        Root => "√",
        Factorial => "!",
        SomeOtherMathOp => "UNKNOWN",
    }
}
