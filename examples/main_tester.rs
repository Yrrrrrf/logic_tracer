#![allow(unused)]


use log::{debug, LevelFilter};
use dev_utils::{
    print_app_data, 
    log::rlog::RLog
};
use logic_tracer::*;

fn main() {
    print_app_data(file!());
    // RLog::init_logger(LevelFilter::Info);
    RLog::init_logger(LevelFilter::Debug);
    // RLog::init_logger(LevelFilter::Trace);


    // * Number types
    let nums = [
        "123",
        "-456",
        "7.892",
        "-456.16",
        // "10.0i",
    ];
    nums.iter().for_each(|num| {
        debug!("{:?}", Number::from(*num).unwrap());
        // println!("Number: {:?}", Number::from(*num));

    });


    // * Operator types
    let ops = [
        "+",
        "*",
        "|",
        ">=",
        "!=",
        "<",
    ];
    ops.iter().for_each(|op| {
            debug!("{:?}", Operator::from(*op).unwrap());
            // println!("{:#?}", from_op(*op).unwrap());
        
    });


    // * PrimitiveToken types
    let tokens = [
        "A",
        "C",
        "D",
        "-23",
        "2",
        "3.0",
        "3.1223",
        
    ];

    tokens.iter().for_each(|token| {
        // println!("{:?}", PrimitiveToken::from(*token));
    });



}
