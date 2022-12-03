use crate::tokens::*;
use std::{collections::VecDeque, fs, path::Path};

#[derive(Debug)]
pub enum TokenizerError {
    InvalidInt,
    UnrecognizedToken,
    //EndOfFile,
}

impl From<std::num::ParseIntError> for TokenizerError {
    fn from(_: std::num::ParseIntError) -> Self {
        TokenizerError::InvalidInt
    }
}

pub struct Tokenizer {
    chars: VecDeque<char>,
    errors: Vec<TokenizerError>,
    peek: Option<Token>,
}

impl Tokenizer {
    pub fn new<P: AsRef<Path>>(file: P) -> Self {
        let mut tknzr = Tokenizer {
            chars: fs::read_to_string(file)
                .expect("failed to read file")
                .trim()
                .chars()
                .collect(),
            errors: vec![],
            peek: None,
        };
        tknzr.next();
        tknzr
    }

    pub fn next(&mut self) -> Option<Token> {
        let mut token = self.get_token();
        if let (Some(_), Some(_)) = (&token, &self.peek) {
            std::mem::swap(&mut token, &mut self.peek)
        }
        token
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

    fn get_string(&mut self) -> Option<Token> {
        let mut end = self.chars.len();
        for (i, &c) in self.chars.iter().enumerate() {
            if c == '"' {
                end = i;
                break;
            }
        }
        let s: String = self.chars.drain(..end).collect();
        Some(Token::from(s))
    }

    pub fn get_token(&mut self) -> Option<Token> {
        if let Some(c) = self.chars.pop_front() {
            if SYMBOLS.contains(&c) {
                match c {
                    '"' => self.get_string(),
                    _ => {
                        if c == '/' && self.is_comment() {
                            self.get_token()
                        } else {
                            Some(Token::from(c))
                        }
                    }
                }
            } else if c.is_numeric() {
                let mut num = String::from(c);
                let mut end = self.chars.len();
                for (i, &c) in self.chars.iter().enumerate() {
                    if !c.is_numeric() {
                        end = i;
                        break;
                    }
                }
                num.extend(self.chars.drain(..end));
                if let Ok(i) = num.parse::<i16>() {
                    Some(Token::from(i))
                } else {
                    self.errors.push(TokenizerError::InvalidInt);
                    self.get_token()
                }
            } else if c.is_alphabetic() || c == '_' {
                let mut word = String::from(c);
                let mut end = self.chars.len();
                for (i, &c) in self.chars.iter().enumerate() {
                    if !(c.is_alphanumeric() || c == '_') {
                        end = i;
                        break;
                    }
                }
                word.extend(self.chars.drain(..end));
                if let Some(&k) = KEYWORDS.get(word.as_str()) {
                    Some(Token::from(k))
                } else {
                    Some(Token::from(Identifier(word)))
                }
            } else if !c.is_whitespace() {
                self.errors.push(TokenizerError::UnrecognizedToken);
                self.get_token()
            } else {
                self.get_token()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keyword() {
        let mut tknzr = Tokenizer {
            chars: "class".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(token, Keyword::Class);
    }

    #[test]
    fn test_symbol() {
        let mut tknzr = Tokenizer {
            chars: VecDeque::from(vec!['(']),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(token, '(');
    }

    #[test]
    fn test_int() {
        let mut tknzr = Tokenizer {
            chars: "12364".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(token, 12364);
    }

    #[test]
    fn test_identifier() {
        let s = "_helf12_3rd";
        let mut tknzr = Tokenizer {
            chars: s.chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(token, Identifier(String::from(s)));
    }

    #[test]
    fn test_string() {
        let s = "\"this is a string with a // comment in it and a /*/comment**/\"";
        let mut tknzr = Tokenizer {
            chars: s.chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(
            token,
            String::from("this is a string with a // comment in it and a /*/comment**/")
        );
    }

    #[test]
    fn test_single_line_comment() {
        let mut tknzr = Tokenizer {
            chars: "//Hello this is a comment\nvoid".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(token, Keyword::Void);
    }

    #[test]
    fn test_multi_line_comment() {
        let mut tknzr = Tokenizer {
            chars: "/**Hello this is a comment\n\n\n**/let".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(token, Keyword::Let);
    }
}
