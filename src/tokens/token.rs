#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot,
    Semicolon, Colon,
    Pipe, Or,
    Ampersand, And,

    Plus, Minus, Star, Slash, Percent,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier(String), String(String), Number(f64),

    Var, Const, Type, Func, Return,
    Class, Interface, Override, Implements,
    Priv, Prot, Pub, Static, Enum, Match,
    Case, If, Else, For, In, While, Export,
    Import, Annotation, Try, Catch, Finally,
    Namespace, New, Typeof,

    True, False, Null,

    EndOfFile,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}