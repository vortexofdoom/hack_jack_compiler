use crate::tokens::*;
use std::error::Error;
use regex::Regex;

pub struct Tokenizer {

}

pub fn create_token(s: &str) -> Result<Token, Box<dyn Error>> {
    if let Ok(n) = s.parse::<i16>() {
        if n >= 0 {
            return Token::IntConst(n)
        }
    }
    return Token::StringConst(String::from("Hi"));
}

pub fn parse(code: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut in_string = false;
    let mut in_word = false;
    let mut in_num = false;
    let mut in_comment = false;
    let mut multiline_comment = false;
    let mut str_start = 0;
    let mut word_start = 0;
    let mut num_start = 0;
    let mut tokens = vec![];
    let mut chars = code.chars().enumerate().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => {
                if !in_string {
                    (in_string, str_start) = (true, i + 1);
                } else {
                    tokens.push(Token::StringConst(code[str_start..i].to_string()));
                    in_string = false;
                }
            }
            '/' => {
                if !in_comment {
                    if let Some((_, '/')) = chars.peek() {
                        (in_comment, multiline_comment) = (true, false);
                    }
                    if let Some((_, '*')) = chars.peek() {
                        (in_comment, multiline_comment) = (true, true);
                    }
                    else {
                        tokens.push(Token::Symbol('/'));
                    }
                }
            }
            '*' => {
                if in_comment && multiline_comment {
                    if let Some(_) = chars.next_if_eq(&(i + 1, '/')) {
                        (in_comment, multiline_comment) = (false, false);
                    }
                } else {
                    tokens.push(Token::Symbol('*'));
                }
            }
            '_' => {}
            '\n' => {}
            _ => {},
        }
    }
    for (i, c) in chars {
        if c == '"' {
            if !in_string {
                str_start = i + 1;
                in_string = true;
            } else {
                
            }
        } else if in_string {
            continue;
        } else if c == ' ' {
            if in_word {
                in_word = false;
                let word = &code[word_start..i];
                if let Some(k) = KEYWORDS.get(word) {
                    tokens.push(Token::Keyword(*k));
                }
            }
            if in_num {
                in_num = false;
                let n = (&code[num_start..i]).parse::<i16>()?;
                tokens.push(Token::IntConst(n));
                
            }
        } else {
            if c == '/' {
                if !in_comment {
                    if &code[i..=i+1] == "//" {
                        in_comment = true;
                        multiline_comment = false;
                    } else if &code[i..=i+1] == "/*" {
                        in_comment = true;
                        multiline_comment = true;
                    } else {
                        tokens.push(Token::Symbol(c));
                    }
                } else if multiline_comment {

                }
            } else if c == '\n' {
                if in_comment && !multiline_comment {
                    in_comment = false;
                }
            }
            if c.is_alphabetic() {
                if !in_word {
                    in_word = true;
                    word_start = i;
                }
            } else if c.is_numeric() {
                if !in_word && !in_num {
                    in_num = true;
                    num_start = i;
                }
            }
            if SYMBOLS.contains(&c) {
                tokens.push(Token::Symbol(c));
            }
        }
    }
    Ok(tokens)
}
