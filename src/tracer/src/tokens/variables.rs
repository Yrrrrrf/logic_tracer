// Handle all the Token types that can be a Variable in the grammar! (Constants, etc...)

#![allow(unused)]

use super::*;
use crate::impl_enum_token;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable;

pub trait VariableTrait: Token {
    // * Add properties and methods for the variable type...
}

/// Macro to define constants with their associated subjects and declare the subject traits.
///
/// # Parameters
///
/// - `$subject`: The subject to which the constants belong (e.g., Math, Physics).
/// - `$name`: The name of the constant struct (e.g., Tau, ElectronMass).
/// - `$str`: The string representation of the constant (e.g., "τ", "\\me").
///
/// # Example
///
/// ```rust
/// define_constants!(
///     Math (
///         Tau => "τ",
///         Pi => "π",
///         Phi => "φ",
///         EulerGamma => "γ",
///         Infinity => "∞",
///         E => "e",
///     ),
///     Physics (
///         SpeedOfLight => "\\c",
///         PlanckConstant => "\\h",
///         Gravitational => "\\G",
///         MuonMass => "\\mμ",
///         ElectronMass => "\\me",
///         ProtonMass => "\\mp"
///     )
/// );
/// ```
// * Add this behavior to the macro (+ '\\const_name')
/// Macro to define constants and their associated traits.
macro_rules! define_constants {
    ($token_type:ident; $trait_name:ident;
        $($name:ident (
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
    };
}

define_constants!(Variable; VariableTrait;

    // * ALPHABETS (on lowercase and uppercase)
    Alphabet (
        A => ("a"),
        B => ("b"),
        C => ("c"),
        D => ("d"),
        E => ("e"),
        F => ("f"),
        G => ("g"),
        H => ("h"),
        I => ("i"),
        J => ("j"),
        K => ("k"),
        L => ("l"),
        M => ("m"),
        N => ("n"),
        O => ("o"),
        P => ("p"),
        Q => ("q"),
        R => ("r"),
        S => ("s"),
        T => ("t"),
        U => ("u"),
        V => ("v"),
        W => ("w"),
        X => ("x"),
        Y => ("y"),
        Z => ("z"),
    ),
    AlphaUpper (
        A => ("A"),
        B => ("B"),
        C => ("C"),
        D => ("D"),
        E => ("E"),
        F => ("F"),
        G => ("G"),
        H => ("H"),
        I => ("I"),
        J => ("J"),
        K => ("K"),
        L => ("L"),
        M => ("M"),
        N => ("N"),
        O => ("O"),
        P => ("P"),
        Q => ("Q"),
        R => ("R"),
        S => ("S"),
        T => ("T"),
        U => ("U"),
        V => ("V"),
        W => ("W"),
        X => ("X"),
        Y => ("Y"),
        Z => ("Z"),
    ),
    GreekAlpha (
        Alpha => ("α"),
        Beta => ("β"),
        Gamma => ("γ"),
        Delta => ("δ"),
        Epsilon => ("ε"),
        Zeta => ("ζ"),
        Eta => ("η"),
        Theta => ("θ"),
        Iota => ("ι"),
        Kappa => ("κ"),
        Lambda => ("λ"),
        Mu => ("μ"),
        Nu => ("ν"),
        Xi => ("ξ"),
        Omicron => ("ο"),
        Pi => ("π"),
        Rho => ("ρ"),
        Sigma => ("σ"),
        Tau => ("τ"),
        Upsilon => ("υ"),
        Phi => ("φ"),
        Chi => ("χ"),
        Psi => ("ψ"),
        Omega => ("ω"),
    ),
    GreekUpperAlpha (
        Alpha => ("Α"),
        Beta => ("Β"),
        Gamma => ("Γ"),
        Delta => ("Δ"),
        Epsilon => ("Ε"),
        Zeta => ("Ζ"),
        Eta => ("Η"),
        Theta => ("Θ"),
        Iota => ("Ι"),
        Kappa => ("Κ"),
        Lambda => ("Λ"),
        Mu => ("Μ"),
        Nu => ("Ν"),
        Xi => ("Ξ"),
        Omicron => ("Ο"),
        Pi => ("Π"),
        Rho => ("Ρ"),
        Sigma => ("Σ"),
        Tau => ("Τ"),
        Upsilon => ("Υ"),
        Phi => ("Φ"),
        Chi => ("Χ"),
        Psi => ("Ψ"),
        Omega => ("Ω"),
    ),

    // * CONSTANTS
    MathConst (
        Tau => ("τ", "\\tau"),
        Pi => ("π", "\\pi"),
        Phi => ("φ"),
        EulerGamma => ("γ"),
        Infinity => ("∞"),
        E => ("e"),
    ),
    PhysicConst (
        SpeedOfLight => ("\\c"),
        PlanckConstant => ("\\h"),
        Gravitational => ("\\G"),
        MuonMass => ("\\mμ"),
        ElectronMass => ("\\me"),
        ProtonMass => ("\\mp"),
    ),

);
