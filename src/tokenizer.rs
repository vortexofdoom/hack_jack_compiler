use crate::tokens::*;
use std::{collections::VecDeque, fs, path::Path};

#[derive(Debug)]
pub enum TokenizerError {
    InvalidInt,
    UnrecognizedToken,
    EndOfFile,
}

impl From<std::num::ParseIntError> for TokenizerError {
    fn from(_: std::num::ParseIntError) -> Self {
        TokenizerError::InvalidInt
    }
}
#[derive(Debug, Clone)]
pub struct Tokenizer {
    chars: VecDeque<char>,
    next: Option<Token>,
}

impl Tokenizer {
    pub fn new<P: AsRef<Path>>(file: P) -> Self {
        let mut tknzr = Tokenizer {
            chars: fs::read_to_string(file)
                .expect("failed to read file")
                .trim()
                .chars()
                .collect(),
            next: None,
        };
        if let Ok(t) = tknzr.get_token() {
            tknzr.next = Some(t);
        }
        tknzr
    }

    pub fn next(&mut self) -> Option<Token> {
        let mut token;
        if let Ok(t) = self.get_token() {
            token = Some(t);
            if let Some(_) = &self.next {
                std::mem::swap(&mut self.next, &mut token);
            }
        } else {
            token = None;
        }
        token
    }

    pub fn peek(&self) -> &Option<Token> {
        &self.next
    }

    fn is_comment(&mut self) -> bool {
        match self.chars.get(0) {
            Some('*') => {
                while let Some(c) = self.chars.pop_front() {
                    if c == '*' && self.chars.get(0) == Some(&'/') {
                        self.chars.pop_front();
                        break;
                    }
                }
                true
            }
            Some('/') => {
                while let Some(c) = self.chars.pop_front() {
                    if c == '\n' {
                        break;
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn handle_string(&mut self) -> Token {
        let mut chars = self.chars.iter().enumerate();
        let mut end = self.chars.len();
        while let Some((i, &c)) = chars.next() {
            if c == '"' {
                end = i;
                break;
            }
        }
        Token::StringConst(self.chars.drain(..end).collect())
    }

    pub fn get_token(&mut self) -> Result<Token, TokenizerError> {
        if let Some(c) = self.chars.pop_front() {
            if SYMBOLS.contains(&c) {
                match c {
                    '"' => Ok(self.handle_string()),
                    '/' => {
                        if self.is_comment() {
                            self.get_token()
                        } else {
                            Ok(Token::Symbol('/'))
                        }
                    }
                    _ => Ok(Token::Symbol(c)),
                }
            } else if c.is_numeric() {
                let mut num = String::from(c);
                let mut chars = self.chars.iter().enumerate();
                let mut end = self.chars.len();
                while let Some((i, &c)) = chars.next() {
                    if !c.is_numeric() {
                        end = i;
                        break;
                    }
                }
                num.extend(self.chars.drain(..end));
                Ok(Token::IntConst(num.parse::<i16>()?))
            } else if c.is_alphabetic() || c == '_' {
                let mut word = String::from(c);
                let mut chars = self.chars.iter().enumerate();
                let mut end = self.chars.len();
                while let Some((i, &c)) = chars.next() {
                    if !(c.is_alphanumeric() || c == '_') {
                        end = i;
                        break;
                    }
                }
                word.extend(self.chars.drain(..end));
                if let Some(&k) = KEYWORDS.get(&word.as_str()) {
                    Ok(Token::Keyword(k))
                } else {
                    Ok(Token::Identifier(word))
                }
            } else if !c.is_whitespace() {
                Err(TokenizerError::UnrecognizedToken)
            } else {
                self.get_token()
            }
        } else {
            Err(TokenizerError::EndOfFile)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyword::*, *};
    #[test]
    fn test_keyword() {
        let mut tknzr = Tokenizer {
            chars: "class".chars().collect(),
            next: None,
        };
        assert_eq!(Some(Token::Keyword(Class)), tknzr.next());
    }

    #[test]
    fn test_symbol() {
        let mut tknzr = Tokenizer {
            chars: VecDeque::from(vec!['(']),
            next: None,
        };
        assert_eq!(Some(Token::Symbol('(')), tknzr.next());
    }

    #[test]
    fn test_int() {
        let mut tknzr = Tokenizer {
            chars: "12364".chars().collect(),
            next: None,
        };
        assert_eq!(Some(Token::IntConst(12364)), tknzr.next());
    }

    #[test]
    fn test_identifier() {
        let s = "_helf12_3rd";
        let mut tknzr = Tokenizer {
            chars: s.chars().collect(),
            next: None,
        };
        assert_eq!(Some(Token::Identifier(String::from(s))), tknzr.next());
    }

    #[test]
    fn test_string() {
        let mut tknzr = Tokenizer {
            chars: "\"12364\"".chars().collect(),
            next: None,
        };
        assert_eq!(
            Some(Token::StringConst(String::from("12364"))),
            tknzr.next()
        );
    }

    #[test]
    fn test_single_line_comment() {
        let mut tknzr = Tokenizer {
            chars: "//Hello this is a comment\nvoid".chars().collect(),
            next: None,
        };
        assert_eq!(Some(Token::Keyword(Void)), tknzr.next());
    }

    #[test]
    fn test_multi_line_comment() {
        let mut tknzr = Tokenizer {
            chars: "/**Hello this is a comment\n\n\n**/let".chars().collect(),
            next: None,
        };
        assert_eq!(Some(Token::Keyword(Let)), tknzr.next());
    }
}
