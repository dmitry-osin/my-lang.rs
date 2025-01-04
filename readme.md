# My Lang

My Lang is a project aimed at creating a custom programming language lexer using Rust. The lexer is responsible for breaking down input code into tokens, which is the first step in the process of parsing and interpreting code.

## Project Overview

The main components of the project are:

1. A [Lexer](src/lexer.rs) struct that tokenizes input code
2. A [LexerFsm](src/lexer_fsm.rs) (Finite State Machine) struct that manages the lexer's state
3. Various token types representing different elements of the language

### Goal
The main goal of this project is to learn the Rust programming language and master the principles of lexer and compiler development.

## Lexer Features

The lexer recognizes the following token types:

- Comment: Represents comments in the code (starting with '#')
- Variable: Represents variable names
- Number: Represents numeric values
- Plus: Represents the addition operator (+)
- Minus: Represents the subtraction operator (-)
- Multiply: Represents the multiplication operator (*)
- Divide: Represents the division operator (/)
- Space: Represents whitespace
- LeftParenthesis: Represents a left parenthesis (
- RightParenthesis: Represents a right parenthesis )
- Equals: Represents the assignment operator (=)
- EndOfLine: Represents the end of a line
- Unknown: Represents any unrecognized token

## Lexer States

The lexer uses a finite state machine with the following states:

- Start: The initial state
- InNumber: Processing a numeric value
- InVariable: Processing a variable name
- InComment: Processing a comment
- Done: Finished processing the input

## Usage Example

Here's how to use the Lexer:

```rust
use my_lang::lexer::Lexer;

fn main() {
    let source_code = String::from("a = 1 + 2 * 3\n# This is a comment\nb = 4 - 5\n# This is another comment\nb = 6 / 2\nc = a + b\n");
    let tokens = Lexer::new(&source_code).tokenize();
    println!("{:#?}", tokens);
}
```

## Future Plans
The next step in the project is to implement a Parser that will take the tokens produced by the Lexer and create an Abstract Syntax Tree (AST) or other intermediate representation for further processing.