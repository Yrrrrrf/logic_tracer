//! This module provides a regex matching functionality.
#![allow(unused)]

use dev_utils::{
    print_app_data,
    log::rlog::RLog,
    console::format::set_fg,
};


/// This fn test the regex matching functionality.
///
/// The regex used is: `(\d+(\.\d+)?)?[a-zA-Z](\_\d+)?`
/// 
/// It matches any string in the form of a Compound (negator + number + variable + sub variable).
fn main() {
    print_app_data(file!());
    RLog::init_logger(log::LevelFilter::Debug);


    let test_vec = vec![  // A vector of test expressions to validate against the regex
        "!a",
        "3b",
        "4.5c_1", 
        "!7d_23", 
        "a_3", 
        "!b", 
        "-2.3c_2", 
        "!9d_43", 
        "e", 
        "!f_5",
        "6.7g", 
        "!h_6", 
        "8i", 
        "!3.1j_7", 
        "!4l_8", 
        "5.5m", 
        "n_9", 
        "!7o", 
        "2p_10", 
        "3x+!9b-6t_3t_4R_4",
        "@",
    ];
    // todo: Fix this to use the new implementation of grammar::Compound
    // test_vec.iter().for_each(|s| {
    //     let mut result_string = String::new();
    //     let mut last_end = 0;
    //     re.captures_iter(s).for_each(|cap| {  // Iterate over each capture
    //         let match_range = cap.get(0).unwrap().range();  // Get the range of the matched part
    //         result_string.push_str(&s[last_end..match_range.start]);  // Append the non-matched part
    //         result_string.push_str(&set_fg(&s[match_range.clone()], 'g'));  // Append the matched part in green
    //         last_end = match_range.end;  // Update the last_end variable
    //     });
    //     result_string.push_str(&s[last_end..]);  // Append any remaining part of the string after the last match
    //     println!("{s:16} -> {result_string}");
    //     re.captures_iter(s).for_each(|cap| println!("\t{}",set_fg(cap.get(0).unwrap().as_str(), 'g')));  // Print each capture in green
    // });

}
