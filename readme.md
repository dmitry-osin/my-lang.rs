# My Lang

My Lang is a project aimed at creating a custom programming language. Currently, it implements a Lexer for tokenizing the input code. Future plans include adding a Parser to further process the tokenized input.

## Goal

The main goal of this project is to learn the Rust programming language and master the principles of compiler development.

## Current Status

- [x] Lexer implementation
- [ ] Parser implementation (in progress)

## Lexer

### Token Types

The Lexer recognizes the following token types:

- Comment: Represents comments in the code
- Variable: Represents variable names
- Number: Represents numeric values
- Plus: Represents the addition operator (+)
- Minus: Represents the subtraction operator (-)
- Multiply: Represents the multiplication operator (*)
- Divide: Represents the division operator (/)
- Equals: Represents the assignment operator (=)
- EndOfLine: Represents the end of a line

Each token type is associated with a regular expression for matching.

The Lexer is responsible for breaking down the input code into tokens. It supports the following token types:

- Comments
- Variables
- Numbers
- Arithmetic operators (+, -, *, /)
- Assignment operator (=)
- End of line

### Usage Example

Here's how to use the Lexer:

```rust
use my_lang::lexer::Lexer;

fn main() {
    let code = String::from("a = 1 + 2 * 3\n# This is a comment\nb = 4 - 5\n");
    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
}

```

This will tokenize the input code and print the resulting tokens in a debug format.

## Project Structure
src/lexer.rs: Contains the Lexer implementation, including token types and tokenization logic.
src/main.rs: Entry point of the application, demonstrating usage of the Lexer.

## Future Plans
The next step in the project is to implement a Parser that will take the tokens produced by the Lexer and create an Abstract Syntax Tree (AST) or other intermediate representation for further processing.