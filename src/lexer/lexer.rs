use crate::tokens::token::{Token, TokenType};

pub struct Lexer {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EndOfFile,
            lexeme: "".to_string(),
            line: self.line,
            column: self.column,
        });

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            ':' => self.add_token(TokenType::Colon),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '%' => self.add_token(TokenType::Percent),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenType::And);
                } else {
                    self.add_token(TokenType::Ampersand);
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenType::Or);
                } else {
                    self.add_token(TokenType::Pipe);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = match text.as_str() {
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "type" => TokenType::Type,
            "func" => TokenType::Func,
            "return" => TokenType::Return,
            "class" => TokenType::Class,
            "interface" => TokenType::Interface,
            "override" => TokenType::Override,
            "implements" => TokenType::Implements,
            "priv" => TokenType::Priv,
            "prot" => TokenType::Prot,
            "pub" => TokenType::Pub,
            "static" => TokenType::Static,
            "enum" => TokenType::Enum,
            "match" => TokenType::Match,
            "case" => TokenType::Case,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "while" => TokenType::While,
            "export" => TokenType::Export,
            "import" => TokenType::Import,
            "annotation" => TokenType::Annotation,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "namespace" => TokenType::Namespace,
            "new" => TokenType::New,
            "typeof" => TokenType::Typeof,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier(text),
        }.clone();

        self.add_token(token_type);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let text = self.source[self.start..self.current].to_string();
        let value = text.parse::<f64>().unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                panic!("Unterminated string.");
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string.");
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(value));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line, self.column));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors().is_empty()
    }

    pub fn print_errors(&self) {
        for error in self.errors().iter() {
            println!("{}", error);
        }
    }

    pub fn print_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }
    }
}
