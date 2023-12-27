//! This module defines the `Propositions` enum, which represents various logical propositions.
//! It is used for modeling logical statements in Rust programs.
//! Import necessary modules and dependencies if needed.
//! The `Propositions` enum represents different logical propositions.

use dev_utils::console::format::set_fg;

// /// The `Proposition` struct represents a logical proposition.
// /// 
// /// It is used for modeling logical statements in Rust programs.
// #[derive(Clone, Default, Debug)]
// pub struct Proposition {
//     pub token_table: Vec<GrammarToken>,
//     // pub f: String,  // function string
//     // pub postfix: String,
//     // pub variables: Vec<>
// }


// impl Proposition {
//     /// Creates a new `Proposition` from the given source string.
//     ///
//     /// If the input string contains unpaired brackets, the function returns an empty `Proposition`.
//     ///
//     /// # Arguments
//     ///
//     /// - `src` - A reference to a string slice containing the source string to parse.
//     ///
//     /// # Returns
//     /// 
//     /// - `Proposition` - A new `Proposition` instance.
//     /// 
//     /// # Examples
//     ///
//     /// ```
//     /// use logic_tracer::grammar::GrammarToken::{self, *};
//     /// use logic_tracer::proposition::Proposition;
//     ///
//     /// assert_eq!(Proposition::new("A & B").token_table, 
//     ///     [Variable('A'), Operator('&'), Variable('B')]);
//     /// assert_eq!(Proposition::new("A + B").token_table, 
//     ///     [Variable('A'), Operator('+'), Variable('B')]);
//     /// // assert_eq!(Proposition::new("A | B").token_table,
//     /// //    [Variable('A'), Operator('+'), Variable('B')]);
//     /// ```
//     pub fn new(src: &str) -> Result<Proposition, &str> {
//         // Lexer -> Remove all unuseful chars & get the TokenTable
//         let trimmed: Vec<char> = src.chars()
//             .filter(|c| !c.is_whitespace() && !c.is_ascii_control()).collect(); // Remove all unuseful chars
//         // Parser -> Validate the sintax & create the AST
//         check_pair_brackets(&trimmed)?;  // Check if the brackets are paired and in the correct order or return an error
//         validate_prop_grammar(&trimmed)?;  // Validate the grammar of the proposition or return an error

//         Ok(Proposition { 
//             token_table: trimmed.iter().map(|c| GrammarToken::from(*c)).collect(),
//             ..Default::default()
//         })
//     }
// }

/// Check if the brackets are paired
/// Also check if the brackets are in the correct order
pub fn check_pair_brackets(src: &Vec<char>) -> Result<bool, &'static str> {
    let mut stack: Vec<char> = Vec::new();
    let brackets: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

    for char in src.clone() {
        match char {
            ch if brackets.iter().any(|(open, _)| open == &ch) => stack.push(char),
            ch if brackets.iter().any(|(_, close)| close == &ch) => {
                if let Some(top) = stack.pop() {
                    if brackets.iter().any(|&(open, close)| open == top && close == ch) {
                        continue;  // If the brackets are paired, continue
                    }
                }
                return Err("Unpaired Brackets");
            }
            _ => (),
        }
    }
    Ok(stack.is_empty())
}




// ? DEFINE THE PROPOSITONS TRAIT =============================================

// Base trait for common functionality
pub trait Proposition {
    fn new(src: &str) -> Self where Self: Sized;
    // fn get_function(&self) -> String;
    // todo: add this methods to the trait
    // fn get_postfix_string(&self) -> String;
    // fn get_prefix_string(&self) -> String;
    // fn get_infix_string(&self) -> String;
}

// Inherit from the base trait and add specific methods
pub trait LogicProposition: Proposition {
    fn evaluate(&self, variables: Vec<bool>) -> bool;
    // Logic-specific methods
    fn get_truth_table_string(&self) -> String;
    fn get_kmap_string(&self) -> String;
    // fn get_result_vec(&mut self) -> Vec<bool>;  // get all the possible results of the proposition
    // fn get_ast_string(&self) -> String;  // get the AST of the proposition
}

// Inherit from the base trait and add specific methods
pub trait MathProposition: Proposition {
    fn evaluate(&self) -> f64;
    // /// Get the plot points of the function
    // fn get_plot_points(&self, x_range: (f64, f64), delta: f64) -> Vec<(f64, f64)>;
    // Math-specific methods
}

// ? MATH PROPOSITIONS ========================================================

// The MathExpr use the - operator to negate the expression value.

enum MathExpr {
    Add(Box<MathExpr>, Box<MathExpr>),
    Subtract(Box<MathExpr>, Box<MathExpr>),
    Multiply(Box<MathExpr>, Box<MathExpr>),
    Divide(Box<MathExpr>, Box<MathExpr>),
    Constant(f64),
    // ... other mathematical operations and variables
}

impl Proposition for MathExpr {
    // The where Self: Sized is needed to allow the use of the Self type in the enum
    // fn new(src: &str) -> Self where Self: Sized {
    fn new(src: &str) -> Self {
        MathExpr::Constant(0.0)
        // Implementation...
    }

    // fn get_function(&self) -> String {
    //     // Common implementation...
    // }
}

// Implement evaluation logic for mathematical expressions
impl MathProposition for MathExpr {
    fn evaluate(&self) -> f64 {
        0.0
        // ... implement evaluation logic
    }
}


// ? LOGIC PROPOSITIONS =======================================================

// The LogicExpr use the ! operator to negate the expression.

enum LogicExpr {
    And(Box<LogicExpr>, Box<LogicExpr>),
    Or(Box<LogicExpr>, Box<LogicExpr>),
    Not(Box<LogicExpr>),
    Variable(bool), // Could be more complex in actual implementation
    // ... other logical operations
}

impl Proposition for LogicExpr {
    fn new(src: &str) -> Self where Self: Sized {
        LogicExpr::Variable(false)
        // Implementation...
    }

    // fn get_function(&self) -> String {
    //     // Common implementation...
    // }

}

// Implement evaluation logic for logical expressions
impl LogicProposition for LogicExpr {
    fn evaluate(&self, variables: Vec<bool>) -> bool {
        false
        // ... implement evaluation logic
    }

    fn get_truth_table_string(&self) -> String {
        "s".to_string()
        // Specific implementation...
    }

    fn get_kmap_string(&self) -> String {
        "s".to_string()
        // Specific implementation...
    }
}
