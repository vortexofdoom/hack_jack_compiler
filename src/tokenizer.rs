use crate::{tokens::*, validation::Token};
use std::{collections::VecDeque, fs, path::Path,};

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

pub struct Tokenizer {
    chars: VecDeque<char>,
    errors: Vec<TokenizerError>,
    peek: Option<Box<dyn Token>>,
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

    pub fn next(&mut self) -> Option<Box<dyn Token>> {
        let mut token = self.get_token();
        match (&token, &self.peek) {
            (Some(_), Some(_)) => std::mem::swap(&mut token, &mut self.peek),
            _ => {}
        }
        token
    }

    pub fn peek(&mut self) -> Option<&dyn Token> {
        if let Some(s) = self.peek {
            Some(s.as_ref())
        } else {
            if let Some(t) = self.get_token() {
                self.peek = Some(t);
                Some(t.as_ref())
            } else {
                None
            }
        }
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

    fn get_string(&mut self) -> Option<Box<dyn Token>> {
        let mut chars = self.chars.iter().enumerate();
        let mut end = self.chars.len();
        while let Some((i, &c)) = chars.next() {
            if c == '"' {
                end = i;
                break;
            }
        }
        let s: String = self.chars.drain(..end).collect();
        Some(s.as_token())
    }

    pub fn get_token(&mut self) -> Option<Box<dyn Token>> {
        if let Some(c) = self.chars.pop_front() {
            if SYMBOLS.contains(&c) {
                match c {
                    '"' => self.get_string(),
                    _ => {
                        if c == '/' && self.is_comment() {
                            self.get_token()
                        } else {
                            Some(c.as_token())
                        }
                    }
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
                if let Ok(i) = num.parse::<i16>() {
                    Some(i.as_token())
                } else {
                    self.errors.push(TokenizerError::InvalidInt);
                    self.get_token()
                }
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
                if let Some(&k) = KEYWORDS.get(word.as_str()) {
                    Some(k.as_token())
                } else {
                    Some(Identifier(word).as_token())
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
        assert_eq!(format!("{token}"), format!("{}", Keyword::Class));
    }

    #[test]
    fn test_symbol() {
        let mut tknzr = Tokenizer {
            chars: VecDeque::from(vec!['(']),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(format!("{token}"), format!("{}", TokenWrapper::wrap('(')));
    }

    #[test]
    fn test_int() {
        let mut tknzr = Tokenizer {
            chars: "12364".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(format!("{token}"), format!("{}", Box::new(TokenWrapper::wrap(12364))));
    }

    // #[test]
    // fn test_identifier() {
    //     let s = "_helf12_3rd";
    //     let mut tknzr = Tokenizer {
    //         chars: s.chars().collect(),
    //         peek: None,
    //     };
    //     if let Ok(token) = tknzr.next()
    //         .expect("no token")
    //         .as_any_box()
    //         .downcast::<Identifier>() {
    //             let Identifier(s2) = *token;
    //             assert_eq!(s2, String::from(s))
    //         }
    // }

    #[test]
    fn test_string() {
        let s = "\"this is a string with a // comment in it and a /*/comment**/\"";
        let mut tknzr = Tokenizer {
            chars: s.chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(format!("{token}"), format!("{}", TokenWrapper::wrap(String::from("this is a string with a // comment in it and a /*/comment**/"))));
    }

    #[test]
    fn test_single_line_comment() {
        let mut tknzr = Tokenizer {
            chars: "//Hello this is a comment\nvoid".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(format!("{token}"), format!("{}", Keyword::Void));
    }

    #[test]
    fn test_multi_line_comment() {
        let mut tknzr = Tokenizer {
            chars: "/**Hello this is a comment\n\n\n**/let".chars().collect(),
            errors: vec![],
            peek: None,
        };
        let token = tknzr.next().expect("no token");
        assert_eq!(format!("{token}"), format!("{}", Keyword::Let));
    }
}
