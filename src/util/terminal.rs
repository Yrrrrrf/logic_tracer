// ? Command Line Interface (CLI) related scripts  -------------------------------------------------    -------------------------------

/// Return a string with the color set
/// 
/// ### Arguments:
/// - string [`str`] - The string to set the color on
/// - fg [`str`] - The color to set the string to
/// 
/// ### Returns:
/// - [`String`] - The string with the foreground color set
pub fn set_fg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        // RGB (Red, Green, Blue)
        "r" | "red"   => format!("\x1b[31m{}\x1b[0m", string),  // Red
        "g" | "green" => format!("\x1b[32m{}\x1b[0m", string),  // Green
        "b" | "blue" => format!("\x1b[34m{}\x1b[0m", string),  // Blue
        // CMY (Cyan, Magenta, Yellow)
        "c" | "cyan" => format!("\x1b[36m{}\x1b[0m", string),  // Cyan
        "m" | "magenta" => format!("\x1b[35m{}\x1b[0m", string),  // Magenta
        "y" | "yellow" => format!("\x1b[33m{}\x1b[0m", string),  // Yellow
        _ => string.to_string(),  // White (default)
    }
}


/// Return a string with the foreground color set
/// 
/// ### Arguments:
/// - string [`str`] - The string to set the color on
/// - fg [`str`] - The color to set the string to
/// 
/// ### Returns:
/// - [`String`] - The string with the foreground color set
pub fn set_bg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        // RGB (Red, Green, Blue)
        "r" | "red"   => format!("\x1b[41m{}\x1b[0m", string),  // Red
        "g" | "green" => format!("\x1b[42m{}\x1b[0m", string),  // Green
        "b" | "blue" => format!("\x1b[44m{}\x1b[0m", string),  // Blue
        // CMY (Cyan, Magenta, Yellow)
        "c" | "cyan" => format!("\x1b[46m{}\x1b[0m", string),  // Cyan
        "m" | "magenta" => format!("\x1b[45m{}\x1b[0m", string),  // Magenta
        "y" | "yellow" => format!("\x1b[43m{}\x1b[0m", string),  // Yellow
        _ => string.to_string(),  // White (default)
    }
}



// ? Ask for input  ---------------------------------------------------------------------------------------------------------------


use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


/// Ask for input from the console
/// 
/// ### Parameters:
/// - `T: std::str::FromStr` - The type of the input
/// 
/// ### Returns:
/// - `T` - The input
pub (crate) fn ask<T: std::str::FromStr>() -> T where <T as FromStr>::Err: std::fmt::Debug {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<T>().unwrap();
}


/// Print the type of a variable
/// 
/// ### Parameters:
/// - `_: &T` - The variable to print the type of
pub (crate) fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


/// Return the type of a variable as a string
/// 
/// ### Parameters:
/// - `_: &T` - The variable to get the type of
/// 
/// ### Returns:
/// - [`String`] - The type of the variable
pub (crate) fn get_type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}
