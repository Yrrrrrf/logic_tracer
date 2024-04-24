
// /// This module contains the Operator trait and the Operator enums.


use super::neo_grammar::*;
use crate::*;


pub trait OperatorTrait: Token {
    // Create an attr for any Operator
    // const NEGATOR: Self;
}



macro_rules! impl_operator_token {
    ($token_type:ident;
        $($name:ident {  // Enum for the types
            $(
                $variant:ident => ($($str:expr),+)
                $(,)?
            )+
        }),+
        $(,)?
    ) => {
        declare_enum!(token $token_type; $($name),+);
        $(
            declare_enum!($name; $($variant),+);
            
            impl OperatorTrait for $name {
                // cont NEGATOR: Self = $negator
            }

            impl Token for $name {
                fn from_str<S: Into<String>>(string: S) -> Option<Self> {
                    match string.into().as_str() {
                        $($($str)|+ => Some($name::$variant),)+
                        _ => None
                    }
                }
            }

        )+

        impl $token_type {
            pub fn from<S: Into<String> + Copy>(string: S) -> Option<Self> {
                $(if let Some(value) = $name::from_str(string) {
                        return Some($token_type::$name(value));
                })+
                None  // Return None if none of the types match
            }
        }
    };

}


impl_operator_token!(Operator;
    MathOp {
        Add => ("+"),
        Subtract => ("-"),
        Multiply => ("*"),
        Divide => ("/"),
        Modulo => ("%"),
        Power => ("^"),
        Root => ("√"),
        Factorial => ("!"),
    },
    LogicOp {
        And => ("&", "^", "∧", "+"),
        Or => ("|", "||", "*"),
        Not => ("!", "~", "¬"),
        XOr => ("^", "⊻", "⨁"),
        XNOr => ("⊙", "⊽"),
        NAnd => ("↑"),
        NOr => ("↓"),
    },
    RelOp {
        Equal => ("="),
        NotEqual => ("!="),
        LessThan => ("<"),
        GreaterThan => (">"),
        LessThanOrEqual => ("≤", "<="),
        GreaterThanOrEqual => ("≥", ">="),
    }
);

