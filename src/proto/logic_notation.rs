// use dev_utils::;
use regex::Regex;

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


pub fn infix_to_postfix(expression: &str) -> String {
    // Define the precedence of operators
    let precedence = |operator: char| -> i32 {
        match operator {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => 0, // Default precedence for other characters (operands)
        }
    };

    let mut output = String::new(); // Output string for postfix expression
    let mut stack = Vec::new(); // Stack for operators

    for token in expression.chars() {
        match token {
            // If the token is an operand, append it to the output
            // 'A'..='Z' | 'a'..='z' => output.push(token),
            '0'..='9' => output.push(token),

            // If the token is an operator
            '+' | '-' | '*' | '/' => {
                // Pop operators from the stack and append them to the output
                // until an operator with lower precedence is encountered or the stack is empty
                while let Some(top) = stack.last() {
                    if precedence(token) <= precedence(*top) {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                // Push the current operator onto the stack
                stack.push(token);
            }

            // If the token is an opening parenthesis, push it onto the stack
            '(' => stack.push(token),

            // If the token is a closing parenthesis, pop operators from the stack
            // and append them to the output until an opening parenthesis is encountered
            ')' => {
                while let Some(top) = stack.last() {
                    if *top == '(' {
                        stack.pop(); // Pop and discard the opening parenthesis
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
            }

            _ => {
                // Ignore spaces and other characters
            }
        }
    }

    // Pop any remaining operators from the stack and append them to the output
    while let Some(top) = stack.pop() {
        output.push(top);
    }

    output
}



pub fn evaluate_postfix(expression: &str) -> i32 {
    let mut stack = Vec::new();

    for token in expression.chars() {
        match token {
            // If the token is an operand, push it onto the stack
            '0'..='9' => {
                let operand = token.to_digit(10).unwrap() as i32;
                stack.push(operand);
            }

            // If the token is an operator, pop the top two operands from the stack,
            // apply the operator, and push the result back onto the stack
            '+' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = a + b;
                    stack.push(result);
                }
            }

            '-' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = a - b;
                    stack.push(result);
                }
            }

            '*' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = a * b;
                    stack.push(result);
                }
            }

            '/' => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b != 0 {
                        let result = a / b;
                        stack.push(result);
                    } else {
                        panic!("Division by zero!");
                    }
                }
            }

            _ => {
                // Ignore other characters (e.g., spaces)
            }
        }
    }

    // The result should be the only item left on the stack
    if let Some(result) = stack.pop() {
        result
    } else {
        panic!("Invalid postfix expression");
    }
}



























fn test_infix_to_postfix(expression: &str) -> String {
    let mut output = String::new();
    let mut stack = Vec::new();

    let precedence = |op: char| -> i32 {
        match op {
            '&' | '|' => 1,
            '=' => 2,
            _ => 0,
        }
    };

    for token in expression.chars() {
        match token {
            'A'..='Z' | 'a'..='z' => {
                output.push(token);
            }
            '&' | '|' | '=' => {
                while let Some(top) = stack.last() {
                    if *top == '(' || precedence(*top) < precedence(token) {
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
            '(' => {
                stack.push(token);
            }
            ')' => {
                while let Some(top) = stack.last() {
                    if *top == '(' {
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.pop(); // Pop the '('
            }
            _ => {}
        }
    }

    while let Some(op) = stack.pop() {
        output.push(op);
    }

    output
}


// pub fn run_prototype() {
//     // let infix_expression = "(A&B)&(C|D)";
//     let infix_expression = "A&(B|C)|D";
//     let postfix_expression = infix_to_postfix(infix_expression);
//     println!("Infix Expression: {}", infix_expression);
//     println!("Postfix Expression: {}", postfix_expression);
// }


pub fn test_var_regex() {
    // * So, if the pattern matches, the sequence contains at least one term
    // ^ TEST VARIABLES ----------------------------------------------------------------------------------------------------------
    let variables: Vec<&str> = vec![
        "A",  // simple term
        "A_1",  // term with index
        "AB",  // term with multiple characters
        "AB_1",  // term with multiple characters and index
        "A_1B_2",  // term with multiple characters and index
        "2A",  // term with number in front
        "2A_1",  // term with number in front and index
        "3421413",
        "32",
        "A23B_2",
        "2363",
        "5 + 232a_2vbq2",
        "a23",
        ""
    ];

    // * This one also works but also accepts empty sequences & sequences with only numbers
    // let variable_pattern = match Regex::new("\\d*([A-Za-z](_\\d+)?)?") {
    // let variable_pattern = match Regex::new("\\d*[A-Za-z](_\\d+)?") {  // Variables with a coheficient (a constant in front)
    let variable_pattern = match Regex::new("[A-Za-z](_\\d+)?") {  // Variables only
        Ok(pattern) => pattern,  // if the pattern is valid
        // Err(err) => {println!("{} {}", terminal::set_fg("Error: ", 'r'), err); return;}
        Err(err) => {println!("{}", "Error: "); return;}
    };
        // \d * get any number in front of the term (0 or more)
        // ([A-Za-z](_\d+)?)*
            // [A-Za-z] get any character (upper or lower)
            // (_[0-9]+)? get any number after the character (it means that is a subterm)
                // _ get this character
                // [0-9]+ get any number after the character (1 or more)
                // ? 0 or 1 times (could be a subterm or not)
            // ? 0 or 1 times (could be more than just a number)

            variables.iter().for_each(|term| println!("{:>16} -> {}", match variable_pattern.is_match(term) {
        // true => terminal::set_fg("valid" ,"g"),  // * These means that these sequences contains at least one term
        // false => terminal::set_fg("invalid" ,"r")  // The sequence is not a term
        true => "valid",  // * These means that these sequences contains at least one term
        false => "invalid"  // The sequence is not a term
    }, term));

    println!("All the extracted sequences (variables) of each term:");
    variables.iter().for_each(|term| {
        println!("{:>16} -> {:?}", term, 
            variable_pattern.captures_iter(term).map(|capture| capture[0].to_string()).collect::<Vec<String>>());
    });    
}


pub fn test_logic_operators_regex() {
    // test the match of the logic operators
    println!("Logic Operators Test:");
    // ^ TEST LOGIC OPERATORS ------------------------------------------------------------------------------------------------
    let logic_operators: Vec<&str> = vec![
        "+",  // simple operator
        "*",  // simple operator
        ];
            
}


pub fn test_logic() {
    // ^ TEST LOGIC EXPRESSIONS -----------------------------------------------------------------------------------------------

    // vec![
    //     "A+B",
    //     "(X+Y)*Z",
    //     "A+(B*C)",
    //     "(A+B)*(C+D)",
    //     "A*(B+(C*D))",
    //     "(A*B)+(C*D)",
    // ].iter().for_each(|expr| println!("{:>16} -> {}", match logic_expression_pattern.is_match(expr) {
    //     true => terminal::set_fg("valid" ,"g"),
    //     false => terminal::set_fg("invalid" ,"r")
    // }, expr));

}


pub fn test_shunting_yard_algorithm() {
    // https://www.youtube.com/watch?v=8QrlSSgRC3s
    // https://www.youtube.com/watch?v=Wz85Hiwi5MY
    // https://www.youtube.com/watch?v=A-SSrZUHYSk
    // ^ TEST SHUNTING YARD ---------------------------------------------------------------------------------------------------
    
}


