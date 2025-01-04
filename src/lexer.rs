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
        let mut tokens: Vec<Token> = Vec::new();
        while self.position < self.symbols.len() {
            match self.current_state {
                State::Start => self.handle_start_state(&mut tokens),
                State::InNumber => self.handle_in_number_state(&mut tokens),
                State::InVariable => self.handle_in_variable_state(&mut tokens),
                State::InComment => self.handle_in_comment_state(&mut tokens),
                State::Done => break,
            }
        }
        tokens
    }

    /// Handle the start state of the lexer.
    ///
    /// This function is responsible for handling the start state of the lexer. It matches the current
    /// character and transitions to the appropriate state.
    ///
    /// The following transitions are supported:
    ///
    /// - `State::InNumber`: The current character is a digit.
    /// - `State::InVariable`: The current character is a letter.
    /// - `State::InComment`: The current character is `#`.
    /// - `State::Done`: The current character is the end of the input string.
    ///
    fn handle_start_state(&mut self, tokens: &mut Vec<Token>) {
        match self.symbols[self.position] {
            '0'..='9' => {
                self.current_state = State::InNumber;
            }
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    value: '+'.to_string(),
                });
                self.position += 1;
            }
            '-' => {
                tokens.push(Token {
                    token_type: TokenType::Minus,
                    value: '-'.to_string(),
                });
                self.position += 1;
            }
            '*' => {
                tokens.push(Token {
                    token_type: TokenType::Multiply,
                    value: '*'.to_string(),
                });
                self.position += 1;
            }
            '/' => {
                tokens.push(Token {
                    token_type: TokenType::Divide,
                    value: '/'.to_string(),
                });
                self.position += 1;
            }
            '=' => {
                tokens.push(Token {
                    token_type: TokenType::Equals,
                    value: '='.to_string(),
                });
                self.position += 1;
            }
            ' ' => {
                tokens.push(Token {
                    token_type: TokenType::Space,
                    value: ' '.to_string(),
                });
                self.position += 1;
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::LeftParenthesis,
                    value: '('.to_string(),
                });
                self.position += 1;
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RightParenthesis,
                    value: ')'.to_string(),
                });
                self.position += 1;
            }
            '#' => {
                self.current_state = State::InComment;
            }
            'A'..='Z' | 'a'..='z' => {
                self.current_state = State::InVariable;
            }
            '\n' => {
                tokens.push(Token {
                    token_type: TokenType::EndOfLine,
                    value: '\n'.to_string(),
                });
                self.position += 1;
            }
            _ => {
                tokens.push(Token {
                    token_type: TokenType::Unknown,
                    value: self.symbols[self.position].to_string(),
                });
                self.position += 1;
            }
        }
    }

    /// Handle the `State::InNumber` state of the lexer.
    ///
    /// This function is responsible for handling the `State::InNumber` state of the lexer. It matches
    /// the current character and transitions to the appropriate state.
    ///
    /// The following transitions are supported:
    ///
    /// - `State::Start`: The current character is not a digit.
    fn handle_in_number_state(&mut self, tokens: &mut Vec<Token>) {
        let start = self.position;
        while self.position < self.symbols.len() && self.symbols[self.position].is_digit(10) {
            self.position += 1;
        }
        let number = self.symbols[start..self.position]
            .iter()
            .collect::<String>();

        tokens.push(Token {
            token_type: TokenType::Number,
            value: number,
        });
        self.current_state = State::Start;
    }

    /// Handle the `State::InVariable` state of the lexer.
    ///
    /// This function is responsible for handling the `State::InVariable` state of the lexer. It matches
    /// the current character and transitions to the appropriate state.
    ///
    /// The following transitions are supported:
    ///
    /// - `State::Start`: The current character is not a letter.
    fn handle_in_variable_state(&mut self, tokens: &mut Vec<Token>) {
        let start = self.position;
        while self.position < self.symbols.len()
            && self.symbols[self.position].is_ascii_alphabetic()
        {
            self.position += 1;
        }
        let variable = self.symbols[start..self.position]
            .iter()
            .collect::<String>();

        tokens.push(Token {
            token_type: TokenType::Variable,
            value: variable,
        });
        self.current_state = State::Start;
    }

    /// Handle the `State::InComment` state of the lexer.
    ///
    /// This function is responsible for handling the `State::InComment` state of the lexer. It matches
    /// the current character and transitions to the appropriate state.
    ///
    /// The following transitions are supported:
    ///
    /// - `State::Start`: The current character is a newline.
    fn handle_in_comment_state(&mut self, tokens: &mut Vec<Token>) {
        let start = self.position;
        while self.position < self.symbols.len() {
            if self.symbols[self.position] == '\n' {
                self.position += 1;
                break;
            }
            self.position += 1;
        }
        let comment = self.symbols[start..self.position]
            .iter()
            .collect::<String>();

        tokens.push(Token {
            token_type: TokenType::Comment,
            value: comment,
        });
        self.current_state = State::Start;
    }
}
