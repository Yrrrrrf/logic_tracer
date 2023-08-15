//! The grammar defines the sequence of tokens that are valid for the language.
//! 
//! It includes BracketState to pair the opening and closing brackets.
//! 
//! Then Grammar Token is the main function of the Parser.

use crate::operators::Operator;

// ? Logic Tokens ---------------------------------------------------------------------------------------------------------------------------------------------------------

// Debug: allows the struct to be printed
// Clone: allows the struct to be cloned (use the clone() method)
// PartialEq: allows the struct to be compared to another struct (use the == operator)
// Default: allows the struct to be created with the default() method

/// The LogicToken Enum describes all the possible tokens that can be recognized by the ['Lexer'].
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GrammarToken {
    /// A token that represents a logic value
    #[default]
    Reading,  // Reading
    /// A token that represents a logic operator
    Operator(Operator),
    /// A token that represents a logic variable
    Variable(String),  // {A-Za-z0-9} 
    /// A token that represents a logic parenthesis
    Brackets(BracketState),
    Space,  // ' '
    /// A token that represents the end of the logic expression
    End,
    /// This token is used when the lexer encounters a Grammar Error
    Error(String),
}


// ? Bracket States ---------------------------------------------------------------------------------------------------------------------------------------------------------


/// This Enum contains the states when the lexer is reading any type of bracket
/// Is used to pair the opening and closing brackets
/// 
/// It also contains the cases for `{`, `}`, `[`, `]`, `(`, `)` and `<`, `>` 
/// 
/// All the Open* BracketStates will generate a new ['Node'] in the ['AST']
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum BracketState {
    /// `(` and `)`
    OpenParenthesis,  // (
    ClosedParenthesis,  // )
    /// `[` and `]
    OpenSquareBracket,  // [
    ClosedSquareBracket,  // ]
    /// `{` and `}`
    OpenCurlyBracket,  // {
    ClosedCurlyBracket,  // }
    /// `<` and `>`
    OpenChevron,  // <
    ClosedChevron,  // >
}
