use crate::{parser::CompilationError, tokens::*};
use std::collections::VecDeque;

impl From<std::num::ParseIntError> for CompilationError {
    fn from(_: std::num::ParseIntError) -> Self {
        CompilationError::InvalidInt
    }
}

#[derive(Debug, Default)]
pub struct Tokenizer {
    chars: VecDeque<char>,
    errors: Vec<CompilationError>,
    //next: Option<Token>,
}

impl Tokenizer {
    pub fn new(file: String) -> Self {
        let tknzr = Tokenizer {
            chars: file.trim().chars().collect(),
            errors: vec![],
            //next: None,
        };
        tknzr
    }

    // pub fn advance(&mut self) -> Option<Token> {
    //     let mut next = Rc::new(self.next);
    //     let mut get = Rc::new(self.get_token());

    //     std::mem::swap(&mut next, &mut get);
    //     // let mut curr_next = self.next.as_ref().as_mut();
    //     // let mut next = self.get_token().as_mut().as_ref();
    //     // let swap: bool;
    //     // match (curr_next, next) {
    //     //     (Some(_), _) => swap = true,
    //     //     (None, Some(_)) => swap = false,
    //     //     _ => return None,
    //     // }
    //     // if swap {
    //     //     std::mem::swap(&mut curr_next, &mut next)
    //     // } else {

    //     // }
    //     // let (mut t1, mut t2) = (self.next.as_mut(), self.get_token().as_mut());
    //     // match (t1, t2) {

    //     // }
    //     // Option::unwrap_or_default(None)
    // }

    // pub fn peek(&self) -> Option<&Token> {
    //     self.next.as_ref()
    // }

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
        self.chars.pop_front();
        Some(Token::StringConstant(s))
    }

    pub fn advance(&mut self) -> Option<Token> {
        if let Some(c) = self.chars.pop_front() {
            if SYMBOLS.contains(&c) {
                match c {
                    '"' => self.get_string(),
                    _ => {
                        if c == '/' && self.is_comment() {
                            self.advance()
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
                    self.errors.push(CompilationError::InvalidInt);
                    self.advance()
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
                    Some(Token::Identifier(word))
                }
            } else if !c.is_whitespace() {
                self.errors.push(CompilationError::UnrecognizedToken);
                self.advance()
            } else {
                self.advance()
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
        let mut tknzr = Tokenizer::new("class".chars().collect());
        let token = tknzr.advance().expect("no token");
        assert_eq!(token, Keyword::Class);
    }

    #[test]
    fn test_symbol() {
        let mut tknzr = Tokenizer::new(String::from('('));
        let token = tknzr.advance().expect("no token");
        assert_eq!(token, '(');
    }

    #[test]
    fn test_int() {
        let mut tknzr = Tokenizer::new("12364".chars().collect());
        let token = tknzr.advance().expect("no token");
        assert_eq!(token, 12364);
    }

    #[test]
    fn test_identifier() {
        let s = "_helf12_3rd";
        let mut tknzr = Tokenizer::new(s.chars().collect());
        let token = tknzr.advance().expect("no token");
        assert_eq!(token, Token::Identifier(String::from(s)));
    }

    #[test]
    fn test_string() {
        let s = "\"this is a string with a // comment in it and a /*/comment**/\"";
        let mut tknzr = Tokenizer::new(s.chars().collect());
        let token = tknzr.advance().expect("no token");
        assert_eq!(
            token,
            String::from("this is a string with a // comment in it and a /*/comment**/")
        );
    }

    #[test]
    fn test_single_line_comment() {
        let s = "//Hello this is a comment\nvoid";
        let mut tknzr = Tokenizer::new(s.chars().collect());
        let token = tknzr.advance().expect("no token");
        assert_eq!(token, Keyword::Void);
    }

    #[test]
    fn test_multi_line_comment() {
        let s = "/**Hello this is a comment\n\n\n**/let";
        let mut tknzr = Tokenizer::new(s.chars().collect());
        let token = tknzr.advance().expect("no token");
        assert_eq!(token, Keyword::Let);
    }
    #[test]
    fn test_multiple_tokens() {
        let s = "let do { } \"strings still work\" ;";
        let mut tknzr = Tokenizer::new(s.chars().collect());
        let mut tokens = vec![];
        while let Some(t) = tknzr.advance() {
            tokens.push(t);
        }
        let t2 = vec![
            Token::from(Keyword::Let),
            Token::from(Keyword::Do),
            Token::from('{'),
            Token::from('}'),
            Token::StringConstant(String::from("strings still work")),
            Token::from(';'),
        ];
        for i in 0..6 {
            assert_eq!(&tokens[i], &t2[i]);
        }
    }
}
