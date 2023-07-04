#![allow(unused)]  // Allow unused code

// ? util methods for (any) rust code

use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)


// ? Ask for input  ---------------------------------------------------------------------------------------------------------------


fn ask_str() -> String {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input;
}


fn ask_int() -> i32 {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<i32>().unwrap();
}


/// 
fn ask_float() -> f32 {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<f32>().unwrap();
}


// ? Terminal related scripts  ----------------------------------------------------------------------------------------------------


/// Clear and cursor position
pub fn clear() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}


/// Return a string with the color set
/// 
/// ### Arguments:
/// - string [`str`] - The string to set the color on
/// - fg [`str`] - The color to set the string to
/// 
/// ### Returns:
pub(crate) fn set_fg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        "r" | "red"   => format!("\x1b[31m{}\x1b[0m", string),  // Red
        "g" | "green" => format!("\x1b[32m{}\x1b[0m", string),  // Green
        "b" | "blue" => format!("\x1b[34m{}\x1b[0m", string),  // Blue
        "c" | "cyan" => format!("\x1b[36m{}\x1b[0m", string),  // Cyan
        "m" | "magenta" => format!("\x1b[35m{}\x1b[0m", string),  // Magenta
        "y" | "yellow" => format!("\x1b[33m{}\x1b[0m", string),  // Yellow
         _ => format!("\x1b[37m{}\x1b[0m", string),  // White (default)
    }
}


/// Return a string with the foreground color set
/// 
/// ### Arguments:
/// - string [`str`] - The string to set the color on
/// - fg [`str`] - The color to set the string to
/// 
/// ### Returns:
/// - [`String`] - The string with the color set
pub(crate) fn set_bg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        "r" | "red"   => format!("\x1b[41m{}\x1b[0m", string),  // Red
        "g" | "green" => format!("\x1b[42m{}\x1b[0m", string),  // Green
        "b" | "blue" => format!("\x1b[44m{}\x1b[0m", string),  // Blue
        "c" | "cyan" => format!("\x1b[46m{}\x1b[0m", string),  // Cyan
        "m" | "magenta" => format!("\x1b[45m{}\x1b[0m", string),  // Magenta
        "y" | "yellow" => format!("\x1b[43m{}\x1b[0m", string),  // Yellow
         _ => format!("\x1b[47m{}\x1b[0m", string),  // White (default)
    }
}
