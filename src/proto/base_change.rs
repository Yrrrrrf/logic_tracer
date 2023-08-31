pub fn change_from_base(n: u64, n_base: u64, new_base: u64) -> String {
    let mut n = n;  // copy n
    let mut result = String::new();  // create an empty string
    while n > 0 {  // while n is greater than 0
        let rem = n % new_base;  // get the remainder
        n /= new_base;  // divide n by the new base
        // push the remainder to the result string (as a char)
        result.push("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(rem as usize).unwrap_or('?'))
    }
    result.chars().rev().collect::<String>()  // reverse the result and return it
}

