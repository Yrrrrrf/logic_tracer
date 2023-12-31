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
};


fn main() {
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Trace);

    let my_proposition = LogicProposition::new("}");
    // let my_proposition = LogicProposition::new("{[AB + C_2!D_3 + E_4D_5D_2]}");

    println!("{:#?}", my_proposition)
    // let test_term = Term::<LogicOp>::parse_from_tokens_vec(
    //     my_proposition.unwrap().token_table.clone());

    // println!("{:#?}", test_term)





}
