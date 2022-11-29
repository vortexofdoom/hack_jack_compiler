use crate::tokens::*;

#[derive(Debug)]
pub enum TokenizerError {
    InvalidInt,
    UnrecognizedToken,
}

impl From<std::num::ParseIntError> for TokenizerError {
    fn from(_: std::num::ParseIntError) -> Self {
        TokenizerError::InvalidInt
    }
}

pub fn tokenize(code: String) -> Result<Vec<Token>, TokenizerError> {
    let mut in_string = false;
    let mut in_comment = false;
    let mut multiline_comment = false;
    let mut str_start = 0;
    let mut tokens = vec![];
    let mut chars = code.chars().enumerate().peekable();
    while let Some((i, c)) = chars.next() {
        if in_comment {
            if multiline_comment {
                if c == '*' {
                    if let Some((_, '/')) = chars.next() {
                        in_comment = false;
                    }
                }
            } else if c == '\n' {
                in_comment = false;
            }
        } else if in_string {
            if c == '"' {
                tokens.push(Token::StringConst(String::from(&code[str_start..i])));
                in_string = false;
            }
        } else if c.is_numeric() {
            let start = i;
            let mut end = start;
            while let Some((j, _)) = chars.next_if(|(_, x)| x.is_numeric()) {
                end = j;
            }
            let n = code[start..=end].parse::<i16>()?;
            tokens.push(Token::IntConst(n));
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            let mut end = start;
            while let Some((j, _)) = chars.next_if(|(_, x)| x.is_alphanumeric() || *x == '_') {
                end = j;
            }
            let word = &code[start..=end];
            if let Some(k) = KEYWORDS.get(word) {
                tokens.push(Token::Keyword(*k));
            } else {
                tokens.push(Token::Identifier(String::from(word)));
            }
        } else if SYMBOLS.contains(&c) {
            if c == '/' {
                if !in_comment && !in_string {
                    if let Some((_, comment_start)) = chars.next_if(|(_, x)| *x == '/' || *x == '*') {
                        in_comment = true;
                        multiline_comment = comment_start == '*';
                    } else {
                        tokens.push(Token::Symbol('/'));
                    }
                }
            } else if c == '"' {
                if !in_string {
                    (in_string, str_start) = (true, i + 1);
                }
            } else {
                tokens.push(Token::Symbol(c));
            }
        } else if !c.is_whitespace() {
                return Err(TokenizerError::UnrecognizedToken);
        }
    }
    Ok(tokens)
}
