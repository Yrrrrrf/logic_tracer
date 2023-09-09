#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// ? Import modules -----------------------------------------------------------------------------------------------------------
mod util;
use crate::util::*;  // terminal mod & print_app_data() fn

// APP COMPONENTS (MODULES)
mod components;
use components::*;

// ? Proto: Dev mode
mod proto;
use logic_tracer::ast::Ast;
use proto::{*, base_change::str_to_num_from_base};
use util::terminal::set_fg;


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


    vec![  // (infix, prefix, postfix)
        ("eekke", "fmfdkodfmo", ""),
        ("", "", "")
    ].iter().for_each(|(infix, prefix, postfix)|
    {
        let mut ast = Ast::new(infix);
        println!("{}\n\tPrefix: {}\n\tPostfix: {}", set_fg(&format!("Infix: {}",infix), "g"), prefix, postfix);
        println!("\tPrefix: {}", set_fg(&ast.get_prefix_string(), "b"));
        println!("\tPostfix: {}", set_fg(&ast.get_postfix_string(), "b"));
    });




}

