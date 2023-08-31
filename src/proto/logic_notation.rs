fn infix_to_postfix(expression: &str) -> String {
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

pub fn run_prototype() {
    // let infix_expression = "(A&B)&(C|D)";
    let infix_expression = "A&(B|C)|D";
    let postfix_expression = infix_to_postfix(infix_expression);
    println!("Infix Expression: {}", infix_expression);
    println!("Postfix Expression: {}", postfix_expression);
}
