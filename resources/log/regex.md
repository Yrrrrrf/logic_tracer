# Regex Explanation

## Terms matching

This regular expression **handles all possible TERMS in the expression**. It consists of several parts, each designed to capture a specific element of the expression.
```rust
"(!|-)?(\d+(\.\d+)?)?[a-zA-Z](\_\d+)?"
```

### Components

- Negator: `(!|-)?`
    - `!|-`: Matches the negation symbol ! or subtraction operator -. This is not a negator, but it's included in the negator part because it's a valid character in the expression.
    - `?`: Makes the negation symbol optional. This means the expression may or may not start with a !.
- Coefficient: `(\d+(\.\d+)?)?`
    - `\d+`: Matches one or more digits. This part is for integer numbers.
    - `(\.\d+)?`: Captures the decimal part of a number. It's optional.
    - `\.`: Matches the decimal point.
    - `\d+`: Matches one or more digits after the decimal point.
The entire coefficient part (\d+(\.\d+)?) is made optional by the trailing ?, meaning the expression can start with a number, but it's not necessary.
- Variable: `[a-zA-Z]`
    - `[a-zA-Z]`: Matches any single uppercase or lowercase alphabet character. This represents the variable in the expression.
- Sub-variable: `(\_\d+)?`
    - `\_`: Matches the underscore character _.
    - `\d+`: Matches one or more digits following the underscore. This is the numerical part of the sub-variable.
The entire sub-variable part `(\_\d+)?` is optional, signifying that the expression can include a sub-variable, but it's not required.

## Operators matching
This regular expression **handles all possible OPERATORS in the expression**. It consists of several parts, each designed to capture a specific element of the expression.
```rust
"(\+|\-|\*|\/|\^|\%|\=|\>\=|\<\=|\>|\<|\&\&|\|\|!)"
```

### Components

- Math:
    - `\+`: Addition operator.
    - `\-`: Subtraction operator.
    - `\*`: Multiplication operator.
    - `\/`: Division operator.
    - `\^`: Exponentiation operator.
    - `\%`: Modulus operator.
- Relational:
    - `\=`: Equals operator.
    - `\>\=`: Greater than or equal to operator.
    - `\<\=`: Less than or equal to operator.
    - `\>`: Greater than operator.
    - `\<`: Less than operator.
- Logical:
    - `\&\&`: Logical AND operator.
    - `\|\|`: Logical OR operator.
    - `!`: Logical NOT operator.
