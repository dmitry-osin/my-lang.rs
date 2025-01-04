mod lexer;
mod lexer_fsm;

use crate::lexer::Lexer;

fn main() {
    let source_code = String::from("a = 1 + 2 * 3\n# This is a comment\nb = 4 - 5\n# This is another comment\nb = 6 / 2\nc = a + b\n");
    let tokens = Lexer::new(&source_code).tokenize();
    println!("{:#?}", tokens);
}
