use crate::tokenizer::tokenize::Tokenizer;
use crate::tokenizer::tokens::{Keyword, Token};

use super::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: Option<Token>,
    previous: Option<Token>,
    next: Option<Token>,
    current_index: usize,
}

pub enum Expectable {
    Token(Token),
    ASTNode(ASTNode),
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

    pub fn parse_sequence(&mut self, tokens: &[Expectable]) -> Result<Vec<Expectable>, String> {
        let mut parsed = vec![];
        for token in tokens {
            self.consume_whitespace();
            match token {
                Expectable::Token(tok) => {
                    if self.current.is_none() {
                        return Err(format!("expected {} got Unexpected End of Input", tok));
                    } else {
                        if self.current.as_mut().unwrap() == tok {
                            parsed.push(Expectable::Token(self.current.as_mut().unwrap().clone()));
                        } else {
                            return Err(format!(
                                "Expected {} got {}",
                                tok,
                                self.current.as_mut().unwrap()
                            ));
                        }
                    }
                }
                Expectable::ASTNode(node) => {
                    if self.current.is_none() {
                        return Err(format!("expected {} got Unexpected End of Input", node));
                    }
                    match node {
                        _ => {
                            let expr = self.parse_statement()?;
                            parsed.push(Expectable::ASTNode(expr));
                        }
                    }
                }
            }
            self.advance();
        }
        return Ok(parsed);
    }

    pub fn advance(&mut self) {
        if self.is_at_end() {
            return;
        }
        self.current_index += 1;
        self.update_state();
    }

    pub fn update_state(&mut self) {
        self.current = self.tokens.iter().nth(self.current_index).cloned();
        self.previous = self.tokens.iter().nth(self.current_index - 1).cloned();
        self.next = self.tokens.iter().nth(self.current_index + 1).cloned();
        // dbg!(&self.current, &self.previous, &self.next);
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
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            self.consume_whitespace();
            if self.current.is_none() {
                break;
            }
            match self.parse_statement() {
                Ok(node) => nodes.push(node),
                Err(e) => return Err(e),
            }
            self.consume_whitespace();
            self.advance();
        }
        Ok(AST { root: None, nodes })
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if self.current.is_none() {
            unreachable!("should not be calling parse_statement on empty tokens");
        }
        match self.current.as_mut().unwrap().clone() {
            Token::KEYWORD(keyword) => self.parse_keyword(&keyword),
            Token::Number(num) => Ok(ASTNode::Expression(Expression::Literal(
                Literal::NumberFloat(num.clone().parse::<f64>().unwrap()),
            ))),
            Token::StringLiteral(string) => Ok(ASTNode::Expression(Expression::Literal(
                Literal::String(string.clone()),
            ))),
            Token::LeftParenthesis => {
                self.advance(); // consume the left parenthesis
                self.consume_whitespace();
                let expr = self.parse_statement()?;
                self.advance(); // move to the next token after the expression
                self.consume_whitespace();
                if self.current != Some(Token::RightParenthesis) {
                    return Err("Expected right parenthesis".to_string());
                }
                Ok(ASTNode::Expression(Expression::Group(Box::new(expr))))
            }
            Token::Not | Token::Minus => {
                let current = self.current.clone();
                self.advance(); // consume the NOT token
                self.consume_whitespace();
                let expr = self.parse_statement();
                let operator = match current.unwrap().clone() {
                    Token::Not => UnaryOperator::Not,
                    Token::Minus => UnaryOperator::Negate,
                    _ => return Err("Expected NOT or MINUS operator".to_string()),
                };

                if let Ok(ASTNode::Expression(exp)) = expr {
                    Ok(ASTNode::Expression(Expression::UnaryOperation(
                        UnaryOperation {
                            operator,
                            operand: Box::new(exp),
                        },
                    )))
                } else {
                    Err(format!("Expected expression after {}", operator))
                }
            }
            (Token::EOF) => Ok(ASTNode::EOF),
            _ => unimplemented!("Yet to implement"),
        }
    }

    fn parse_keyword(&self, keyword: &Keyword) -> Result<ASTNode, String> {
        fn literal_expression(lit: Literal) -> Result<ASTNode, String> {
            Ok(ASTNode::Expression(Expression::Literal(lit)))
        }

        match keyword {
            Keyword::FALSE => literal_expression(Literal::Boolean(false)),
            Keyword::NIL => literal_expression(Literal::NIL),
            Keyword::TRUE => literal_expression(Literal::Boolean(true)),
            _ => unimplemented!("yet to implement other keywords"),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.tokens.len()
    }
}
