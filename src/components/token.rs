//! # Tokens
//! 
//! This module contains the different types of tokens that can be used in the application.
//! 
//! 

// ? Logic Tokens ---------------------------------------------------------------------------------------------------------------------------------------------------------

// Debug: allows the struct to be printed
// Clone: allows the struct to be cloned (use the clone() method)
// PartialEq: allows the struct to be compared to another struct (use the == operator)
#[derive(Debug, Clone, PartialEq)]
pub enum LogicToken {
    /// A token that represents a logic operator
    LogicOperator(LogicOperators),
    /// A token that represents a logic variable
    LogicVariable(String),
    /// A token that represents a logic parenthesis
    Parenthesis(String),
    /// A token that represents a logic space
    Space,
    Reading,
    /// A token that represents the end of the file
    EndOfFile,
    /// A token that represents an error
    Error(String),
}



#[derive(Debug, Clone, PartialEq)]
/// This enum contains all the possible logic operators that can be used in the application.  
/// These operators are used to evaluate the logic of a circuit or proposition.
/// 
/// The most basic logic operators (AND, OR, and NOT) have two symbols that can be used to represent them.
/// 
/// | Operator | Symbol | ASCII Value | Unicode Value |
/// | :---: | :---: | :---: | :---: |
/// | AND | & | 38 | U+0026 |
/// | AND | ∧ | 8743 | U+2227 |
/// | OR | \| | 124 | U+007C |
/// | OR | ∨ | 8744 | U+2228 |
/// | NOT | ! | 33 | U+0021 |
/// | NOT | ¬ | 172 | U+00AC |
/// | NAND | ↑ | 8593 | U+2191 |
/// | NOR | ↓ | 8595 | U+2193 |
/// | XOR | ⊕ | 8853 | U+2295 |
/// | XNOR | ⊙ | 8857 | U+2299 |
/// | IMPLIES | → | 8594 | U+2192 |
/// | IFF | ↔ | 8596 | U+2194 |
pub enum LogicOperators {
    /// & -> ASCII 38 and Unicode U+0026  
    /// ∧ -> ASCII 8743 and Unicode U+2227
    AND,  // &, ∧
    /// | -> ASCII 124 and Unicode U+007C  
    /// ∨ -> ASCII 8744 and Unicode U+2228
    OR,  // |, ∨
    /// ! -> ASCII 33 and Unicode U+0021  
    /// ¬ -> ASCII 172 and Unicode U+00AC
    NOT,  // !, ¬

    /// ↑ -> ASCII 8593 and Unicode U+2191
    NAND,  // ↑
    /// ↓ -> ASCII 8595 and Unicode U+2193
    NOR,  // ↓
    /// ⊕ -> ASCII 8853 and Unicode U+2295
    XOR,  // ⊕
    /// ⊙ -> ASCII 8857 and Unicode U+2299
    XNOR,  // ⊙
    /// → -> ASCII 8594 and Unicode U+2192
    IMPLIES,  // →
    /// ↔ -> ASCII 8596 and Unicode U+2194
    IFF,  // ↔
}


impl LogicOperators {
    /// `Override` the default `to_string()` method to return a string representation of the token.
    pub fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
}

