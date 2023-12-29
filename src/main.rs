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
    alphabet::*,
    // circuits::*,
};
// * Prototyped modules (Not ready for production)
mod proto;
use proto::logic_notation::*;
// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Trace);


    // let my_proposition_a = LogicProposition::new("A & B | C ! D + E");
    let my_proposition_b = MathProposition::new("A & B | C ! D + E");

    // println!("{:?}", my_proposition_a.unwrap());
    println!("{:?}", my_proposition_b);
}
