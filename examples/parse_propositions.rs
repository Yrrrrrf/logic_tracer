//! This module provides a regex matching functionality.

use dev_utils::{
    print_app_data,
    log::rlog::RLog,
};
use logic_tracer::{
    grammar::GrammarToken, 
    LogicOp, 
    MathOp
};


fn main() {
    print_app_data(file!());
    RLog::init_logger(log::LevelFilter::Debug);

    let src_vec = vec![
        // Some math expressions
        "A + B",
        "A + B * C",
        "A + B * C / D",
        "A + B - C",
        "A + B - C * D",
        // Some logic expressions
        "A & B",
        "A & B | C",
        "A & B | C & D",
        "A & B | C ! D + E",
    ];

    for expression in src_vec.iter() {
        let src: Vec<char> = expression.chars()  // Remove whitespace and control characters from the expression
            .filter(|c| !c.is_whitespace() && !c.is_ascii_control()).collect();

        println!("\nSource Expression: {}", expression);  // Print the source expression

        println!("Logic: {:?}",  // Parse as a Logic Expression
            src.iter().map(|c| GrammarToken::<LogicOp>::from(*c)).collect::<Vec<GrammarToken<LogicOp>>>()
        );
        println!("Math: {:?}",  // Parse as a Math Expression
            src.iter().map(|c| GrammarToken::<MathOp>::from(*c)).collect::<Vec<GrammarToken<MathOp>>>()
        );

    }
    

}
