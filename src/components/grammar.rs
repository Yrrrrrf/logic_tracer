//! The grammar defines the sequence of tokens that are valid for the language.
//! 
//! It includes BracketState to pair the opening and closing brackets.
//! 
//! Then Grammar Token is the main function of the Parser.

use std::fmt::write;

use dev_utils::terminal::set_fg;

use crate::components::operators::*;


// ? Logic Tokens ---------------------------------------------------------------------------------------------------------------------------------------------------------

/// The LogicToken Enum describes all the possible tokens that can be recognized by the ['Lexer'].
#[derive(Clone, Debug, PartialEq)]
pub enum GrammarToken {
    Operator(char),
    Variable(char),  // {A-Za-z0-9}
    Number(u8),  // Number can represent a sequence of digits in any base that count as one number (only if it's a valid number)
    // BracketState(char),
    Invalid(char)
}

impl GrammarToken {
    pub fn from(c: char) -> GrammarToken {
        match c {
            'A'..='Z' | 'a'..='z' => GrammarToken::Variable(c),
            '0'..='9' => GrammarToken::Number(c.to_digit(10).unwrap() as u8),
            '.' => GrammarToken::Number(10),  // * Decimal point
            // '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => GrammarToken::BracketState(c),
            // * OPERATORS
            // Logic Operators  
            '&' | '*' | '⋅' | '∧' => GrammarToken::Operator(c),  // * AND or MULTIPLY
            '+' | '|' => GrammarToken::Operator(c),  // * OR or ADD (SUM)
            '!' | '¬' => GrammarToken::Operator(c),  // * NOT or FACTORIAL
            '^' | '⊻' => GrammarToken::Operator(c),  // * XOR or POWER
            '⊙' => GrammarToken::Operator(c),  // XNOR (Exclusive NOR)
            '↑' => GrammarToken::Operator(c),  // NAND
            '↓' => GrammarToken::Operator(c),  // NOR
            '→' => GrammarToken::Operator(c),  // IMPLIES
            '↔' => GrammarToken::Operator(c),  // IFF
            // Math Operators
            '/' => GrammarToken::Operator(c),  // DIVIDE
            '-' => GrammarToken::Operator(c),  // SUBTRACT
            '%' => GrammarToken::Operator(c),  // MODULO
            '√' => GrammarToken::Operator(c),  // SQUARE ROOT
            // * Any other char
            _ => GrammarToken::Invalid(c),  // Invalid chars
        }
    }
}


// impl the Display trait for GrammarToken
impl std::fmt::Display for GrammarToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GrammarToken::Operator(op) => op.to_string(),
            GrammarToken::Variable(var) => var.to_string(),
            GrammarToken::Number(num) => num.to_string(),
            // GrammarToken::BracketState(bracket) => bracket.to_string(),
            GrammarToken::Invalid(c) => set_fg(&c.to_string(), "r")
            }
        )
    }
}


// ? Bracket States ---------------------------------------------------------------------------------------------------------------------------------------------------------
// This is only used to pair the opening and closing brackets
// But I didn't use it in the lexer, I just used the char directly

// /// This Enum contains the states when the lexer is reading any type of bracket
// /// Is used to pair the opening and closing brackets
// /// 
// /// It also contains the cases for `{`, `}`, `[`, `]`, `(`, `)` and `<`, `>` 
// /// 
// /// All the Open* BracketStates will generate a new ['Node'] in the ['AST']
// /// 
// #[derive(Debug, Clone, PartialEq)]
// pub enum BracketState {
//     /// `(` and `)`
//     OpenParenthesis,  // (
//     CloseParenthesis,  // )
//     /// `[` and `]
//     OpenSquareBracket,  // [
//     CloseSquareBracket,  // ]
//     /// `{` and `}`
//     OpenCurlyBracket,  // {
//     CloseCurlyBracket,  // }
//     /// `<` and `>`
//     OpenChevron,  // <
//     CloseChevron,  // >
// }

// // IMPLEMENT DISPLAY FOR BRACKETSTATE
// impl std::fmt::Display for BracketState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", 
//             match self {
//                 BracketState::OpenParenthesis => "(",
//                 BracketState::CloseParenthesis => ")",
//                 BracketState::OpenSquareBracket => "[",
//                 BracketState::CloseSquareBracket => "]",
//                 BracketState::OpenCurlyBracket => "{",
//                 BracketState::CloseCurlyBracket => "}",
//                 BracketState::OpenChevron => "<",
//                 BracketState::CloseChevron => ">",
//             }
//         )
//     }    
// }
