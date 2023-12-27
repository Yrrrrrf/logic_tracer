//! The grammar defines the sequence of tokens that are valid for the language.
//! 
//! It includes BracketState to pair the opening and closing brackets.
//! 
//! Then Grammar Token is the main function of the Parser.

use dev_utils::console::format::set_fg;
use crate::components::operators::*;

// ? Logic Tokens ---------------------------------------------------------------------------------------------------------------------------------------------------------

/// The LogicToken Enum describes all the possible tokens that can be recognized by the 'Lexer'.
#[derive(Clone, Debug, PartialEq)]
pub enum GrammarToken {
    Variable(char),  // {A-Za-z0-9}
    Number(char),  // Number can represent a sequence of digits in any base that count as one number (only if it's a valid number)í
    Dot(),  // Dot is used to separate the integer part from the decimal part of a number
    // todo: change the inpit of Operator from char to <LogicOperator or MathOperator>
    // todo: thir to allow the use of the same char for different operators
    Operator(char),  // {+,-,*,/,%,^,√,!,¬,∧,∨,⊻,⊙,↑,↓,→,↔,=,≠,>,<,≥,≤}
    Invalid(char)  // Any other char
}

impl GrammarToken {
    pub fn from(c: char) -> GrammarToken {
        match c {
            'A'..='Z' | 'a'..='z' => GrammarToken::Variable(c),
            '0'..='9' => GrammarToken::Number(c),
            '.' => GrammarToken::Dot(),  // Only one dot is allowed per number
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
            // Math Exlusive Operators
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
        write!(f, "{}", self.to_string())  // * Use the to_string() method to format the GrammarToken
        // write!(f, "{}", match self {
        //     GrammarToken::Operator(op) => op,
        //     GrammarToken::Variable(var) => var,
        //     GrammarToken::Number(num) => num,
        //     // GrammarToken::BracketState(bracket) => bracket.to_string(),
        //     GrammarToken::Invalid(c) => c,
        //     }.to_string()
        // )
    }
}


// create the public struct Term that contains a sequence of GrammarTokens
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub tokens: Vec<GrammarToken>,
}

impl Term {
    pub fn from(tokens: Vec<GrammarToken>) -> Term {
        Term {tokens}
    }

}

impl Default for Term {
    fn default() -> Self {
        Term {tokens: vec![GrammarToken::Number('0')]}
    }
}
