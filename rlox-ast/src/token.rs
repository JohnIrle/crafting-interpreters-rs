use crate::object::Object;
use crate::token_type::TokenType;
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn literal(&self) -> Option<&Object> {
        // TODO: handle unwrap.
        self.literal.as_ref()
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal.as_ref().unwrap())
    }
}

