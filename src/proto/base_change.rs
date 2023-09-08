/// This function reads a string with the form: `[A-Za-z\d]+(\.[A-Za-z\d]+)?`.
/// 
/// Where:
/// - `[A-Za-z\d]+`  ->  Any number of letters or numbers (at least one)
/// - `(\.[A-Za-z\d]+)?`  ->  Optional: A dot followed by dot and any number of letters or numbers (at least one)
/// 
/// And returns a number in the base specified.
/// 
/// The considered alphabet is: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`.
/// So the maximum base is 62. Even though the function can handle any base up to 256, the alphabet is limited to 62 characters.
/// 
/// # Arguments:
/// - src: `&str` - The string to convert to a number
/// - src_base: [`u8`] - The base of the source number 
/// - new_base: [`u8`] - The base of the new number
/// 
/// # Returns:
/// - [`f64`] - The number in the new base
pub fn str_to_num_from_base(src: &str, src_base: u8, new_base: u8) -> Result<String, String> {
    if src_base < 2 || src_base > 62 || new_base < 2 || new_base > 62 {return Err("Base must be between 2 and 62".to_string())}  // * Check if the base is between 2 and 62
    // src.chars().for_each(|c| println!("{} -> {:?}", c, digit_value(c)));  // Print the value of all the chars in the string

    /// This function converts any character to its numerical value
    /// 
    /// The function can interpret any character in the Alphabet as a digit
    /// - `0..9` ->  0.. 9
    /// - `A..Z` -> 10..35
    /// - `a..z` -> 36..61
    fn digit_value(c: char) -> Result<u8, String> {  // Function to convert a digit character to its numerical value
        if c.is_ascii_digit() {Ok(c as u8 - '0' as u8)  // If the character is a digit, return it's value
        } else if c.is_ascii_uppercase() {Ok(c as u8 - 'A' as u8 + 10)  // If c is uppercase
        } else if c.is_ascii_lowercase() {Ok(c as u8 - 'a' as u8 + 36)  // If c is lowercase
        } else if c == '.' {Ok(255)  // 255 is the value of the dot
        // * Here we could add more characters to the alphabet (to support bases up to 256)
        } else {return Err(format!("Invalid char in src string: {}", c))}  // If c is not a valid character, return an error
    }

    // * Convert the src string to base 10 float (integer + decimal)
    let mut parts = src.split('.');  // Split the string in two parts: integer and decimal
    let integer = parts.next().unwrap_or("");  // Get the integer part
    let decimal = parts.next().unwrap_or("");  // Get the decimal part
    if parts.next().is_some() {return Err("Invalid string: more than one dot".to_string())}  // Return an error

    // If any element of the integer part is greater than the base, return an error
    if integer.chars().any(|c| digit_value(c).unwrap_or(0) >= src_base) {return Err("Invalid char on integer part".to_string())}
    if decimal.chars().any(|c| digit_value(c).unwrap_or(0) >= src_base) {return Err("Invalid char on decimal part".to_string())}

    println!("{}.{}", integer, if decimal.is_empty() {"0"} else {decimal});  // Print the number

    // * Integer part
    let mut n = 0.0;  // Create a variable to store the integer part of the number
    for (i, c) in integer.chars().rev().enumerate() {  // For each character in the string
        let digit = digit_value(c)?;  // Get the value of the digit
        let digit = digit as f64 * (src_base as f64).powi(i as i32);  // Convert the digit to base 10
        n += digit;  // Add the digit to the number
    }
    println!("  N -> {}", n);  // From Natural part to base 10

    // * Decimal part
    let mut d = 0.0;  // Create a variable to store the decimal part of the number
    for (i, c) in decimal.chars().enumerate() {  // For each character in the string
        let digit = digit_value(c)?;  // Get the value of the digit
        let digit = digit as f64 * (src_base as f64).powi(-(i as i32 + 1));  // Convert the digit to base 10
        d += digit;  // Add the digit to the number
    }
    println!("  D -> {:}", d);  // From Decimal part to base 10

    // * Convert the number to the new base
    let mut n = n as u64;  // Convert the number to an integer ()
    let mut new_number = String::new();  // Create a string to store the new number
    
    // * Integer part (new base)
    while n > 0 {  // While the number is greater than 0
        let digit = n % new_base as u64;  // Get the remainder of the division
        let digit = if digit < 10 {digit as u8 + '0' as u8} else {digit as u8 - 10 + 'A' as u8};  // Convert the digit to a char
        new_number.insert(0, digit as char);  // Insert the digit at the beginning of the string
        n /= new_base as u64;  // Divide the number by the base
    }
    new_number.push('.');  // Add a dot to the string

    // * Decimal part (new base) (Also define the number of decimals to calculate)
    for _ in 0..10 { // This define the number of decimals to calculate (more decimals equals less exactitude)
        let digit = (d * new_base as f64) as u8;  // Get the remainder of the division
        let digit = if digit < 10 {digit as u8 + '0' as u8} else {digit as u8 - 10 + 'A' as u8};  // Convert the digit to a char
        new_number.push(digit as char);  // Insert the digit at the beginning of the string
        d = (d * new_base as f64) % 1.0;  // Get the remainder of the division (the new decimal part) and store it in d
    }

    Ok(new_number)  // Return the new number in the new base as a string

}
