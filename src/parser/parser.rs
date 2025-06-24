use crate::tokenizer::tokenize::{Position, Tokenizer};
use crate::tokenizer::tokens::{PositionedToken, Token, KW};
use std::process::exit;

use super::ast::*;

pub struct Parser {
    tokens: Vec<PositionedToken>,
    current: Token,
    previous: Token,
    next: Token,
    current_index: usize,
    pos: Position,
}

impl Parser {
    pub fn new(file_content: String) -> Self {
        let mut tokenizer = Tokenizer::new(file_content);
        let tokens = tokenizer.tokenize();
        let mut instance = Parser {
            tokens: tokens,
            current: Token::EOF,
            previous: Token::EOF,
            next: Token::EOF,
            current_index: 0,
            pos: Position { line: 0, column: 0 },
        };
        instance.update_state();
        instance.consume_whitespace();
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
        let invalid_pos_token = PositionedToken {
            tok: Token::EOF,
            pos: Position { line: 0, column: 0 },
        };

        let current_pos_token = self
            .tokens
            .iter()
            .nth(self.current_index)
            .unwrap_or(&invalid_pos_token)
            .clone();

        self.current = current_pos_token.tok;

        self.pos = current_pos_token.pos;

        self.previous = self
            .tokens
            .iter()
            .nth(self.current_index - 1)
            .unwrap_or(&invalid_pos_token)
            .clone()
            .tok;

        self.next = self
            .tokens
            .iter()
            .nth(self.current_index + 1)
            .unwrap_or(&invalid_pos_token)
            .clone()
            .tok;
    }

    pub fn consume_whitespace(&mut self) {
        while !self.is_at_end() && self.current.is_ignorable() {
            self.current_index += 1;
            self.update_state();
        }
    }

    pub fn expect_next_token_to_be(&mut self, tok: Token) {
        if self.current != tok {
            self.expectation_failure(&format!("{:?}", tok))
        }
        self.advance();
    }

    pub fn exit_with_msg(&self, msg: &String) -> ! {
        eprintln!("{}", msg);
        exit(65)
    }

    pub fn expectation_failure(&self, expected: &str) -> ! {
        self.exit_with_msg(&format!(
            "[line {}] Error at '{}': Expect {}.",
            &self.pos.line, &self.current, expected
        ))
    }

    pub fn parse(&mut self) -> AST {
        let nodes = self.parse_program_till(Token::EOF);
        return AST { root: None, nodes };
    }

    pub fn parse_program_till(&mut self, ending_token: Token) -> Vec<ASTNode> {
        let mut nodes: Vec<ASTNode> = vec![];
        while !self.is_at_end() && self.current != Token::EOF && self.current != ending_token {
            let stm = self.parse_statement();
            if stm.is_some() {
                if self.current == Token::Semicolon {
                    self.advance();
                }
                nodes.push(ASTNode::Statement(stm.unwrap()));
            } else {
                let expr = self.parse_expression();
                if self.current == Token::Semicolon {
                    self.advance();
                    nodes.push(ASTNode::Statement(Statement::Expression(expr)));
                } else {
                    nodes.push(ASTNode::Expression(expr));
                }
            }
        }
        return nodes;
    }

    pub fn parse_expression(&mut self) -> Expression {
        let expr = self.parse_expression_within_binding_power(0.0);
        return expr;
    }

    pub fn infix_binding_power(&self, op: &BinaryOperator) -> (f32, f32) {
        match op {
            BinaryOperator::Assignment => (1.9, 1.2),

            BinaryOperator::Or => (2.0, 2.0),

            BinaryOperator::And => (3.0, 3.0),

            BinaryOperator::Equal | BinaryOperator::NotEqual => (4.1, 4.2),

            BinaryOperator::LessThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThanOrEqual => (5.1, 5.2),

            BinaryOperator::Add | BinaryOperator::Subtract => (6.1, 6.2),

            BinaryOperator::Divide | BinaryOperator::Multiply => (7.1, 7.2),
            //Unary not and minus
            BinaryOperator::Call => (9.1, 9.2),

            BinaryOperator::Dot => (10.1, 10.2),
            _ => (0.0, 0.0),
        }
    }

