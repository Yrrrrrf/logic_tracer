//! This module contains all the possible states that the ['Lexer'] can be in.

use crate::LogicOperators;

// ? Logic Tokens ---------------------------------------------------------------------------------------------------------------------------------------------------------

// Debug: allows the struct to be printed
// Clone: allows the struct to be cloned (use the clone() method)
// PartialEq: allows the struct to be compared to another struct (use the == operator)
#[derive(Debug, Clone, PartialEq)]
/// The LogicToken Enum describes all the possible tokens that can be recognized by the ['Lexer'].
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
