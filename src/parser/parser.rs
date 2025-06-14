use crate::tokenizer::tokenize::Tokenizer;
use crate::tokenizer::tokens::{Token, KW};

use super::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: Option<Token>,
    previous: Option<Token>,
    next: Option<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(file_content: String) -> Self {
        let mut tokenizer = Tokenizer::new(file_content);
        let tokens = tokenizer.tokenize();
        let mut instance = Parser {
            tokens: tokens,
            current: None,
            previous: None,
            next: None,
            current_index: 0,
        };
        instance.update_state();
        instance
    }

    pub fn advance(&mut self) {
        if self.is_at_end() {
            return;
        }
        self.current_index += 1;
        self.update_state();
        self.consume_whitespace();
    }

    pub fn update_state(&mut self) {
        self.current = self.tokens.iter().nth(self.current_index).cloned();
        self.previous = self.tokens.iter().nth(self.current_index - 1).cloned();
        self.next = self.tokens.iter().nth(self.current_index + 1).cloned();
    }

    pub fn consume_whitespace(&mut self) {
        while !self.is_at_end()
            && self.current.is_some()
            && self.current.as_ref().unwrap().is_ignorable()
        {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        let nodes = self.parse_program();
        return Ok(AST { root: None, nodes });
    }

    pub fn parse_program(&mut self) -> Vec<ASTNode> {
        let mut nodes: Vec<ASTNode> = vec![];
        while !self.is_at_end() && self.current.clone().unwrap() != Token::EOF {
            let stm = self.parse_statement();
            if stm.is_some() {
                nodes.push(ASTNode::Statement(stm.unwrap()));
            } else {
                let expr = self.parse_expression();
                nodes.push(ASTNode::Expression(expr));
            }
        }
        return nodes;
    }

    pub fn parse_expression(&mut self) -> Expression {
        self.parse_expression_within_binding_power(0.0)
    }

    pub fn infix_binding_power(&self, op: &BinaryOperator) -> (f32, f32) {
        match op {
            BinaryOperator::Add => (2.1, 2.2),
            BinaryOperator::Subtract => (2.1, 2.2),
            BinaryOperator::Divide => (4.1, 4.2),
            BinaryOperator::Multiply => (4.1, 4.2),
            _ => (0.0, 0.0),
        }
    }

    pub fn prefix_binding_power(&self, op: &UnaryOperator) -> ((), f32) {
        match op {
            UnaryOperator::Negate => ((), 5.1),
            UnaryOperator::Not => ((), 5.1),
            UnaryOperator::Plus => ((), 5.1),
        }
    }
    pub fn parse_expression_within_binding_power(
        &mut self,
        minimum_binding_power: f32,
    ) -> Expression {
        let mut lhs = match self.current.as_ref().unwrap() {
            Token::Minus | Token::Plus | Token::Not => {
                let unary_op = match self.current.as_ref().unwrap() {
                    Token::Minus => UnaryOperator::Negate,
                    Token::Plus => UnaryOperator::Plus,
                    Token::Not => UnaryOperator::Not,
                    t => panic!("bad token: {:?}", t),
                };
                let ((), rbp) = self.prefix_binding_power(&unary_op);
                self.advance();
                let rhs = self.parse_expression_within_binding_power(rbp);
                Expression::UnaryOperation(UnaryOperation {
                    operator: unary_op,
                    operand: Box::new(rhs),
                })
            }
            Token::LeftParenthesis => {
                self.advance();
                let inner = self.parse_expression_within_binding_power(0.0);
                Expression::Group(Box::new(inner))
            }
            _ => self.parse_atomic_expression(),
        };
        loop {
            let op = match self.current.as_ref().unwrap() {
                Token::EOF => break,
                Token::RightParenthesis => {
                    self.advance();
                    break;
                }
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                Token::Divide => BinaryOperator::Divide,
                Token::Multiply => BinaryOperator::Multiply,
                t => panic!("bad token: {:?}", t),
            };
            let (l_bp, r_bp) = self.infix_binding_power(&op);
            if l_bp < minimum_binding_power {
                break;
            }
            self.advance();
            let rhs = self.parse_expression_within_binding_power(r_bp);
            lhs = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(lhs),
                operator: op,
                right: Box::new(rhs),
            });
        }
        lhs
    }

    pub fn parse_atomic_expression(&mut self) -> Expression {
        let expr = self.parse_literal();
        if expr.is_none() {
            let expr = self.parse_identifier();
            if expr.is_none() {
                panic!("Expected Atom found {:?}", self.current)
            } else {
                self.advance();
                Expression::Identifier(expr.unwrap())
            }
        } else {
            self.advance();
            Expression::Literal(expr.unwrap())
        }
    }

    pub fn parse_literal(&self) -> Option<Literal> {
        match self.current.as_ref().unwrap().clone() {
            Token::Number(num) => Some(Literal::NumberFloat(num.clone().parse::<f64>().unwrap())),
            Token::StringLiteral(string) => Some(Literal::String(string.clone())),
            Token::Keyword(KW::FALSE) => Some(Literal::Boolean(false)),
            Token::Keyword(KW::TRUE) => Some(Literal::Boolean(true)),
            Token::Keyword(KW::NIL) => Some(Literal::NIL),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Option<String> {
        match self.current.as_ref().unwrap().clone() {
            Token::Identifier(x) => Some(x),
            _ => None,
        }
    }

    fn parse_for_loop(&self) -> ForStatement {
        unimplemented!()
    }

    fn parse_if_statement(&self) -> IfStatement {
        unimplemented!()
    }

    fn parse_while_statement(&self) -> WhileStatement {
        unimplemented!()
    }

    fn parse_return_statement(&self) -> ReturnStatement {
        unimplemented!()
    }

    fn parse_variable_declaration(&self) -> VariableDeclaration {
        unimplemented!()
    }

    fn parse_function_declaration(&self) -> FunctionDeclaration {
        unimplemented!()
    }

    fn parse_block_statement(&self) -> Block {
        unimplemented!()
    }

    fn parse_declaration(&self) -> Declaration {
        match self.current.as_ref().unwrap().clone() {
            Token::Keyword(KW::VAR) => Declaration::Variable(self.parse_variable_declaration()),
            Token::Keyword(KW::FUN) => Declaration::Function(self.parse_function_declaration()),
            _ => unreachable!("Not a declaration"),
        }
    }

    fn parse_statement(&self) -> Option<Statement> {
        match self.current.as_ref().unwrap().clone() {
            Token::Keyword(KW::FOR) => Some(Statement::For(self.parse_for_loop())),
            Token::Keyword(KW::IF) => Some(Statement::If(self.parse_if_statement())),
            Token::Keyword(KW::WHILE) => Some(Statement::While(self.parse_while_statement())),
            Token::Keyword(KW::RETURN) => Some(Statement::Return(self.parse_return_statement())),
            Token::Keyword(KW::VAR) | Token::Keyword(KW::FUN) => {
                Some(Statement::Declaration(self.parse_declaration()))
            }
            Token::LeftBrace => Some(Statement::Block(self.parse_block_statement())),
            _ => None,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.tokens.len()
    }
}
