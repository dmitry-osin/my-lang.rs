use crate::lexer::State;

pub(crate) struct LexerFsm {
    symbols: Vec<char>,
    position: usize,
    current_state: State,
}

impl LexerFsm {
    pub(crate) fn new(symbols: &Vec<char>) -> Self {
        LexerFsm {
            symbols: symbols.clone(),
            position: 0,
            current_state: State::Start,
        }
    }
}