    pub fn prefix_binding_power(&self, op: &UnaryOperator) -> ((), f32) {
        match op {
            UnaryOperator::Negate => ((), 8.1),
            UnaryOperator::Not => ((), 8.1),
            UnaryOperator::Plus => ((), 8.1),
            UnaryOperator::Group => ((), 9.1),
        }
    }

    pub fn parse_expression_within_binding_power(
        &mut self,
        minimum_binding_power: f32,
    ) -> Expression {
        let mut lhs = match &self.current {
            Token::Minus | Token::Plus | Token::Not => {
                let unary_op = match &self.current {
                    Token::Minus => UnaryOperator::Negate,
                    Token::Plus => UnaryOperator::Plus,
                    Token::Not => UnaryOperator::Not,
                    _ => unreachable!("Impossible to get here since operator was already matched"),
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
                if self.current != Token::RightParenthesis {
                    self.expectation_failure("')'")
                }
                self.advance();
                Expression::UnaryOperation(UnaryOperation {
                    operator: UnaryOperator::Group,
                    operand: Box::new(inner),
                })
            }
            _ => self.parse_atomic_expression(),
        };
        loop {
            let op = match &self.current {
                Token::EOF => break,
                Token::RightParenthesis => break,
                Token::Semicolon => break,
                Token::RightBrace => break,
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                Token::Divide => BinaryOperator::Divide,
                Token::Multiply => BinaryOperator::Multiply,
                Token::And | Token::Keyword(KW::AND) => BinaryOperator::And,
                Token::Or | Token::Keyword(KW::OR) => BinaryOperator::Or,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                Token::LessThan => BinaryOperator::LessThan,
                Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                Token::Equals => BinaryOperator::Equal,
                Token::NotEquals => BinaryOperator::NotEqual,
                Token::Assignment => BinaryOperator::Assignment,
                Token::LeftParenthesis => BinaryOperator::Call,
                Token::Dot => BinaryOperator::Dot,
                _ => self.expectation_failure(&format!("Binary Operator after {:#?}", &lhs)),
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
                self.expectation_failure("Atomic Expression")
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
        match &self.current {
            Token::Number(num) => Some(Literal::NumberFloat(num.clone().parse::<f64>().unwrap())),
            Token::StringLiteral(string) => Some(Literal::String(string.clone())),
            Token::Keyword(KW::FALSE) => Some(Literal::Boolean(false)),
            Token::Keyword(KW::TRUE) => Some(Literal::Boolean(true)),
            Token::Keyword(KW::NIL) => Some(Literal::NIL),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Option<String> {
        match &self.current {
            Token::Identifier(x) => Some(x.clone()),
            _ => None,
        }
    }

    fn parse_for_loop(&mut self) -> ForStatement {
        self.expect_next_token_to_be(Token::Keyword(KW::FOR));
        self.expect_next_token_to_be(Token::LeftParenthesis);
        let initializer = if self.current == Token::Semicolon {
            None
        } else if self.current == Token::Keyword(KW::VAR) {
            Some(Statement::Declaration(Declaration::Variable(
                self.parse_variable_declaration(),
            )))
        } else {
            Some(Statement::Expression(self.parse_expression()))
        };
        if self.current == Token::Semicolon {
            self.advance();
        }
        let condition = if self.current != Token::Semicolon {
            Some(self.parse_expression_within_binding_power(0.0))
        } else {
            None
        };

        if self.current == Token::Semicolon {
            self.advance();
        }

        let increment = if self.current == Token::RightParenthesis {
            None
        } else {
            Some(self.parse_expression_within_binding_power(0.0))
        };
        self.expect_next_token_to_be(Token::RightParenthesis);

        let body = self
            .parse_statement()
            .expect("Expected body of for loop after condition");
        if matches!(body, Statement::Declaration(_)) {
            self.expectation_failure("statement or expression");
        }

        ForStatement {
            initializer: Box::new(initializer),
            condition: condition,
            increment: Box::new(increment.and_then(|e| Some(Statement::Expression(e)))),
            body: Box::new(body),
        }
    }

    fn parse_if_statement(&mut self) -> IfStatement {
        self.expect_next_token_to_be(Token::Keyword(KW::IF));
        self.expect_next_token_to_be(Token::LeftParenthesis);
        let condition = self.parse_expression_within_binding_power(0.0);
        self.expect_next_token_to_be(Token::RightParenthesis);
        let then_branch = self
            .parse_statement()
            .expect("Expected then branch after if condition");
        if self.current == Token::Semicolon {
            self.advance();
        }
        let else_branch = if self.current == Token::Keyword(KW::ELSE) {
            self.advance();
            let else_statement = Some(
                self.parse_statement()
                    .expect("Expected else branch after else condition"),
            );
            if self.current == Token::Semicolon {
                self.advance();
            }
            else_statement
        } else {
            None
        };

        IfStatement {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    fn parse_while_statement(&mut self) -> WhileStatement {
        self.expect_next_token_to_be(Token::Keyword(KW::WHILE));
        self.expect_next_token_to_be(Token::LeftParenthesis);
        let condition = self.parse_expression_within_binding_power(0.0);
        self.expect_next_token_to_be(Token::RightParenthesis);
        let body = self
            .parse_statement()
            .expect("Expected body of while loop after condition");
        WhileStatement {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    fn parse_return_statement(&self) -> ReturnStatement {
        unimplemented!()
    }

    fn parse_variable_declaration(&mut self) -> VariableDeclaration {
        self.expect_next_token_to_be(Token::Keyword(KW::VAR));
        let identifier = self.parse_identifier().expect("Expected variable name");
        self.advance();
        if self.current == Token::Assignment {
            self.advance();
            let initializer = self.parse_expression();
            VariableDeclaration {
                name: identifier,
                value: Some(initializer),
            }
        } else {
            VariableDeclaration {
                name: identifier,
                value: None,
            }
        }
    }

    fn parse_function_declaration(&self) -> FunctionDeclaration {
        unimplemented!()
    }

    fn parse_block_statement(&mut self) -> Block {
        self.expect_next_token_to_be(Token::LeftBrace);
        let nodes = self.parse_program_till(Token::RightBrace);
        self.expect_next_token_to_be(Token::RightBrace);
        Block { statements: nodes }
    }

    fn parse_declaration(&mut self) -> Declaration {
        match &self.current {
            Token::Keyword(KW::VAR) => Declaration::Variable(self.parse_variable_declaration()),
            Token::Keyword(KW::FUN) => Declaration::Function(self.parse_function_declaration()),
            _ => unreachable!("Not a declaration"),
        }
    }

    fn parse_print_statement(&mut self) -> PrintStatement {
        self.expect_next_token_to_be(Token::Keyword(KW::PRINT));
        let expr = self.parse_expression();
        PrintStatement { value: expr }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current {
            Token::Keyword(KW::FOR) => Some(Statement::For(self.parse_for_loop())),
            Token::Keyword(KW::IF) => Some(Statement::If(self.parse_if_statement())),
            Token::Keyword(KW::WHILE) => Some(Statement::While(self.parse_while_statement())),
            Token::Keyword(KW::RETURN) => Some(Statement::Return(self.parse_return_statement())),
            Token::Keyword(KW::VAR) | Token::Keyword(KW::FUN) => {
                Some(Statement::Declaration(self.parse_declaration()))
            }
            Token::LeftBrace => Some(Statement::Block(self.parse_block_statement())),
            Token::Keyword(KW::PRINT) => Some(Statement::Print(self.parse_print_statement())),
            _ => None,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.tokens.len()
    }
}
