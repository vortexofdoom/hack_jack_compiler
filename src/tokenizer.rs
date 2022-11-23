use crate::tokens::*;
use std::error::Error;

fn create_token<T>(data: T) -> Result<Token, Box<dyn Error>>
where T: char + str {
    let n = u16::try_from(data)
    Ok(Token::IntConst(0))
}

pub fn parse(code: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut in_string = false;
    let mut in_word = false;
    let mut in_comment = false;
    let mut multiline_comment = false;
    let mut str_start = 0;
    let mut word_start = 0;
    let mut tokens = vec![];
    let mut chars = code.chars().enumerate().peekable();
    while let Some((i, c)) = chars.next() {
        if in_comment {
            if multiline_comment {
                if c == '*' {
                    if let Some((_, c)) = chars.next() {
                        in_comment = c == '/';
                        multiline_comment = in_comment;
                    }
                }
            } else {
                if c == '\n' {
                    in_comment = false;
                }
            }
        } else if in_string {
            if c == '"' {
                tokens.push(create_token(&code[str_start..i])?);
                in_string = false;
            }
        } else {
            if c.is_numeric() && !in_word {
                let start = i;
                let mut end = start;
                while let Some((j, _)) = chars.next_if(|(_, x)| x.is_numeric()) {
                    end = j;
                }
                tokens.push(create_token(&code[start..=end])?);
            } else if c.is_alphabetic() && !in_word {
                (in_word, word_start) = (true, i + 1);
            } else {
                match c {
                    '"' => {
                        if !in_string {
                            (in_string, str_start) = (true, i + 1);
                        }
                    }
                    '/' => {
                        if !in_comment && !in_string {
                            if let Some((_, '/')) | Some((_, '*')) = chars.peek() {
                                in_comment = true;
                                multiline_comment = chars.next().unwrap().1 == '*'; 
                            } else {
                                tokens.push(create_token('/')?);
                            }
                        }
                    }
                    '*' => {
                        if in_comment && multiline_comment {
                            if let Some(_) = chars.next_if_eq(&(i + 1, '/')) {
                                (in_comment, multiline_comment) = (false, false);
                            }
                        } else {
                            tokens.push(create_token(c)?);
                        }
                    }
                    ' ' => {
                        if in_word {
                            in_word = false;
                            let word = &code[word_start..i];
                            if let Some(k) = KEYWORDS.get(word) {
                                tokens.push(Token::Keyword(*k));
                            }
                        }
                    }
                    '\n' => {}
                    _ => {
                        if SYMBOLS.contains(&c) {
                            tokens.push(Token::Symbol(c));
                        }
                    },
                }
            }
        }
    }
    Ok(tokens)
}
