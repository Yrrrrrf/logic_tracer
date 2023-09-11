#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// ? Import modules -----------------------------------------------------------------------------------------------------------
mod util;
use crate::{util::*, components::proposition::Proposition};  // terminal mod & print_app_data() fn

// APP COMPONENTS (MODULES)
mod components;
use components::*;

// ? Proto: Dev mode
mod proto;
use proto::{*, base_change::*};
use util::*;


// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    print_app_data();  // Print the app data
    run();  // Run the app
}


/// Run the app
pub fn run() {

    // vec![
    //     (10, 16, "450.5", "1C2.8"),
    //     (10, 16, "8.5", "8.8"),
    //     (8, 10, "450.5", "296.625"),
    //     (8, 10, "7.5", "7.625"),
    //     (2, 10, "1010.1", "10.5"),
    //     (2, 10, "1010", "10"),
    //     (20, 6, "AA.21", "550.03405012"),
    //     (10, 16, "2197.42", "895.6B851EB8"),
    //     (16, 10, "9E.D", "158.8125"),
    // ].iter().for_each(|(src_base, new_base, src, result)|
    //     println!("{} b{:_>2} -> {} b{:_>2}\t{}", src, src_base, 
    //         str_to_num_from_base(src, *src_base, *new_base).unwrap(), new_base,
    //         terminal::set_fg(result, if str_to_num_from_base(src, *src_base, *new_base).unwrap() == result.to_string() {"g"} else {"r"})
    //     )
    // );


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


    use logic_tracer::grammar::*;
    use logic_tracer::grammar::GrammarToken::*;
    use logic_tracer::operators::*;

    let prop = Proposition::new("A + b + c + d + e + f + g + h + i + j + k");
    println!("{}", prop);


}
