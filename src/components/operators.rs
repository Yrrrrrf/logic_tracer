//! # Logic Operators
//! 
//! This module contains the behavior of the Operations that can be used in the application.
//! 
//! Rust itself implements some of the logic gates, but not all of them.

use core::fmt;

use crate::util::terminal::set_fg;

// create the Operator dyn trait
// This trait is used to implement the eval() function
pub trait Operator: fmt::Debug + fmt::Display {
    fn to_string(&self) -> String;
}


/// Represents logic operators used for evaluating logical circuits and propositions.
/// 
/// This enum contains various logic operators, including bitwise operations and common logical operators.
/// 
/// # Bitwise Operators
/// - `And` (&): Bitwise AND operator.
/// - `Or` (|): Bitwise OR operator.
/// - `Not` (!): Bitwise NOT operator.
/// - `XOr` (^): Bitwise XOR operator.
/// 
/// # Common Logical Operators
/// - `XNOr`: Exclusive NOR operator (⊙).
/// - `NAnd`: NAND operator (↑).
/// - `NOr`: NOR operator (↓).
/// - `Implies`: Implies operator (→).
/// - `IFf`: If and only if (Iff) operator (↔).
/// 
/// The basic logic operators (And, Or, and Not) have alternative symbols that can be used to represent them.
/// 
/// | Operator | Alternative Symbol | ASCII Value | Unicode Value |
/// | :---: | :---: | :---: | :---: |
/// | And | * | 42 | U+002A |
/// | And | & | 38 | U+0026 |
/// | Or | + | 43 | U+002B |
/// | Or | \| | 124 | U+007C |
/// | Not | ¬ | 172 | U+00AC |
/// | Not | ! | 33 | U+0021 |
/// | XOr | ⊻ | 8859 | U+229B |
/// | XNOr | ⊙ | 8857 | U+2299 |
/// | NAnd | ↑ | 8593 | U+2191 |
/// | NOr | ↓ | 8595 | U+2193 |
/// | Implies | → | 8594 | U+2192 |
/// | IFf | ↔ | 8596 | U+2194 |
#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
    And,
    Or,
    Not,
    XOr,
    XNOr,
    NAnd,
    NOr,
    Implies,
    IFf,
}

impl Operator for LogicOp {
    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }
}

/// Formats a `LogicOp` value for user-facing output.
///
/// The `Display` trait is used to format a value using the [`format!`] macro.
///
/// # Examples
///
/// ```
/// use crate::LogicOp;
///
/// let op = LogicOp::And;
/// assert_eq!(format!("{}", op), "\u{001b}[34m&\u{001b}[0m");
/// ```
impl std::fmt::Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", set_fg(
            match self {  // Match the operator to the symbol
                LogicOp::And => "&",
                LogicOp::Or => "|",
                LogicOp::Not => "!",
                LogicOp::XOr => "^",
                LogicOp::XNOr => "⊙",
                LogicOp::NAnd => "↑",
                LogicOp::NOr => "↓",
                LogicOp::Implies => "→",
                LogicOp::IFf => "↔",
            }, "b")
        )
    }    
}


/// This enum represents various mathematical operators that can be used in calculations.
///
/// # Examples
///
/// ```
/// use logic_tracer::MathOp;
///
/// let op = MathOp::Add;
/// assert_eq!(op, MathOp::Add);
/// ```
///
/// # ASCII and Unicode Values
///
/// Here are the ASCII and Unicode values for each operator:
///
/// | Operator           | Symbol | ASCII | Unicode |
/// | ------------------ | ------ | ----- | ------- |
/// | **Addition**       |   +    |  43   | U+002B  |
/// | **Subtraction**    |   -    |  45   | U+002D  |
/// | **Multiplication** |   *    |  42   | U+002A  |
/// | **Division**       |   /    |  47   | U+002F  |
/// | **Modulo**         |   %    |  37   | U+0025  |
/// | **Exponentiation** |   ^    |  94   | U+005E  |
/// | **Square Root**    |   √    |  8730 | U+221A  |
/// | **Factorial**      |   !    |  33   | U+0021  |
/// | **Absolute Value** |   \|   |  124  | U+007C  |
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum MathOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Root,
    Factorial,
    SomeOtherMathOp,
}

impl Operator for MathOp {
    fn to_string(&self) -> String {
        match self {
            MathOp::Add => "ADD",
            MathOp::Subtract => "SUB",
            MathOp::Multiply => "MUL",
            MathOp::Divide => "DIV",
            MathOp::Modulo => "MOD",
            MathOp::Power => "PWR",
            MathOp::Root => "ROOT",
            MathOp::Factorial => "FAC",
            _ => "UNKNOWN",
        }.to_string()
    }
}

/// Formats a `MathOp` value for user-facing output.
///
/// The `Display` trait is used to format a value using the [`format!`] macro.
///
/// # Examples
///
/// ```
/// use logic_tracer::MathOp;
/// 
/// let op = MathOp::Add;
/// assert_eq!(format!("{}", op), "\u{001b}[34m+\u{001b}[0m");
/// ```
impl std::fmt::Display for MathOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", set_fg(
            match self {  // Match the operator to the symbol
                MathOp::Add => "+",
                MathOp::Subtract => "-",
                MathOp::Multiply => "*",
                MathOp::Divide => "/",
                MathOp::Modulo => "%",
                MathOp::Power => "^",
                MathOp::Root => "√",
                MathOp::Factorial => "!",
                _ => "UNKNOWN",
            }, "b")
        )
    }    
}
