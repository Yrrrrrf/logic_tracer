# Proposition Grammar

The grammar for the proposition language is defined below. The grammar is written in Extended Backus-Naur Form (EBNF).


proposition ::= expression ;

- expression ::= term { operator term } | advanced_operation ;
- term ::= factor | "NOT" factor ;
- factor ::= constant | "TRUE" | "FALSE" | variable | "(" expression ")" ;
- function ::= variable | expression ;

- Probably...
The factor true and false are the terminal symbols of the grammar. The variable is a placeholder for any variable name. The sequence is a special case of the expression that represents a summation. The operator is a placeholder for any of the logical operators. 