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
pub enum TokenOrTerm<T> where T: Operator {
    SingleToken(Token<T>),
    Term(Term<T>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Term<T> where T: Operator {
    pub tokens: Vec<Token<T>>,
}

impl<T> Term<T> where T: Operator {
    pub fn from(tokens: Vec<Token<T>>) -> Self {
        Self {tokens,}
    }

    pub fn parse_from_tokens_vec(src_tokens: Vec<Token<T>>) -> Vec<TokenOrTerm<T>> {
        let mut iter = src_tokens.into_iter().peekable();
        let mut result_tokens = Vec::new();
    
        while let Some(token) = iter.next() {
            let mut term_tokens = Vec::new();  // Collect tokens for a term
            match &token {  // Start building a term if the token is a negator, number, or variable
                Token::Operator(op) if *op == T::NEGATOR => term_tokens.push(token),
                Token::Number(_) => term_tokens.push(token),
                Token::Variable(_) => {  // Add the variable and continue adding to term if there's an underscore followed by numbers
                    term_tokens.push(token);
                    // Continue adding to term if there's an underscore followed by numbers
                    if iter.peek() == Some(&Token::UnderScore()) {
                        term_tokens.push(iter.next().unwrap()); // Consume the underscore
                        while iter.peek().map_or(false, |t| matches!(t, Token::Number(_))) {
                            term_tokens.push(iter.next().unwrap()); // Consume the number
                        }
                    }
                    result_tokens.push(TokenOrTerm::Term(Term::from(term_tokens)));  // Add the collected term
                }
                _ => result_tokens.push(TokenOrTerm::SingleToken(token)),
            }
        }
        result_tokens
    }
    
}
