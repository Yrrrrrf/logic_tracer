//! # Logic Operators
//! 
//! This module contains the behavior of the Operations that can be used in the application.
//! 
//! Rust itself implements some of the logic gates, but not all of them.

use core::fmt;

use crate::util::terminal::set_fg;

// create the Operator dyn trait
// This trait is used to implement the eval() function
pub trait Operator {
    /// This function is used to evaluate the operator
    fn to_string(&self) -> String;
}

/// This enum contains all the possible logic operators that can be used in the application.  
/// These operators are used to evaluate the logic of a circuit or proposition.
/// 
/// ### Rust built-in logic gates (operators)
/// - AND (&)
/// - OR (|)
/// - NOT (!)
///  
/// The most basic logic operators (AND, OR, and NOT) have two symbols that can be used to represent them.
/// 
/// | Operator | Symbol | ASCII Value | Unicode Value |
/// | :---: | :---: | :---: | :---: |
/// | AND | * | 42 | U+002A |
/// | AND | & | 38 | U+0026 |
/// | AND | ^ | 94 | U+005E |
/// | OR | + | 43 | U+002B |
/// | OR | \| | 124 | U+007C |
/// | OR | ∨ | 8744 | U+2228 |
/// | NOT | ! | 33 | U+0021 |
/// | NOT | ¬ | 172 | U+00AC |
/// | XOR | ⊕ | 8853 | U+2295 |
/// | XOR | ⊻ | 8859 | U+229B |
/// | XNOR | ⊙ | 8857 | U+2299 |
/// | NAND | ↑ | 8593 | U+2191 |
/// | NOR | ↓ | 8595 | U+2193 |
/// | IMPLIES | → | 8594 | U+2192 |
/// | IFF | ↔ | 8596 | U+2194 |
#[derive(Debug, Clone, PartialEq)]
// #[derive(Debug, Clone, Default, PartialEq)]
pub enum LogicOp {
    And,  // & ∧ *
    Or,  // | ∨ +
    Not,  // ! ¬
    XOr,  // ⊕ ⊻
    XNOr,  // ⊙
    NAnd,  // ↑
    NOr,  // ↓
    Implies,  // →
    IFf,  // ↔
}

impl Operator for LogicOp {
    fn to_string(&self) -> String {
        match self {
            LogicOp::And => "AND".to_string(),
            LogicOp::Or => "OR".to_string(),
            LogicOp::Not => "NOT".to_string(),
            LogicOp::XOr => "XOR".to_string(),
            LogicOp::XNOr => "XNOR".to_string(),
            LogicOp::NAnd => "NAND".to_string(),
            LogicOp::NOr => "NOR".to_string(),
            LogicOp::Implies => "IMPLIES".to_string(),
            LogicOp::IFf => "IFF".to_string(),
        }
    }
}


/// Math Operators
/// 
/// This enum contains all the possible math operators that can be used in the application.
/// 
/// | Operator | Symbol | ASCII Value | Unicode Value |
/// | :---: | :---: | :---: | :---: |
/// | ADD | + | 43 | U+002B |
/// | SUBTRACT | - | 45 | U+002D |
/// | MULTIPLY | * | 42 | U+002A |
/// | DIVIDE | / | 47 | U+002F |
/// | MODULO | % | 37 | U+0025 |
/// | POWER | ^ | 94 | U+005E |
/// | ROOT | √ | 8730 | U+221A |
/// | FACTORIAL | ! | 33 | U+0021 |
/// | ABSOLUTE VALUE | \| | 124 | U+007C |
// #[derive(Debug, Clone, Default, PartialEq)]
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
    AbsoluteValue,
}

impl Operator for MathOp {
    fn to_string(&self) -> String {
        match self {
            MathOp::Add => "ADD".to_string(),
            MathOp::Subtract => "SUB".to_string(),
            MathOp::Multiply => "MUL".to_string(),
            MathOp::Divide => "DIV".to_string(),
            MathOp::Modulo => "MOD".to_string(),
            MathOp::Power => "PWR".to_string(),
            MathOp::Root => "ROOT".to_string(),
            MathOp::Factorial => "FAC".to_string(),
            MathOp::AbsoluteValue => "ABS".to_string(),
        }
    }
}
