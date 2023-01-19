# Quantum Grammar

This document describes the grammar of the Quantum language.

```ebnf
program = { statement } ;

statement =
    | "var" identifier ":" type "=" expression ";"
    | "const" identifier ":" type "=" expression ";"
    | "type" identifier "=" type { "|" type } ";"
    | "if" "(" expression ")" statement [ "else" statement ]
    | "{" { statement } "}"
    | expression ";"
    | "return" expression ";"
    | "break" ";"
    | "continue" ";"
    | "while" "(" expression ")" statement
    | "for" "(" [ statement ] ";" [ expression ] ";" [ statement ] ")" statement ;

expression =
    | identifier
    | literal
    | "(" expression ")"
    | expression "+" expression
    | expression "-" expression
    | expression "*" expression
    | expression "/" expression
    | expression "%" expression
    | expression "^" expression
    | expression "==" expression
    | expression "!=" expression
    | expression "<" expression
    | expression ">" expression
    | expression "<=" expression
    | expression ">=" expression
    | expression "&&" expression
    | expression "||" expression
    | expression "?" expression ":" expression
    | expression "[" expression "]"
    | expression "[" expression ":" expression "]"
    | expression "[" expression ":" expression ":" expression "]"
    | expression "." identifier
    | expression "(" [ expression { "," expression } ] ")"
    | expression "++"
    | expression "--"
    | "++" expression
    | "--" expression
    | "-" expression
    | "+" expression
    | "!" expression
    | "~" expression
    | "new" type "(" [ expression { "," expression } ] ")"
    | "new" type "[" expression "]" ;
    
literal = 
    | integer
    | float
    | string
    | char
    | bool
    | null
    | array
    | map ;

integer = [0-9]+ ;
float = [0-9]+ "." [0-9]+ ;
string = "\"" [^"]* "\"" ;
char = "'" [^'] "'" ;
bool = "true" | "false" ;
null = "null" ;
array = "[" [ expression { "," expression } ] "]" ;
map = "{" [ expression ":" expression { "," expression ":" expression } ] "}" ;

identifier = [a-zA-Z_][a-zA-Z0-9_]* ;

type =
    | Int
    | Float
    | String
    | Char
    | Bool
    | Array "[" type "]"
    | Map "[" type "," type "]" ;


```