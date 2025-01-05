use crate::lexer_fsm::LexerFsm;

/// Represents the type of token
#[derive(Debug)]
pub(crate) enum TokenType {
    Comment,
    Variable,
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    Space,
    LeftParenthesis,
    RightParenthesis,
    Equals,
    EndOfLine,
    Unknown,
}

/// Represents a token
#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
}

/// Represents the state of the lexer
#[derive(Debug)]
pub(crate) enum State {
    Start,
    InNumber,
    InVariable,
    InComment,
    Done,
}

/// Represents the lexer
pub(crate) struct Lexer {
    symbols: Vec<char>,
    position: usize,
    current_state: State,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
        Lexer {
            symbols: input.chars().collect(),
            position: 0,
            current_state: State::Start,
        }
    }

    /// Tokenize the input string and return a vector of tokens. The tokenization process depends on the
    /// current state of the lexer, which is determined by the type of the current character.
    ///
    /// The following states are supported:
    ///
    /// - `State::Start`: The lexer is at the start of the input string.
    /// - `State::InNumber`: The lexer is currently processing a number.
    /// - `State::InVariable`: The lexer is currently processing a variable name.
    /// - `State::InComment`: The lexer is currently processing a comment.
    /// - `State::Done`: The lexer has finished processing the input string.
    pub(crate) fn tokenize(&mut self) -> Vec<Token> {
        let mut fsm = LexerFsm::new(&self.symbols);
        fsm.handle_states()
    }
}
