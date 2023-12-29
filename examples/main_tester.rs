#![allow(unused)]

// ? Import modules -----------------------------------------------------------------------------------------------------------

// Third party crates
use log::LevelFilter;
use dev_utils::{print_app_data, log::rlog::RLog, };
use logic_tracer::{
    MathProposition, 
    PropositionTrait
};


// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Trace);


    // let my_proposition_a = LogicProposition::new("A & B | C ! D + E");
    let my_proposition_b = MathProposition::new("A & B | C ! D + E");

    // println!("{:?}", my_proposition_a.unwrap());
    println!("{:?}", my_proposition_b);
}
