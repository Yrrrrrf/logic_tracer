#![allow(unused)]

use log::LevelFilter;
use dev_utils::{print_app_data, log::rlog::RLog, };
use logic_tracer::{
    PropositionTrait,
    Token,
    MathProposition, 
    MathOp,
    LogicProposition,
    LogicOp, Operator,
};


fn main() {
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Trace);

    let my_proposition = LogicProposition::new("A & B | C ! D + E");
    // let mut my_proposition = MathProposition::new("A & B | C ! D + E");

    my_proposition.unwrap().token_table.iter().for_each(|token| {
        match token == &Token::Operator(LogicOp::Not) {
            true => log::info!("Found a NOT operator"),
            false => log::info!("{:?}", token),
        }
    });

    println!("{:?}", LogicOp::NEGATOR);
    println!("{:?}", MathOp::NEGATOR);

    // println!("{:?}", my_proposition_a.unwrap());
    // println!("{:?}", my_proposition_b);
}
