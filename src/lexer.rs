use regex::Regex;

// Constants
const PLUS: char = '+';
const MINUS: char = '-';
const MULTIPLY: char = '*';
const DIVIDE: char = '/';
const EQUALS: char = '=';
const END_OF_LINE: char = '\n';
const COMMENT: char = '#';

/// Represents the type of token
#[derive(Debug)]
pub(crate) enum TokenType {
    Comment(Regex),
    Variable(Regex),
    Number(Regex),
    Plus(Regex),
    Minus(Regex),
    Multiply(Regex),
    Divide(Regex),
    Equals(Regex),
    EndOfLine(Regex),
}

/// Represents a token
#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
    pub(crate) position: i32,
    pub(crate) line: i32,
}


impl Token {
    /// Creates a new `Token` instance.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token, represented by the `TokenType` enum.
    /// * `value` - The string value of the token.
    /// * `position` - The position of the token within the line.
    /// * `line` - The line number where the token is located.
    ///
    /// # Returns
    ///
    /// A new `Token` instance with the specified properties.
    fn new(token_type: TokenType, value: String, position: i32, line: i32) -> Self {
        Token {
            token_type,
            value,
            position,
            line,
        }
    }
}

pub(crate) struct Lexer {
    code: String,
}

impl Lexer {

    pub(crate) fn new(code: String) -> Self {
        Lexer {
            code,
        }
    }

    /// Tokenize the code into a vector of vectors of tokens, where each inner
    /// vector represents a line in the code and contains the tokens for that
    /// line.
    pub(crate) fn tokenize(&self) -> Vec<Vec<Token>> {
        let mut tokens: Vec<Vec<Token>> = Vec::new();

        for (l_idx, line) in self.code.lines().enumerate() {
            let mut position: i32 = 0;

            if line.is_empty() { continue }

            let mut segment: Vec<Token> = Vec::new();
            let mut is_comment = false;
            for char in line.chars() {
                if char.is_whitespace() { continue }
                if is_comment { continue }
                if char == COMMENT { is_comment = true; continue }

                let token_type = self.match_token(char);
                position += 1;
                segment.push(Token::new(token_type, char.to_string(), position, l_idx as i32));
            }
            tokens.push(segment);
        }
        tokens
    }

    /// Matches a character to a token type.
    ///
    /// Matches a character to a token type based on the type of character. If
    /// the character is a digit, it is matched to a number token type. If the
    /// character is a letter or underscore, it is matched to a variable token
    /// type. If the character is a comment, it is matched to a comment token
    /// type. Otherwise, the character is matched to a token type based on the
    /// operator it represents.
    fn match_token(&self, char: char) -> TokenType {
        let token_type = match char {
            PLUS => TokenType::Plus(Regex::new(r"\+").unwrap()),
            MINUS => TokenType::Minus(Regex::new(r"-").unwrap()),
            MULTIPLY => TokenType::Multiply(Regex::new(r"\*").unwrap()),
            DIVIDE => TokenType::Divide(Regex::new(r"/").unwrap()),
            EQUALS => TokenType::Equals(Regex::new(r"=").unwrap()),
            END_OF_LINE => TokenType::EndOfLine(Regex::new(r"\n").unwrap()),
            '0'..='9' => TokenType::Number(Regex::new(r"[0-9]").unwrap()),
            COMMENT => TokenType::Comment(Regex::new(r"#.*").unwrap()),
            _ => TokenType::Variable(Regex::new(r"[a-zA-Z_]").unwrap()),
        };
        token_type
    }
}