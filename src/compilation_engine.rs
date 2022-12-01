use crate::{
    names::*,
    tokenizer::Tokenizer,
    tokens::{
        Keyword::{self, *},
        Token,
    },
    validation::{TokenType, ValidToken},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub struct CompilationEngine {
    writer: BufWriter<File>,
    tokenizer: Tokenizer,
    curr_token: Box<Token>,
    errors: Vec<CompilationError>,
    names: Names,
}
pub enum CompilationError {
    DuplicateIdentifier,
    UnexpectedToken(Token),
    UnrecognizedIdentifier,
    UnexpectedEndOfTokens,
}

use crate::validation::TokenType::*;
impl CompilationEngine {
    pub fn compile(filename: &str, mut tokenizer: Tokenizer) -> Result<(), Vec<CompilationError>> {
        let output =
            File::create(Path::new(filename).with_extension("xml")).expect("failed to create file");
        let token = tokenizer.next().expect("unexpected end of tokens");
        let mut engine = CompilationEngine {
            writer: BufWriter::new(output),
            tokenizer: tokenizer,
            curr_token: Box::new(token),
            errors: vec![],
            names: Names::new(),
        };
        writeln!(engine.writer, "<tokens>").expect("failed to start writing tokens");
        engine.construct_class();
        write!(engine.writer, "</tokens>").expect("failed to finish writing tokens");
        if !engine.errors.is_empty() {
            Err(engine.errors)
        } else {
            Ok(())
        }
    }

    fn consume<T: PartialEq + ValidToken + TryFrom<Box<Token>>>(&mut self, token: T) {
        let valid: bool;
        if let Ok(t) = <Box<Token> as TryInto<T>>::try_into(self.curr_token)  {
            if t == token {
                writeln!(self.writer, "{}", self.curr_token).expect("failed to write token");
                if let Some(t) = self.tokenizer.next() {
                    *self.curr_token = t;
                } else {
                    self.errors.push(CompilationError::UnexpectedEndOfTokens);
                }
            }
        } else {
            self.errors
                .push(CompilationError::UnexpectedToken(*self.curr_token));
        }
    }

    fn check_for_duplicate(&mut self, set: NameSet) {
        let name_set = self.names.get(set);
        if let Some(Token::Identifier(s)) = self.tokenizer.peek() {
            if name_set.insert(s.to_owned()) {
                self.consume(TokenType::Name(set));
            } else {
                self.errors.push(CompilationError::DuplicateIdentifier);
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
    }

    fn construct_class(&mut self) {
        writeln!(self.writer, "<class>").expect("failed to start writing class");
        self.consume(Class);
        self.check_for_duplicate(NameSet::Classes);
        self.consume('{');
        while let Some(Token::Keyword(k)) = self.tokenizer.peek() {
            match k {
                Static | Field => self.compile_class_var_dec(*k),
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(Token::Keyword(*k))),
            }
        }
        while let Some(Token::Keyword(k)) = self.tokenizer.peek() {
            match k {
                Constructor | Function | Method => self.compile_subroutine_dec(*k),
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(Token::Keyword(*k))),
            }
        }
        self.consume('}');
        writeln!(self.writer, "</class>").expect("failed to finish writing class");
    }

    fn compile_class_var_dec(&mut self, keyword: Keyword) {
        writeln!(self.writer, "<classVarDec>").expect("failed to start class var declaration");
        self.consume(keyword);
        self.consume(TokenType::Type);
        self.check_for_duplicate(NameSet::ClassVars);
        writeln!(self.writer, "</classVarDec>").expect("failed to finish class var declaration");
    }

    fn compile_subroutine_dec(&mut self, keyword: Keyword) {
        writeln!(self.writer, "<classVarDec>").expect("failed to start class var declaration");
        self.consume(ClassVarDec);
        self.check_for_type();
        self.check_for_duplicate(NameSet::Subroutines);
        self.consume('(');
        if self.tokenizer.peek() != &Some(Token::Symbol(')')) {
            self.compile_parameter_list();
        }
        self.consume(')');
        self.compile_subroutine_body();
        writeln!(self.writer, "</classVarDec>").expect("failed to finish class var declaration");
    }

    fn compile_parameter_list(&mut self) {
        self.names.get(NameSet::Vars).clear();
        while self.tokenizer.peek() != &Some(Token::Symbol(')')) {
            self.check_for_type();
            self.check_for_duplicate(NameSet::Vars);
            if self.tokenizer.peek() == &Some(Token::Symbol(',')) {
                self.consume(',');
            }
        }
    }

    fn compile_subroutine_body(&mut self) {
        self.names.get(NameSet::Vars).clear();
        writeln!(self.writer, "<subroutineBody>").expect("failed to start subroutine body");
        self.consume('{');
        while self.tokenizer.peek() == &Some(Token::Keyword(Var)) {
            self.compile_var_dec();
        }
        self.compile_statements();
        self.consume('}');
        writeln!(self.writer, "</subroutineBody>").expect("failed to finish subroutine body");
    }

    fn compile_var_dec(&mut self) {
        writeln!(self.writer, "<varDec>").expect("failed to start var declaration");
        self.consume(Var);
        self.check_for_type();
        if let Some(Token::Identifier(_)) = self.tokenizer.peek() {
            self.check_for_duplicate(NameSet::Vars);
        }
        self.consume(';');
        writeln!(self.writer, "</varDec>").expect("failed to finish var declaration");
    }

    //REFACTOR WITH NEW VALIDATION
    fn compile_statements(&mut self) {
        writeln!(self.writer, "<statements>").expect("failed to start statements");
        while let &Some(t) = self.tokenizer.peek() {
            match t {
                Token::Keyword(Let) => self.compile_let(),
                Token::Keyword(If) => self.compile_if(),
                Token::Keyword(While) => self.compile_while(),
                Token::Keyword(Do) => self.compile_do(),
                Token::Keyword(Return) => self.compile_return(),
                Token::Symbol('}') => break,
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(t.clone())),
            }
        }
        writeln!(self.writer, "<statements>").expect("failed to finish statements");
    }

    fn compile_let(&mut self) {
        writeln!(self.writer, "<letStatement>").expect("failed to start let statement");
        self.consume(Let);
        if let Some(Token::Identifier(s)) = self.tokenizer.peek() {
            if self.names.get(NameSet::Vars).contains(s) || self.names.get(NameSet::ClassVars).contains(s) {
                self.consume(&Token::Identifier(s.clone()));
            }
        }
        if let Some(Token::Symbol('[')) = self.tokenizer.peek() {
            self.consume('[');
            self.compile_expression();
            self.consume(']');
        }
        self.consume('=');
        self.compile_expression();
        self.consume(';');
        writeln!(self.writer, "</letStatement>").expect("failed to finish let statement");
    }

    fn compile_while(&mut self) {
        writeln!(self.writer, "<whileStatement>").expect("failed to start if statement");
        self.consume(While);
        self.consume('(');
        self.compile_expression();
        self.consume(')');
        self.consume('{');
        self.compile_statements();
        self.consume('}');
        writeln!(self.writer, "</whileStatement>").expect("failed to finish if statement");
    }

    fn compile_if(&mut self) {
        writeln!(self.writer, "<ifStatement>").expect("failed to start if statement");
        self.consume(If);
        self.consume('(');
        self.compile_expression();
        self.consume(')');
        self.consume('{');
        self.compile_statements();
        self.consume('}');
        if self.tokenizer.peek() == &Some(Token::Keyword(Else)) {
            self.consume(Else);
            if let Some(t) = self.tokenizer.peek() {
                match t {
                    Token::Keyword(If) => self.compile_if(),
                    Token::Symbol('{') => {
                        self.consume('{');
                        self.compile_statements();
                        self.consume('}');
                    }
                    _ => self
                        .errors
                        .push(CompilationError::UnexpectedToken(t.clone())),
                }
            } else {
                self.errors.push(CompilationError::UnexpectedEndOfTokens);
            }
        }
        writeln!(self.writer, "</ifStatement>").expect("failed to finish if statement");
    }

    fn compile_do(&mut self) {
        writeln!(self.writer, "<doStatement>").expect("failed to start do statement");
        self.consume(Do);
        if let Some(t) = self.tokenizer.peek() {
            match t {
                Token::Identifier(s) => self.compile_subroutine_call(&s.as_str()),
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(t.clone())),
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
        writeln!(self.writer, "</doStatement>").expect("failed to finish do statement");
    }

    fn compile_return(&mut self) {
        writeln!(self.writer, "<returnStatement>").expect("failed to start return statement");
        self.consume(Return);
        if self.tokenizer.peek() != &Some(Token::Symbol(';')) {
            self.compile_expression();
        } else {
        }
        self.consume(';');
        writeln!(self.writer, "</returnStatement>").expect("failed to finish return statement");
    }

    fn compile_subroutine_call(&mut self, name: &str) {
        if self.names.get(NameSet::Subroutines).contains(name) {
            self.consume(Token::Identifier(name.to_string()));
            self.consume('(');
            self.compile_expression_list();
            self.consume(')');
        } else {
            self.errors.push(CompilationError::UnrecognizedIdentifier);
        }
    }

    fn compile_term(&mut self) {
        writeln!(self.writer, "<term>").expect("failed to start term");
        if let Some(Token::Symbol(c)) = self.tokenizer.peek() {
            match c {
                '-' | '~' => self.consume(*c),
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(Token::Symbol(*c))),
            }
        }
        if let Some(t) = self.tokenizer.peek() {
            match t {
                Token::Keyword(True)
                | Token::Keyword(False)
                | Token::Keyword(This)
                | Token::Keyword(Null)
                | Token::IntConst(_)
                | Token::StringConst(_) => self.consume(t),
                Token::Identifier(s) => {
                    if self.names.vars.contains(s) | self.names.class_vars.contains(s) {
                        self.consume(t);
                        if let Some(Token::Symbol(c)) = self.tokenizer.peek() {
                            match c {
                                '[' => {
                                    self.consume(&Token::Symbol('['));
                                    self.compile_expression();
                                    self.consume(&Token::Symbol(']'));
                                }
                                '.' => {
                                    if let Some(Token::Identifier(s)) = self.tokenizer.peek() {
                                        self.compile_subroutine_call(s);
                                    }
                                }
                                _ => self
                                    .errors
                                    .push(CompilationError::UnexpectedToken(Token::Symbol(*c))),
                            }
                        }
                    } else if self.names.classes.contains(s) {
                        self.consume(&Token::Symbol('.'));
                        if let Some(Token::Identifier(s)) = self.tokenizer.peek() {
                            self.compile_subroutine_call(s);
                        }
                    } else {
                        self.compile_subroutine_call(s);
                    }
                }
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(t.clone())),
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
        writeln!(self.writer, "</term>").expect("failed to finish term");
    }

    fn compile_expression(&mut self) {
        writeln!(self.writer, "<expression>").expect("failed to start expression");
        self.compile_term();
        if let Some(Token::Symbol(c)) = self.tokenizer.peek() {
            match c {
                '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' => {
                    self.consume(*c);
                    self.compile_term();
                }
                _ => self
                    .errors
                    .push(CompilationError::UnexpectedToken(Token::Symbol(*c))),
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
        writeln!(self.writer, "</expression>").expect("failed to finish expression");
    }

    fn compile_expression_list(&mut self) {
        writeln!(self.writer, "<expressionList>").expect("failed to start expression list");
        while self.tokenizer.peek() != &Some(Token::Symbol(')')) {
            self.compile_expression();
            if self.tokenizer.peek() == &Some(Token::Symbol(',')) {
                self.consume(',');
            }
        }
        writeln!(self.writer, "</expressionList>").expect("failed to finish expression list");
    }

    fn check_for_type(&mut self) {
        todo!()
    }
}
