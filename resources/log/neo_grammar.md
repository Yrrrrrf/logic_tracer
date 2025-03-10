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
    Arithmetic ::= self Variable '=' Range(Natural) { expr };
        Sum ::= 'sum' | 'Σ';
        Product ::= 'prod' | 'Π';

    Algebraic ::= ('abs' | 'floor' | 'ceil' | 'gcd' | 'lcm') '(' Expression ')';

    Geometric ::= TrigonometricFunction | InverseTrigonometricFunction | HyperbolicFunction | InverseHyperbolicFunction;
        TrigonometricFunction ::= ('sin' | 'cos' | 'tan' | 'cot' | 'sec' | 'csc') '(' Expression ')';
        InverseTrigonometricFunction ::= ('arcsin' | 'arccos' | 'arctan' | 'arccot' | 'arcsec' | 'arccsc') '(' Expression ')';
        HyperbolicFunction ::= ('sinh' | 'cosh' | 'tanh' | 'coth' | 'sech' | 'csch') '(' Expression ')';
        InverseHyperbolicFunction ::= ('arsinh' | 'arcosh' | 'artanh' | 'arcoth' | 'arsech' | 'arcsch') '(' Expression ')';

    CalculusFunction ::= Integral | Derivative | Limit;
        Integral ::= 'int' Range;
        Derivative ::= 'diff' '(' Expression ',' Variable ')';
        Limit ::= 'lim' Range;

Range(Number)::= Number '..' Number (..Number)?;

        <!-- todo: Add some new calculus operations (e.g. Gradient, Divergence, Curl, Laplacian, etc.)... -->

    <!-- todo: Add some new functions (e.g. Statistical, etc.)...
<!--  -->
<!--  -->
<!--  -->
    Logical ::= Proposition | Predicate | Quantifier;  // ? This is more like a placeholder for logical operations...
        Proposition ::= 'prop' '(' Expression ')';
        Predicate ::= 'pred' '(' Expression ')';
        Quantifier ::= 'forall' '(' Variable ',' Expression ')' | 'exists' '(' Variable ',' Expression ')';
<!--  -->
<!--  -->
<!--  -->



```