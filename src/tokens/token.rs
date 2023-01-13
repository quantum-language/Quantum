#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot,
    Semicolon, Colon,

    Plus, Minus, Star, Slash, Percent,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    Var, Const, Type, Func, Return,
    Class, Interface, Override, Implements,
    Priv, Prot, Pub, Static, Enum, Match,
    Case, If, Else, For, In, While, Export,
    Import, Annotation, Try, Catch, Finally,
    Namespace, New,

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