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
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Info);
    // RLog::init_logger(LevelFilter::Debug);
    // RLog::init_logger(LevelFilter::Trace);

    // the log important levels are:
    // - Error (print errors)
    // - Warn  (print errors and warnings)
    // - Info  (print errors, warnings and info)
    // - Debug (print errors, warnings, info and debug)
    // - Trace (print everything)
    
    // todo: Add these to the lib tests
    let test_props = vec![
        "{[AB + C_2!D_3 + E_4D_5D_2]}",
        "!a",
        "(b_)3+a+v+!a",
        "b_)3+a+v+!a",
        "+b_2+a+v+!a",
        "28a_3+b+!(cd)",
        "!A+",
        "!A(B+C)",
        "{{B}}",
        "![A(B(&(!C)))]",
        "![A(B(&(!C)))]",
        "A+1A",
        "1",
        "1+A",
        "2a_3+3b_3",
        "2.2a_3*3b_3",
    ];

    // ? Test proposition
    let my_proposition = LogicProposition::new("2a_3*3b_3");
    
    println!("{:?}", my_proposition);
    // println!("{:#?}", my_proposition);
    
    // let test_Compound = Compound::<LogicOp>::parse_from_tokens_vec(
        //     my_proposition.unwrap().token_table.clone());
    // println!("{:#?}", test_Compound)
}
