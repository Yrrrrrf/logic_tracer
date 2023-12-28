#![allow(unused)]

// ? Import modules -----------------------------------------------------------------------------------------------------------

// Standard library

// Third party crates
use dev_utils::{
    print_app_data,
    log::rlog::RLog,
};
use log::LevelFilter;

// Internal modules
mod components;
use components::{
    operators::*,  // LogicOp, MathOp, RelOp (RelOp is: Relation Operator)
    // A RELATION OPERATOR IS: =, ≠, >, <, ≥, ≤
    proposition::*,
    grammar::*,
    // circuits::*,
};
// * Prototyped modules (Not ready for production)
mod proto;
use proto::logic_notation::*;
// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Trace);


    // let prop: Proposition<LogicOp> = Proposition::new("A & B | C ! D + E"); // Change the type of LogicOp
    let my_proposition = LogicProposition::new("yo^%ur_input_string");

    println!("Proposition: {:?}", my_proposition);
}





