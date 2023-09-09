/// Utils Module
/// 
/// This module contains utility functions that are used throughout the program.
pub mod terminal;


/// Reads the Cargo.toml file and prints the app data
/// 
/// This is also the same as the package data.
pub fn print_app_data() {
    // print!("{}[2J{}[1;1H", 27 as char, 27 as char);  // Clear the terminal
    let mut file = std::fs::File::open("Cargo.toml")  // Open the file (read only)
        .unwrap_or_else(|e| {println!("{}: {}", terminal::set_fg("Error reading file", "r"), e); std::process::exit(1);});
    let mut content = String::new();  // create a new string
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();  // read the file and put the contents in the string

    for line in content.lines() {  // iterate over the lines in the string
        match line {  // match the line
            line if line.starts_with("name = ")    => {print!("{}", terminal::set_fg(&line[7..].to_string().replace('\"', "").to_uppercase(), "g"));}
            line if line.starts_with("version = ") => {println!(" {}", terminal::set_fg(&format!("v{}", line[10..].to_string().replace('\"', "")), "b"));}
            line if line.starts_with("authors = ") => {println!("Authors: {}", line[10..].to_string().replace('\"', "")); break;}
            _ => {}  // do nothing
        }
    }
    println!();  // Print a newline (for spacing)
}
