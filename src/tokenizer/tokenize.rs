use super::tokens::{Token, KW};
use std::cmp::min;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub struct Tokenizer {
    input: String,
    current_index: usize,
    char: Option<char>,
    position: Position,
    healthy: bool,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer {
            input,
            current_index: 0,
            char: None,
            position: Position { line: 1, column: 1 },
            healthy: true,
        };
        tokenizer.advance(); // Initialize the first character
        tokenizer
    }

    pub fn has_errors(&self) -> bool {
        !self.healthy
    }

    pub fn seek(&self, len: Option<usize>, start: Option<usize>) -> Option<String> {
        let start = start.unwrap_or(self.current_index - 1);
        let end = match len {
            Some(length) => min(start + length, self.input.len()),
            None => self.input.len(),
        };

        if start < self.input.len() && end <= self.input.len() {
            let seek_value = self.input[start..end].to_string();
            Some(seek_value)
        } else {
            None // Out of bounds
        }
    }

    pub fn advance(&mut self) {
        if self.current_index < self.input.len() {
            self.char = Some(self.input[self.current_index..].chars().next().unwrap());
            self.current_index += self.char.unwrap().len_utf8();
            if self.char == Some('\n') {
                self.position.line += 1;
                self.position.column = 1;
            } else {
                self.position.column += 1;
            }
        } else {
            self.char = None; // End of input
        }
    }

    #[allow(dead_code)]
    pub fn retreat(&mut self) {
        if self.current_index > 0 {
            self.current_index -= self.char.unwrap().len_utf8();
            if self.char == Some('\n') {
                if self.position.line > 1 {
                    self.position.line -= 1;
                }
                // Reset column to the start of the line (not implemented)
                self.position.column = 1; // This is a simplification
            } else {
                self.position.column -= 1;
            }
            self.char = Some(self.input[self.current_index..].chars().next().unwrap());
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        fn single_character_token(c: char) -> Option<Token> {
            match c {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                // '%' => Some(Token::Modulus),
                ',' => Some(Token::Comma),
                ';' => Some(Token::Semicolon),
                ':' => Some(Token::Colon),
                '.' => Some(Token::Dot),
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                '[' => Some(Token::LeftBracket),
                ']' => Some(Token::RightBracket),
                '=' => Some(Token::Assignment),
                '!' => Some(Token::Not),
                '&' => Some(Token::BitwiseAnd),
                '|' => Some(Token::BitwiseOr),
                '^' => Some(Token::BitwiseXor),
                '~' => Some(Token::BitwiseNot),
                '<' => Some(Token::LessThan),
                '>' => Some(Token::GreaterThan),
                _ => None,
            }
        }

        fn double_character_token(str: String) -> Option<Token> {
            match str.as_str() {
                "==" => Some(Token::Equals),
                "!=" => Some(Token::NotEquals),
                "<=" => Some(Token::LessThanOrEqual),
                ">=" => Some(Token::GreaterThanOrEqual),
                "&&" => Some(Token::And),
                "||" => Some(Token::Or),
                // "<<" => Some(Token::BitwiseShiftLeft),
                // ">>" => Some(Token::BitwiseShiftRight),
                // "++" => Some(Token::Increment),
                // "--" => Some(Token::Decrement),
                "//" => Some(Token::Comment(String::new())), // Placeholder for comments
                _ => None,
            }
        }
        fn keyword_from(word: &str) -> Option<Token> {
            match word {
                "if" => Some(Token::Keyword(KW::IF)),
                "else" => Some(Token::Keyword(KW::ELSE)),
                "while" => Some(Token::Keyword(KW::WHILE)),
                "for" => Some(Token::Keyword(KW::FOR)),
                "return" => Some(Token::Keyword(KW::RETURN)),
                "fun" => Some(Token::Keyword(KW::FUN)),
                "var" => Some(Token::Keyword(KW::VAR)),
                "nil" => Some(Token::Keyword(KW::NIL)),
                "true" => Some(Token::Keyword(KW::TRUE)),
                "false" => Some(Token::Keyword(KW::FALSE)),
                "class" => Some(Token::Keyword(KW::CLASS)),
                "print" => Some(Token::Keyword(KW::PRINT)),
                "super" => Some(Token::Keyword(KW::SUPER)),
                "this" => Some(Token::Keyword(KW::THIS)),
                "and" => Some(Token::Keyword(KW::AND)),
                "or" => Some(Token::Keyword(KW::OR)),
                _ => None,
            }
        }

        while let Some(c) = self.char {
            match c {
                '0'..='9' => {
                    let mut current_word = String::new();
                    current_word.push(c);
                    self.advance();
                    let mut found_dot = false;
                    while let Some(next_char) = self.char {
                        if next_char == '.' && !found_dot {
                            found_dot = true; // Allow one dot for decimal numbers
                            current_word.push(next_char);
                            self.advance();
                        } else if next_char.is_digit(10) {
                            current_word.push(next_char);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(current_word.clone()));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut current_word = String::new();
                    current_word.push(c);
                    self.advance();
                    while let Some(next_char) = self.char {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            current_word.push(next_char);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if let Some(keyword_token) = keyword_from(&current_word) {
                        tokens.push(keyword_token);
                    } else {
                        tokens.push(Token::Identifier(current_word.clone()));
                    }
                }
                '\'' => {
                    let mut current_word = String::new();
                    let mut terminated = false;
                    self.advance(); // Skip the opening quote
                    while let Some(next_char) = self.char {
                        if next_char == '\'' {
                            self.advance(); // Skip the closing quote
                            terminated = true;
                            break;
                        } else {
                            current_word.push(next_char);
                            self.advance();
                        }
                    }
                    if !terminated {
                        self.healthy = false; // Mark as unhealthy if not terminated
                        tokens.push(Token::Error(
                            "Unterminated string.".to_string(),
                            self.position.clone(),
                        ));
                    } else {
                        tokens.push(Token::StringLiteral(current_word.clone()));
                    }
                }
                '"' => {
                    let mut current_word = String::new();
                    self.advance(); // Skip the opening quote
                    let mut terminated = false;
                    while let Some(next_char) = self.char {
                        if next_char == '"' {
                            self.advance(); // Skip the closing quote
                            terminated = true;
                            break;
                        } else {
                            current_word.push(next_char);
                            self.advance();
                        }
                    }
                    if !terminated {
                        self.healthy = false; // Mark as unhealthy if not terminated
                        tokens.push(Token::Error(
                            "Unterminated string.".to_string(),
                            self.position.clone(),
                        ));
                    } else {
                        tokens.push(Token::StringLiteral(current_word.clone()));
                    }
                }
                '`' => {
                    let mut current_word = String::new();
                    self.advance(); // Skip the opening backtick
                    while let Some(next_char) = self.char {
                        if next_char == '`' {
                            self.advance(); // Skip the closing backtick
                            break;
                        } else {
                            current_word.push(next_char);
                            self.advance();
                        }
                    }
                    tokens.push(Token::InterpolatedString(current_word.clone()));
                    current_word.clear();
                }
                _ if c.is_whitespace() => {
                    let mut whitespace = String::new();
                    while let Some(next_char) = self.char {
                        if next_char.is_whitespace() {
                            whitespace.push(next_char);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Whitespace(whitespace));
                }
                _ => {
                    if let Some(two_chars) = self.seek(Some(2), None) {
                        if let Some(token) = double_character_token(two_chars) {
                            match token {
                                Token::Comment(_) => {
                                    // Handle comments
                                    let mut comment = String::new();
                                    self.advance(); // Skip the first character
                                    self.advance(); // Skip the second character
                                    while let Some(next_char) = self.char {
                                        if next_char == '\n' {
                                            break; // End of comment
                                        }
                                        comment.push(next_char);
                                        self.advance();
                                    }
                                    tokens.push(Token::Comment(comment));
                                }
                                _ => {
                                    tokens.push(token);
                                    self.advance(); // Skip the next character as well
                                    self.advance();
                                }
                            }
                        } else if let Some(token) = single_character_token(c) {
                            tokens.push(token);
                            self.advance();
                        } else {
                            tokens.push(Token::UnexpectedCharacter(c, self.position.clone()));
                            self.advance();
                            self.healthy = false; // Mark as unhealthy on unexpected character
                        }
                    }
                }
            }
        }

        tokens.push(Token::EOF); // End of file token
        tokens
    }
}
