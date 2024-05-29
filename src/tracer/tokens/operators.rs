use super::*;

use crate::impl_enum_token;


pub trait OperatorTrait: Token {
    // * Add properties and methods for the operator type...
    // todo: Add all the stuff below...
        // todo: Add `precedence`, `associativity`, and `arity` properties for the operator type...
        // todo: Add `is_unary`, `is_binary`, and `is_nary` methods for the operator type... (use `arity`)
        // todo: Add `is_left_associative`, `is_right_associative`, and `is_non_associative` methods for the operator type... (use `associativity`)
}
#[derive(Debug, Clone, PartialEq)]
pub struct Operator;

pub trait OperatorNegator: OperatorTrait {
    const NEGATOR: Self;
}

/// Macro to implement operator tokens and their associated traits.
/// 
/// # Parameters
/// 
/// - `$token_type`: The type of the token (e.g., `Operator`).
/// - `$trait_name`: The name of the trait that all operator tokens will implement.
/// - `$name`: The name of the enum representing a specific operator type (e.g., `MathOp`).
/// - `$negator`: An optional parameter representing the negator variant for the operator type.
/// - `$variant`: The variants of the operator enum, mapping strings to enum variants.
/// 
/// # Example
/// 
/// ```rust
/// impl_operator_token!(Operator; OperatorTrait;
///     MathOp (Subtract;  // this will be the negator of the operator type
///         Add => ("+"),
///         Subtract => ("-"),
///         Multiply => ("*"),
///         Divide => ("/"),
///         Modulo => ("%"),
///         Power => ("^"),
///         Root => ("√"),
///         Factorial => ("!"),
///     ),
///     LogicOp (Not;  // this will be the negator of the operator type
///         And => ("&", "^", "∧", "+"),
///         Or => ("|", "||", "*"),
///         Not => ("!", "~", "¬"),
///         XOr => ("^", "⊻", "⨁"),
///         XNOr => ("⊙", "⊽"),
///         NAnd => ("↑"),
///         NOr => ("↓"),
///     ),
///     RelationalOp (NotEqual;
///         Equal => ("==", "="),
///         NotEqual => ("!=", "≠"),
///         LessThan => ("<"),
///         LessThanOrEqual => ("<=", "≤"),
///         GreaterThan => (">"),
///         GreaterThanOrEqual => (">=", "≥"),
///     ),
///     SomeOtherOp (;  // this will not have a negator (don't implement OperatorNegator for it)
///         SomeOp => ("some_op"),
///         AnotherOp => ("another_op"),
///    ),
/// );
/// ```
macro_rules! impl_operator_token {
    ($token_type:ident; $trait_name:ident;
        $($name:ident (
            $( $negator:ident )?;
            $(
                $variant:ident => ($($str:expr),+)
                $(,)?
            )+
        )),+
        $(,)?
    ) => {
        impl_enum_token!($token_type; $trait_name;
            $($name (
                $(
                    $variant => ($($str),+)
                )+
            )),+
        );

        $(
            $(
                impl OperatorNegator for $name {
                    const NEGATOR: Self = $name::$negator;
                }
            )?
        )+
    };
}

impl_operator_token!(Operator; OperatorTrait;
    MathOp (Subtract;
        Add => ("+"),
        Subtract => ("-"),
        Multiply => ("*"),
        Divide => ("/"),
        Modulo => ("%"),
        Power => ("^"),
        Root => ("√"),
        Factorial => ("!"),
    ),
    LogicOp (Not;
        And => ("&", "^", "∧", "+"),
        Or => ("|", "||", "*"),
        Not => ("!", "~", "¬"),
        XOr => ("^", "⊻", "⨁"),
        XNOr => ("⊙", "⊽"),
        NAnd => ("↑"),
        NOr => ("↓"),
    ),
    RelationalOp (;  // ^ without implementing OperatorNegator trait
        Equal => ("==", "="),
        NotEqual => ("!=", "≠"),
        LessThan => ("<"),
        LessThanOrEqual => ("<=", "≤"),
        GreaterThan => (">"),
        GreaterThanOrEqual => (">=", "≥"),
    ),
);
