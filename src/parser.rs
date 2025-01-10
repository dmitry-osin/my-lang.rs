use crate::lexer::Token;

#[derive(Debug)]
enum Expression {
    Variable(String),
    Number(i32),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    Assignment(String, Box<Expression>),
}

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Statement {
    Expression(Expression),
}

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn parse_assignment(&mut self) -> Statement {
        todo!()
    }

    fn parse_expression(&mut self) -> Expression {
        todo!()
    }

    fn parse_binary_operation(&mut self) -> Expression {
        todo!()
    }

    fn parse_number(&mut self) -> Expression {
        todo!()
    }

    fn parse_variable(&mut self) -> Expression {
        todo!()
    }

}
