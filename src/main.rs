#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// ? Import modules -----------------------------------------------------------------------------------------------------------
// Extern crates
use log::{LevelFilter, warn, info, debug, trace, error};

// ? Own modules
mod util;  // some util functions to startup a rust application
use util::*;  // terminal mod & print_app_data() fn

mod components;
use components::*;

// ? Proto: Dev mode
mod proto;
use proto::{
    base_change::*,
    hashing::*,
    encryption::*,
    logic_notation::*,
};


// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    print_app_data();  // Read the Cargo.toml file and print the app data (name, version, authors)
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level

    run();  // Run the app
}


/// Run the app
pub fn run() {

    vec![
        (10, 2, "450.5", "111000010.1"),
        (10, 2, "8.5", "1000.1"),
        (10, 8, "450.5", "702.4"),
        (10, 8, "7.5", "7.4"),
        (10, 16, "450.5", "1C2.8"),
        (10, 16, "8.5", "8.8"),
        (8, 10, "450.5", "296.625"),
        (8, 10, "7.5", "7.625"),
        (2, 10, "1010.1", "10.5"),
        (2, 10, "1010", "10"),
        (20, 6, "AA.21", "550.03405012"),
        (10, 16, "2197.42", "895.6B851EB8"),
        (16, 10, "9E.D", "158.8125"),

    ].iter().for_each(|(src_base, new_base, src, result)|
        println!("{} b{:_>2} -> {} b{:_>2}\t{}", src, src_base, 
            str_to_num_from_base(src, *src_base, *new_base).unwrap(), new_base,
            terminal::set_fg(result, if str_to_num_from_base(src, *src_base, *new_base).unwrap() == result.to_string() {"g"} else {"r"})
        )
    );


    // // Sum 2 numbers in bin
    // let mut a = "10111010";
    // let mut b = "11011101";
    // // sum the equivalent numbers in base 10
    // let a_b10 = src_int_b10(a, 2);
    // let b_b10 = src_int_b10(b, 2);
    // let sum = a_b10 + b_b10;
    // println!("{} + {} = {}", a_b10, b_b10, sum);
    // // convert the sum to bin
    // let mut sum_bin = int_b10_dst(sum, 2);
    // println!("{} + {} = {}", a, b, sum_bin);


    // use logic_tracer::grammar::*;
    // use logic_tracer::grammar::GrammarToken::*;
    // use logic_tracer::operators::*;

    // let prop = Proposition::new("A + b + c + d + e + f + g + h + i + j + k");
    // println!("{}", prop);


    // regex = (\d\w)* *[\*\+\!\-] *(\d|\d\w)+
    // temp regex

    // 2b + 3a
    // 5 * 4
    // 1 + 2 * 3
    // a4 * (6 + 2) // bad
    // (32 + 21c) * 7c
    // 9 + 12 * 2
    // (5 + 3) * (6 - 2)
    // 10 * 3 + 7
    // 2 + 2 + 2 + 2 + 2
    // 4 * 5 * 6
    // use a bitwise operator
    // let mut a = 0b10111010;
    // let mut b = 0b11011101;
    // println!("{:b} + {:x} = {:o}", a, b, a ^ b);


    // let src = "10111010";

    // let h = hashing::hash(&src);
    // println!("hash({}) => {}", src, h);



}
