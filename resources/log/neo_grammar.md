# Neo Grammar (improve version of the proposition grammar)

This is an improved version of the proposition grammar.

The grammar is written in Extended Backus-Naur Form (EBNF).

The EBNF is a notation technique for context-free grammars, often used to describe the syntax of languages used in computing.


**TODO**:

`Chech to add some 'function' Function = "f(Letter*) = Expression" to the grammar.`

`Check how to add some graph that represents the grammar...`

<!-- * -> 0..=n -->
<!-- ? -> 0..=1 -->
<!-- + -> 1..=n -->


<!-- * NUMBER TYPES -->
$$
n \in \mathbb{N} \subset \mathbb{Z} \subset \mathbb{R} \subset \mathbb{C}
$$

<!-- * OPERATIONS for Logic Proposition -->
<!-- ^ LogicProp uses Number: NaturalNum + LogicOp -->
<!-- $$  -->
<!-- \sum \prod -->
<!-- $$ -->

## Grammar
```ebnf
Expression ::= Term (Operator Term)*;

Term ::= (Neg)? (Variable)* | AdvancedOperation | GroupedExpression;

GroupedExpression ::= '(' Expression ')' 
                    | '[' Expression ']'
                    | '{' Expression '}'
                    | '⌈' Expression '⌉'  /* Ceiling brackets */
                    | '⌊' Expression '⌋'  /* Floor brackets */
                    | '⟦' Expression '⟧'  /* Double square brackets */
                    | '⟨' Expression '⟩'  /* Angle brackets */
                    | '|' Expression '|'; /* Absolute value brackets */

Variable ::= Constant |  Letter ('_' Natural)?;
    Letter ::=;
        Greek ::=;
            lowercase ::= 'α' | 'β' | 'γ' | 'δ' | 'ε' | 'ζ' | 'η' | 'θ' | 'ι' | 'κ' | 'λ' | 'μ' | 'ν' | 'ξ' | 'ο' | 'π' | 'ρ' | 'σ' | 'τ' | 'υ' | 'φ' | 'χ' | 'ψ' | 'ω';
            uppercase ::= 'Α' | 'Β' | 'Γ' | 'Δ' | 'Ε' | 'Ζ' | 'Η' | 'Θ' | 'Ι' | 'Κ' | 'Λ' | 'Μ' | 'Ν' | 'Ξ' | 'Ο' | 'Π' | 'Ρ' | 'Σ' | 'Τ' | 'Υ' | 'Φ' | 'Χ' | 'Ψ' | 'Ω';
        English ::=;
            lowercase ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z';
            uppercase ::= 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
            Spanish ::= English + ('ñ' | 'Ñ');

    Constant ::=;
        Math ::=;
            Tau         ::= "τ" | "\tau" | "\better_circle_const";
            E           ::= "e" | "\e";  // Euler's number
            Pi          ::= "π" | "\pi";  // Archimedes' constant
            Phi         ::= "φ" | "\phi";  // Golden ratio
            EulerGamma  ::= "γ" | "\gamma";  // Euler's gamma constant
            Infinity    ::= "∞" | "\infty";
        Physics ::=;
            SpeedOfLight    ::= "\c";  // Speed of light
            PlanckConstant  ::= "\h";  // Planck constant
            Gravitational   ::= "\G";  // Gravitational constant
            MuonMass        ::= "\mμ";  // Muon rest mass
            ElectronMass ::= "\me";  // Electron rest mass
            ProtonMass ::= "\mp";     // Proton rest mass
        <!-- todo: Add some new constants... -->

Operator ::=;  <!-- * 100% impl (src/components/tokens/operators.rs) * -->
    Neg ::= Some(Operator);
    Logic ::=;
        And     ::= "\And"  | '&' | '*' | '⋅' | '∧';
        Or      ::= "\Or"   | '+' | '|';
        Not     ::= "\Not"  | '!' | '¬' | '\'';
        XOr     ::= "\XOr"  | '^' | '⊻' | '⨁';
        XNOr    ::= "\XNOr" | '⊙';
        NAnd    ::= "\NAnd" | '↑';
        NOr     ::= "\NOr"  | '↓';
    Comparison ::=;
        Equal ::= '=' | '==';
        NotEqual ::= '!=' | '≠';
        GreaterThan ::= '>' | '>';
        LessThan ::= '<' | '<';
        GreaterThanOrEqual ::= '>=' | '≥';
        LessThanOrEqual ::= '<=' | '≤';
    Arithmetic ::=;
        Add ::= '+';
        Subtract ::= '-';
        Multiply ::= '*' | '×' | '⋅' | '∙' | '•';
        Divide ::= '/' | '÷';
        Modulus ::= '%' | 'mod';
    <!-- todo: Add some new operators.(e.g. Bitwise, Shift, etc.)... -->


Number ::=;  <!-- * impl: digit, Natural, Integer, Real (src/components/tokens/numbers.rs) * -->
    digit ::= `0..=9`;
    Natural ::= digit+;
    Integer ::= Neg?Natural;
    Real ::= Integer '.' Natural;

    <!-- todo: Add the rest of the number types... -->
    <!-- Rational ::= Integer '/' Integer; -->
    Imaginary ::= Real 'i';
    Complex ::= Real '+' Imaginary;
    Irrational ::= Real;  // ? This is more like a placeholder for irrational numbers...

AdvancedOperation ::= '\\' Function '(' Expression ')';
    Function ::=;
        Arithmetic ::=;
            Sum ::= "\sum" Range;
            Product ::= "\prod" Range;
        Geometric ::=;
            Trigonometric ::=;
                Sine ::= "sin";
                Cosine ::= "cos";
                Tangent ::= "tan";
                Cotangent ::= "cot";
                Secant ::= "sec";
                Cosecant ::= "csc";
            InverseTrigonometric ::= "a" Trigonometric;
            Hyperbolic ::= Trigonometric "h";
            InverseHyperbolic ::= "a" Hyperbolic;
        Algebraic ::=;
            AbsoluteValue ::= "abs";
            Floor ::= "floor";
            Ceiling ::= "ceil";
            GreatestCommonDivisor ::= "gcd";
            LeastCommonMultiple ::= "lcm";
        Logarithm ::=;
            Natural ::= "ln";
            LogBase ::= "log";
        <!-- todo: Improve this part of the grammar... -->        
        <!-- todo: Add some new functions (e.g. Statistical, etc.)...



        <!-- todo: Add some new operations (e.g. Derivative, Integral, Limit, etc.)... -->
        Calculus ::= _ Range
            DIfferential and PartialDifferential ::=;
            Integral (Definite and Indefinite) ::=;
            <!-- todo: Add some new calculus operations (e.g. Gradient, Divergence, Curl, Laplacian, etc.)... -->
        <!-- todo: Add some new series (e.g. Product, etc.)...

Range ::= "{Number}_{Number}";

```