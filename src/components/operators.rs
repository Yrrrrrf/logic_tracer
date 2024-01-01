//! This module contains the definition of the `Operator` trait and its implementations.
//! 
//! The `Operator` trait is used to represent operators in the expression.
//! It is implemented for various enums, each representing a different type of operator.
use std::fmt;


// /// Enum representing the associativity of an operator.
// /// 
// /// The associativity deCompoundines the order in which operators are evaluated.
// /// 
// /// For example, the expression `1 + 2 + 3` can be evaluated in two ways:
// /// 
// /// - Left-to-right: `(1 + 2) + 3 = 3 + 3 = 6`
// /// - Right-to-left: `1 + (2 + 3) = 1 + 5 = 6`
// /// 
// /// - `Add` operator is left-to-right, so the first
// /// - `Subtract` operator is right-to-left, so the second
// /// - `Multiply` operator is left-to-right, so the first
// /// - `Divide` operator is right-to-left, so the second
// pub enum Associativity {
//     LeftToRight,
//     RightToLeft,
//     None,
// }


// trait OperatorFromChar {
//     fn from_char(c: char) -> Option<Self> where Self: Sized;
// }


/// A trait for operators that can be formatted and debugged.
///
/// This trait is used to implement the `to_string` method for operator enums,
/// allowing them to be easily converted to their string representation.
// pub trait Operator: fmt::Debug + fmt::Display + OperatorFromChar {
pub trait Operator: fmt::Debug + fmt::Display + PartialEq + Clone {
    const NEGATOR: Self;

    fn to_string(&self) -> String;
    // // fn evaluate(&self, operands: &[OperandType]) -> Result<OperandType, Error>;
    // fn precedence(&self) -> usize;
    // fn associativity(&self) -> Associativity;
    // fn is_binary(&self) -> bool;
    // fn is_unary(&self) -> bool;
}


/// Macro to implement the `Operator` and `Display` traits for enums.
///
/// This macro simplifies the process of implementing these traits for
/// different operator enums by automatically generating the necessary code.
macro_rules! impl_operator_traits {
    ($enum_name:ident, {$($variant:ident => $str:expr),* $(,)?}, $negator:expr) => {
        // Implementation of the Operator trait
        impl Operator for $enum_name {

            const NEGATOR: $enum_name = $negator;

            fn to_string(&self) -> String {
                match self {$( $enum_name::$variant => $str.to_string(), )*}
            }
        }
        // Implementation of the Display trait
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {$( $enum_name::$variant => write!(f, "{}", $str), )*}
            }    
        }

    };
}


/// Macro to define enums and implement `Operator` and `Display` traits.
///
/// This macro facilitates the creation of enums representing operators,
/// and automatically implements the `Operator` and `Display` traits for them.
macro_rules! define_operator_enum {
    ($enum_name:ident, {$($variant:ident => [$($str:expr),* $(,)?]),* $(,)?}, $negator:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $enum_name {
            $($variant,)*
        }

        // Implementation of the From trait
        impl $enum_name {    
            pub fn from(c: char) -> Option<$enum_name> {
                match c {
                    $( $( $str => Some($enum_name::$variant), )* )*
                    _ => None,
                }
            }

            pub fn to_char(&self) -> char {
                match self {
                    $( $enum_name::$variant => stringify!($variant).chars().next().unwrap(), )*
                }
            }

        }
        // Implementation of the Operator trait and Display trait
        impl_operator_traits!($enum_name, {$($variant => stringify!($variant)),*}, $negator);
    };
}


// Enum representing various logic operators.
//
// This enum includes both basic and complex logic operators,
// each associated with a specific symbol for display.
define_operator_enum! {
    LogicOp, {
        // ASQII CODE | UNICODE
        And => ['&', '*', '⋅', '∧'],  // 38 | U+0026 _ 42 | U+002A _ 8901 | U+22C5 _ 8743 | U+2227
        Or => ['+', '|'],  // 43 | U+002B _ 124 | U+007C
        Not => ['!', '¬', '\''],  // 33 | U+0021 _ 172 | U+00AC _ 8216 | U+2018
        XOr => ['^', '⊻'],  // 94 | U+005E _ 8853 | U+22BB
        XNOr => ['⊙'],  // 8855 | U+22BD
        NAnd => ['↑'],  // 8593 | U+2191
        NOr => ['↓'],  // 8595 | U+2193
        Implies => ['→'],  // 8594 | U+2192
        IFf => ['↔'],  // 8596 | U+2194
    }, LogicOp::Not
}


// Enum representing various mathematical operators.
//
// This enum covers a range of common mathematical operations,
// such as addition, subtraction, and more, each with its own symbol.
define_operator_enum! {
    MathOp, {
        // ASQII CODE | UNICODE
        Add => ['+'],  // 43 | U+002B
        Subtract => ['-'],  // 45 | U+002D
        Multiply => ['*'],  // 42 | U+002A
        Divide => ['/'],  // 47 | U+002F
        Modulo => ['%'],  // 37 | U+0025
        Power => ['^'],  // 94 | U+005E
        Root => ['√'],  // 8730 | U+221A
        Factorial => ['!'],  // 33 | U+0021
    }, MathOp::Subtract
}


/// Enum representing various relation operators.
/// 
/// This enum covers a range of common relation operations,
/// such as equal, not equal, greater than, less than, etc.
/// 
/// The symbols used for the operators are the same as the ones used in the
/// `LogicOp` enum, but they are different operators.
define_operator_enum!{
    RelOp, {
        // ASQII CODE | UNICODE
        
        Equal => ['='],  // 61 | U+003D
        NotEqual => ['≠'],  // 8800 | U+2260
        LessThan => ['<'],  // 60 | U+003C
        GreaterThan => ['>'],  // 62 | U+003E
        LessThanOrEqual => ['≤'],  // 8804 | U+2264
        GreaterThanOrEqual => ['≥'],  // 8805 | U+2265
    }, RelOp::Equal
}
