//! The grammar defines the sequence of tokens that are valid for the language.
//! 
//! It includes BracketState to pair the opening and closing brackets.
//! 
//! Then Grammar Token is the main function of the Parser.

use crate::components::operators::*;


// ? Logic Tokens ---------------------------------------------------------------------------------------------------------------------------------------------------------

/// The LogicToken Enum describes all the possible tokens that can be recognized by the ['Lexer'].
#[derive(Clone, Debug, PartialEq)]
pub enum GrammarToken {
    Operator(LogicOp),  // TODO: CHANGE THIS TO A GENERIC OPERATOR ENUM
    Variable(char),  // {A-Za-z0-9}
    // Number can be an integer or a float, so. The Number Enum contains the float value of the number
    Number(u8),  // Number can represent a sequence of digits in any base that count as one number (only if it's a valid number)
    Brackets(BracketState),
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
    CloseParenthesis,  // )
    /// `[` and `]
    OpenSquareBracket,  // [
    CloseSquareBracket,  // ]
    /// `{` and `}`
    OpenCurlyBracket,  // {
    CloseCurlyBracket,  // }
    /// `<` and `>`
    OpenChevron,  // <
    CloseChevron,  // >
}
