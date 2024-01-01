#![allow(unused)]

use log::LevelFilter;
use dev_utils::{
    print_app_data, 
    log::rlog::RLog
};
use logic_tracer::{
    PropositionTrait,
    Token,
    MathProposition, 
    MathOp,
    LogicProposition,
    LogicOp,
    Operator,
    Term,
    BracketType,
};


fn main() {
    // print_app_data(file!());    
    // RLog::init_logger(LevelFilter::Trace);

    // let my_proposition = LogicProposition::new("{[AB + C_2!D_3 + E_4D_5D_2]}");

    // todo: Add these to the lib tests
    // let my_proposition = LogicProposition::new("!a");
    // let my_proposition = LogicProposition::new("(b_)3+a+v+!a");
    // let my_proposition = LogicProposition::new("+b_2+a+v+!a");
    
    // ? testing proposition
    let my_proposition = LogicProposition::new("a_+b+!(cd)");
    
    
    println!("{:#?}", my_proposition);

    // let test_term = Term::<LogicOp>::parse_from_tokens_vec(
    //     my_proposition.unwrap().token_table.clone());
    // println!("{:#?}", test_term)
}
