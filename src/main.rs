mod lexer;
mod lexer_fsm;
mod parser;

use crate::lexer::Lexer;
use crate::parser::Parser;

// Добавим структуру с временем жизни
struct SourceCode<'a> {
    content: &'a str,
}

// Реализация для структуры
impl<'a> SourceCode<'a> {
    fn new(content: &'a str) -> Self {
        SourceCode { content }
    }
}

fn main() {
    let source_code = String::from("a = 1 + 2 * 3\n# This is a comment\nb = 4 - 5");
    
    // Создаем экземпляр с явным временем жизни
    let source = SourceCode::new(&source_code);
    let tokens = Lexer::new(source.content).tokenize();

    println!("{:#?}", tokens);
}
