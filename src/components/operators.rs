//! # Logic Operators
//! 
//! This module contains the behavior of the Operations that can be used in the application.
//! 
//! Rust itself implements some of the logic gates, but not all of them.

use core::fmt;

use crate::util::terminal::set_fg;

/// This enum contains all the possible operators that can be used in the application.
#[derive(Clone, PartialEq)]
pub enum Operator {
    Logic(LogicOp),
    Math(MathOp),
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
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
/// | AND | & | 38 | U+0026 |
/// | AND | ^ | 94 | U+005E |
/// | AND | * | 42 | U+002A |
/// | OR | \| | 124 | U+007C |
/// | OR | ∨ | 8744 | U+2228 |
/// | OR | + | 43 | U+002B |
/// | NOT | ! | 33 | U+0021 |
/// | NOT | ¬ | 172 | U+00AC |
/// | NAND | ↑ | 8593 | U+2191 |
/// | NOR | ↓ | 8595 | U+2193 |
/// | XOR | ⊕ | 8853 | U+2295 |
/// | XNOR | ⊙ | 8857 | U+2299 |
/// | IMPLIES | → | 8594 | U+2192 |
/// | IFF | ↔ | 8596 | U+2194 |
#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
    /// `&` -> ASCII 38 and Unicode U+0026  
    /// `^` -> ASCII 94 and Unicode U+005E
    /// `*` -> ASCII 42 and Unicode U+002A
    And,  // &, ∧
    /// `|` -> ASCII 124 and Unicode U+007C  
    /// `∨` -> ASCII 8744 and Unicode U+2228  
    /// `+` -> ASCII 43 and Unicode U+002B
    Or,  // |, ∨, +
    /// `!` -> ASCII 33 and Unicode U+0021  
    /// `¬` -> ASCII 172 and Unicode U+00AC
    Not,  // !, ¬
    /// `↑` -> ASCII 8593 and Unicode U+2191
    NAnd,  // ↑
    /// `↓` -> ASCII 8595 and Unicode U+2193
    NOr,  // ↓
    /// `⊕` -> ASCII 8853 and Unicode U+2295
    XOr,  // ⊕
    /// `⊙` -> ASCII 8857 and Unicode U+2299
    XNOr,  // ⊙
    /// `→` -> ASCII 8594 and Unicode U+2192
    Implies,  // →
    /// `↔` -> ASCII 8596 and Unicode U+2194
    IFf,  // ↔
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
#[derive(Debug, Clone, PartialEq)]
pub enum MathOp {
    /// `+` -> ASCII 43 and Unicode U+002B
    Add,
    /// `-` -> ASCII 45 and Unicode U+002D
    Subtract,
    /// `*` -> ASCII 42 and Unicode U+002A
    Multiply,
    /// `/` -> ASCII 47 and Unicode U+002F
    Divide,
    /// `%` -> ASCII 37 and Unicode U+0025
    Modulo,
    /// `^` -> ASCII 94 and Unicode U+005E
    Power,
    /// `√` -> ASCII 8730 and Unicode U+221A
    Root,
    /// `!` -> ASCII 33 and Unicode U+0021
    Factorial,
    /// `|` -> ASCII 124 and Unicode U+007C
    AbsoluteValue,
}


// Implement my own Debug trait
impl fmt::Debug for Operator {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("Operator", "g"))
            // .field("Type:", 

            // todo: add a better way to print the data for this operator
            // print the ways to represent it (ASCII and Unicode)

            .finish()
    }
}
