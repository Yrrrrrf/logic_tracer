
// /// This module contains the Operator trait and the Operator enums.


use super::neo_grammar::*;
use crate::*;


pub trait OperatorTrait: Token {
    // Create an attr for any Operator
    // const NEGATOR: Self;
}



macro_rules! impl_operator_token {
    ($token_type:ident; $trait_name:ident;
        $($name:ident {  // Enum for the types
            $(
                $variant:ident => ($($str:expr),+)
                $(,)?
            )+
        }),+
        $(,)?
    ) => {
        impl_token_type!($token_type; $trait_name; $($name),+);
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub enum $name { $($variant,)+ }

            impl $trait_name for $name {
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
    };

}


impl_operator_token!(Operator; OperatorTrait;
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
