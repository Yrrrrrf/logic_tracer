// This regex represents a possible Term Negator+(\d+(\.\d+)?)?[a-zA-Z](\_\d+)?

// Regex explanation:
// 1. Term Negator: !?
// 2. Number: (\d+(\.\d+)?)? (can be a float or an integer)
// 3. Variable: [a-zA-Z](\_\d+)? (can be a single letter or a letter with an underscore and a number)
// There can be many variables on the same term, but only one number (At the beginning of the term)


use crate::components::{
    operators::*,
    alphabet::*,
};


#[derive(Clone, Debug, PartialEq)]
pub struct Term<T> where T: Operator {
    pub tokens: Vec<Token<T>>,
}

impl<T> Term<T> where T: Operator {
    pub fn from(tokens: Vec<Token<T>>) -> Term<T> {
        Term { tokens }
    }

    pub fn parse_term(src: &str) -> Result<Vec<Token<T>>, &'static str> {
        let mut tokens = Vec::new();
        let mut chars = src.chars().peekable();

        // Parse Negator (if exists)
        if let Some(&'!') = chars.peek() {
            chars.next(); // Consume '!'
            tokens.push(Token::Operator(T::NEGATOR));
        }

        // Parse Number (if exists)
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) || c == '.' {
                chars.next(); // Consume the digit or dot
                tokens.push(Token::Number(c));
            } else {
                break;
            }
        }

        // Parse Variable
        if let Some(c) = chars.next() {
            if c.is_alphabetic() {
                tokens.push(Token::Variable(c));
                if let Some('_') = chars.next() {
                    tokens.push(Token::UnderScore());
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) {
                            chars.next(); // Consume the digit
                            tokens.push(Token::Number(c));
                        } else {
                            break;
                        }
                    }
                }
            } else {
                return Err("Invalid token in term");
            }
        }

        Ok(tokens)
    }



}
