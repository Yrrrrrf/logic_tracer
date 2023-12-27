//! The grammar defines the sequence of tokens that are valid for the language.
//! 
//! It includes BracketState to pair the opening and closing brackets.
//! 
//! Then Grammar Token is the main function of the Parser.

use dev_utils::console::format::set_fg;
use crate::components::operators::*;


/// The LogicToken Enum describes all the possible tokens that can be recognized by the 'Lexer'.
#[derive(Clone, Debug, PartialEq)]
pub enum GrammarToken<T> where T: Operator {
// pub enum GrammarToken<T> where T: Operator {
    Variable(char),  // Any letter from A to Z (uppercase or lowercase)
    Number(char),  // Represent a sequence of digits in any base that count as one number
    UnderScore(),  // Used to add a "Subterm" to a term (e.g. A_1, A_2, A_3, ...)
    Dot(),  // Separate the integer part from the decimal part of a number
    Operator(T),  // {+,-,*,/,%,^,√,!,¬,∧,∨,⊻,⊙,↑,↓,→,↔,=,≠,>,<,≥,≤}
    Invalid(char)  //* Any other char that is not recognized by the 'Lexer'
}

impl<T> GrammarToken<T> where T: Operator {
    fn from_char(c: char, op_converter: impl Fn(char) -> Option<T>) -> GrammarToken<T> {
        match c {
            'A'..='Z' | 'a'..='z' => GrammarToken::Variable(c),
            '0'..='9' => GrammarToken::Number(c),
            '_' => GrammarToken::UnderScore(),
            '.' => GrammarToken::Dot(),
            _ => match op_converter(c) {
                Some(op) => GrammarToken::Operator(op),
                None => GrammarToken::Invalid(c),
            },
        }
    }
}

impl GrammarToken<LogicOp> {
    pub fn from(c: char) -> GrammarToken<LogicOp> {
        GrammarToken::<LogicOp>::from_char(c, LogicOp::from)
    }
}

impl GrammarToken<MathOp> {
    pub fn from(c: char) -> GrammarToken<MathOp> {
        GrammarToken::<MathOp>::from_char(c, MathOp::from)
    }
}




#[derive(Clone, Debug, PartialEq)]
pub struct Term<T> where T: Operator {
    pub tokens: Vec<GrammarToken<T>>,
}

impl<T> Term<T> where T: Operator {
    pub fn from(tokens: Vec<GrammarToken<T>>) -> Term<T> {
        Term { tokens }
    }
}
