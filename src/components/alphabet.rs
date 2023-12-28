//! The grammar defines the sequence of tokens that are valid for the language.
//! 
//! It includes BracketState to pair the opening and closing brackets.
//! 
//! Then Grammar Token is the main function of the Parser.
use dev_utils::console::format::set_fg;
use crate::components::operators::*;


/// The LogicToken Enum describes all the possible tokens that can be recognized by the 'Lexer'.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<T> where T: Operator {
    Variable(char),  // Any letter from A to Z (uppercase or lowercase)
    Number(char),  // Represent a sequence of digits in any base that count as one number
    UnderScore(),  // Used to add a "Subterm" to a term (e.g. A_1, A_2, A_3, ...)
    Dot(),  // Separate the integer part from the decimal part of a number
    Operator(T),  // Any type that implements the ['Operator'] trait
    OpenBracket(BracketType),  // Any type of bracket that opens another level of nesting
    CloseBracket(BracketType),  // Any type of bracket that closes a level of nesting
    Invalid(char),  //* Any other char that is not recognized by the 'Lexer'
}

impl<T> Token<T> where T: Operator {
    pub fn from_char(c: char, op_converter: impl Fn(char) -> Option<T>) -> Token<T> {
        Self::from_bracket(c).unwrap_or_else(|| match c {
            'A'..='Z' | 'a'..='z' => Token::Variable(c),
            '0'..='9' => Token::Number(c),
            '_' => Token::UnderScore(),
            '.' => Token::Dot(),
            _ => match op_converter(c) {
                Some(op) => Token::Operator(op),
                None => Token::Invalid(c),
            },
        })
    }


    pub fn from_bracket(c: char) -> Option<Token<T>> {
        match c {
            '(' => Some(Token::OpenBracket(BracketType::Parenthesis)),
            ')' => Some(Token::CloseBracket(BracketType::Parenthesis)),
            '[' => Some(Token::OpenBracket(BracketType::Square)),
            ']' => Some(Token::CloseBracket(BracketType::Square)),
            '{' => Some(Token::OpenBracket(BracketType::Curly)),
            '}' => Some(Token::CloseBracket(BracketType::Curly)),
            _ => None,
        }
    }

    // pub fn to_char(&self) -> char {
    //     match self {
    //         Token::Variable(c) => *c,
    //         Token::Number(c) => *c,
    //         Token::UnderScore() => '_',
    //         Token::Dot() => '.',
    //         Token::Operator(op) => op.to_string().chars().next().unwrap(),
    //         Token::OpenBracket(bracket_type) => bracket_type.to_char(),
    //         Token::CloseBracket(bracket_type) => bracket_type.to_char(),
    //         Token::Invalid(c) => *c,
    //     }
    // }

    // This is a helper function, to it should probably be private
    // Also it should be moved to the 'Lexer' module
    pub fn is_invalid(&self) -> bool {
        matches!(self, Token::Invalid(_))
    }
}

impl Token<LogicOp> {
    pub fn from(c: char) -> Self {
        Token::<LogicOp>::from_char(c, LogicOp::from)
    }
}

impl Token<MathOp> {
    pub fn from(c: char) -> Self {
        Token::<MathOp>::from_char(c, MathOp::from)
    }
}


/// Represents types of brackets.
#[derive(Clone, Debug, PartialEq)]
pub enum BracketType {
    Parenthesis,  // ()
    Square,       // []
    Curly,        // {}
    // This shouldn't be used because
    // It can be confused with the less than and greater than operators
    // Angle,        // <> 
}
