use crate::tokens::{Token::{*, self}, Keyword::{*, self}, TokenType};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    iter::Peekable,
};

pub struct CompilationEngine<'a> {
    writer: BufWriter<File>,
    tokens: Peekable<std::slice::Iter<'a, Token>>,
    errors: Vec<CompilationError>,
    names: Names,
}

pub enum CompilationError {
    DuplicateIdentifier,
    UnexpectedToken(Token),
    UnrecognizedIdentifier,
    UnexpectedEndOfTokens,
}

pub struct Names {
    classes: HashSet<String>,
    class_vars: HashSet<String>,
    subroutines: HashSet<String>,
    vars: HashSet<String>,
}

impl Names {
    pub fn new() -> Self {
        Names {
            classes: HashSet::new(),
            class_vars: HashSet::new(),
            subroutines: HashSet::new(),
            vars: HashSet::new(),
        }
    }
}

impl<'a> CompilationEngine<'a> {
    pub fn compile(filename: &str, tokens: &'a [Token]) -> Result<(), Vec<CompilationError>> {
        let output = File::create(Path::new(filename).with_extension("xml"))
            .expect("failed to create file");
        let mut engine = CompilationEngine {
            writer: BufWriter::new(output),
            tokens: tokens.iter().peekable(),
            errors: vec![],
            names: Names::new(),
        };
        write!(engine.writer, "<tokens>").expect("failed to start writing tokens");
        engine.construct_class();
        write!(engine.writer, "</tokens>").expect("failed to finish writing tokens");
        if engine.errors.len() != 0 {
            Err(engine.errors)
        } else {
            Ok(())
        }

    }

    // fn parse_token(&mut self, token: &Token) -> TokenType {
    //     match token {
    //         Token::Keyword(k) => match k {
    //             Class => TokenType::Terminal(token),
    //             Constructor | Function | Method => TokenType::Subroutine,
    //             Field | Static => TokenType::ClassVar,
    //             Var => TokenType::Var,
    //             Int | Char | Boolean | Void => TokenType::Type,
    //             True | False | Null | This => TokenType::Constant,
    //             Let | Do | If | Else | While | Return => TokenType::Statement,
    //         },
    //         Token::Symbol(_) => TokenType::Terminal(token),
    //         Token::Identifier(s) => {
    //             if self.class_name == *s {
    //                 TokenType::Type
    //             } else if self.class_vars.contains(s) {
    //                 TokenType::VarName
    //             } else if self.subroutine_names.contains(s) {
    //                 TokenType::SubroutineName
    //             } else if self.vars.contains(s) {
    //                 TokenType::VarName
    //             } else {
    //                 TokenType::Terminal(token)
    //             }
    //         },
    //         Token::IntConst(_) => todo!(),
    //         Token::StringConst(_) => todo!(),
    //     }
    // }

    fn consume(&mut self, token: &Token) {
        if let Some(t) = self.tokens.next() {
            if token == t {
                writeln!(self.writer, "{t}").expect("failed to write token");
            } else {
                self.errors.push(CompilationError::UnexpectedToken(t.clone()));
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
    }

    fn check_for_type(&mut self) {
        if let Some(&t) = self.tokens.peek() {
            match t {
                Keyword(Int) | Keyword(Char) | Keyword(Boolean) => self.consume(t),
                Identifier(s) => {
                    if self.names.classes.contains(s) {
                        self.consume(t);
                    }
                }
                _ => self.errors.push(CompilationError::UnexpectedToken(*t)),
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
    }

    fn check_for_duplicate(&mut self, set: &mut HashSet<String>) {
        if let Some(Identifier(s)) = self.tokens.peek() {
            if set.insert(*s) {
                self.consume(self.tokens.next().unwrap());
            } else {
                self.errors.push(CompilationError::DuplicateIdentifier);
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
    }
    
    fn construct_class(&mut self) {
        writeln!(self.writer, "<class>").expect("failed to start writing class");
        self.consume(&Token::Keyword(Class));
        self.check_for_duplicate(&mut self.names.classes);
        self.consume(&Token::Symbol('{'));
        while let Some(Token::Keyword(k)) = self.tokens.peek() {
            match k {
                Static | Field => self.compile_class_var_dec(*k),
                _ => self.errors.push(CompilationError::UnexpectedToken(Keyword(*k))),
            }
        }
        while let Some(Keyword(k)) = self.tokens.peek() {
            match k {
                Constructor | Function | Method => self.compile_subroutine_dec(*k),
                _ => self.errors.push(CompilationError::UnexpectedToken(Keyword(*k))),
            }
        }
        self.consume(&Token::Symbol('}'));
        write!(self.writer, "</class>").expect("failed to finish writing class");
    }

    fn compile_class_var_dec(&mut self, keyword: Keyword) {
        writeln!(self.writer, "<classVarDec>").expect("failed to start class var declaration");
        self.consume(&Token::Keyword(keyword));
        self.check_for_type();
        self.check_for_duplicate(&mut self.names.class_vars);
        writeln!(self.writer, "</classVarDec>").expect("failed to finish class var declaration");
    }

    fn compile_subroutine_dec(&mut self, keyword: Keyword) {
        writeln!(self.writer, "<classVarDec>").expect("failed to start class var declaration");
        self.consume(&Token::Keyword(keyword));
        self.check_for_type();
        self.check_for_duplicate(&mut self.names.subroutines);
        self.consume(&Token::Symbol('('));
        if self.tokens.peek() != Some(&&Token::Symbol(')')) {
            self.compile_parameter_list();
        } 
        self.consume(&Token::Symbol(')'));
        self.compile_subroutine_body();
        writeln!(self.writer, "</classVarDec>").expect("failed to finish class var declaration");
    }

    fn compile_parameter_list(&mut self) {
        self.names.vars.clear();
        while self.tokens.peek() != Some(&&Token::Symbol(')')) {
            self.check_for_type();
            self.check_for_duplicate(&mut self.names.vars);
            if self.tokens.peek() == Some(&&Token::Symbol(',')) {
                self.consume(&Token::Symbol(','));
            }
        }
    }

    fn compile_subroutine_body(&mut self) {
        self.names.vars.clear();
        writeln!(self.writer, "<subroutineBody>").expect("failed to start subroutine body");
        self.consume(&Token::Symbol('{'));
        while self.tokens.peek() == Some(&&Token::Keyword(Var)) {
            self.compile_var_dec();
        }
        self.compile_statements();
        self.consume(&Token::Symbol('}'));
        writeln!(self.writer, "</subroutineBody>").expect("failed to finish subroutine body");
    }

    fn compile_var_dec(&mut self) {
        writeln!(self.writer, "<varDec>").expect("failed to start var declaration");
        self.consume(&Token::Keyword(Var));
        self.check_for_type();
        if let Some(Identifier(s)) = self.tokens.peek() {
            self.check_for_duplicate(&mut self.names.vars);
        }
        self.consume(&Token::Symbol(';'));
        writeln!(self.writer, "</varDec>").expect("failed to finish var declaration");
    }

    fn compile_statements(&mut self) {
        writeln!(self.writer, "<statements>").expect("failed to start statements");
        while let Some(&t) = self.tokens.peek() {
            match t {
                Keyword(Let) => self.compile_let(),
                Keyword(If) => self.compile_if(),
                Keyword(While) => self.compile_while(),
                Keyword(Do) => self.compile_do(),
                Keyword(Return) => self.compile_return(),
                Symbol('}') => break,
                _ => self.errors.push(CompilationError::UnexpectedToken(t.clone())),
            }
        }
        writeln!(self.writer, "<statements>").expect("failed to finish statements");
    }
    
    fn compile_let(&mut self) {
        writeln!(self.writer, "<letStatement>").expect("failed to start let statement");
        self.consume(&Token::Keyword(Let));
        if let Some(Token::Identifier(s)) = self.tokens.peek() {
            if self.names.vars.contains(s) || self.names.class_vars.contains(s) {
                self.consume(&Token::Identifier(*s));
            }
        }
        if let Some(Token::Symbol('[')) = self.tokens.peek() {
            self.consume(&Token::Symbol('['));
            self.compile_expression();
            self.consume(&Token::Symbol(']'));
        }
        self.consume(&Token::Symbol('='));
        self.compile_expression();
        self.consume(&Token::Symbol(';'));
        writeln!(self.writer, "</letStatement>").expect("failed to finish let statement");
    }

    fn compile_while(&mut self) {
        writeln!(self.writer, "<whileStatement>").expect("failed to start if statement");
        self.consume(&Token::Keyword(While));
        self.consume(&Token::Symbol('('));
        self.compile_expression();
        self.consume(&Token::Symbol(')'));
        self.consume(&Token::Symbol('{'));
        self.compile_statements();
        self.consume(&Token::Symbol('}'));
        writeln!(self.writer, "</whileStatement>").expect("failed to finish if statement");
    }

    fn compile_if(&mut self) {
        writeln!(self.writer, "<ifStatement>").expect("failed to start if statement");
        self.consume(&Token::Keyword(If));
        self.consume(&Token::Symbol('('));
        self.compile_expression();
        self.consume(&Token::Symbol(')'));
        self.consume(&Token::Symbol('{'));
        self.compile_statements();
        self.consume(&Token::Symbol('}'));
        if self.tokens.peek() == Some(&&Token::Keyword(Else)) {
            self.consume(&Token::Keyword(Else));
            if let Some(&t) = self.tokens.peek() {
                match t {
                    Keyword(If) => self.compile_if(),
                    Symbol('{') => {
                        self.consume(t);
                        self.compile_statements();
                        self.consume(&Token::Symbol('}'));
                    }
                    _ => self.errors.push(CompilationError::UnexpectedToken(*t)),
                }
            } else {
                self.errors.push(CompilationError::UnexpectedEndOfTokens);
            }
        }
        writeln!(self.writer, "</ifStatement>").expect("failed to finish if statement");
    }

    fn compile_do(&mut self) {
        writeln!(self.writer, "<doStatement>").expect("failed to start do statement");
        self.consume(&Token::Keyword(Do));
        if let Some(&t) = self.tokens.peek() {
            match t {
                Identifier(s) => self.compile_subroutine_call(s),
                _ => self.errors.push(CompilationError::UnexpectedToken(*t)),
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
        writeln!(self.writer, "</doStatement>").expect("failed to finish do statement");
    }

    fn compile_return(&mut self) {
        writeln!(self.writer, "<returnStatement>").expect("failed to start return statement");
        self.consume(&Token::Keyword(Return));
        if self.tokens.peek() != Some(&&Symbol(';')) {
            self.compile_expression();
        } else {
            
        }
        self.consume(&Token::Symbol(';'));
        writeln!(self.writer, "</returnStatement>").expect("failed to finish return statement");
    }

    fn compile_subroutine_call(&mut self, name: &str) {
        if let Some(s) = self.names.subroutines.get(name) {

        }
        if self.names.subroutines.contains(name) {
            self.consume();
            self.consume(&Token::Symbol('('));
            self.compile_expression_list();
            self.consume(&Token::Symbol(')'));
        } else {
            self.errors.push(CompilationError::UnrecognizedIdentifier);
        }
    }

    fn compile_term(&mut self) {
        writeln!(self.writer, "<term>").expect("failed to start term");
        if let Some(Symbol(c)) = self.tokens.peek() {
            match c {
                '-' | '~' => self.consume(&Token::Symbol(*c)),
                _ => self.errors.push(CompilationError::UnexpectedToken(Token::Symbol(*c))),
            }
        }
        if let Some(&t) = self.tokens.peek() {
            match t {
                Token::Keyword(True)| Token::Keyword(False) | Token::Keyword(This) | Token::Keyword(Null)
                | IntConst(_) | StringConst(_) => self.consume(t),
                Identifier(s) => {
                    if self.names.vars.contains(s) | self.names.class_vars.contains(s) {
                        self.consume(&Token::Identifier(*s));
                        if let Some(Symbol(c)) = self.tokens.peek() {
                            match c {
                                '[' => {
                                    self.consume(&Token::Symbol('['));
                                    self.compile_expression();
                                    self.consume(&Token::Symbol(']'));
                                }
                                '.' => {
                                    if let Some(Identifier(s)) = self.tokens.peek() {
                                        self.compile_subroutine_call(s);
                                    }
                                }
                            }
                        }
                    } else if self.names.classes.contains(s) {
                        self.consume(&Token::Symbol('.'));
                        if let Some(Token::Identifier(s)) = self.tokens.peek() {
                            self.compile_subroutine_call(s);
                        }
                    } else {
                        self.compile_subroutine_call(s);
                    }
                },
                _ => self.errors.push(CompilationError::UnexpectedToken(t.clone()))
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
        writeln!(self.writer, "</term>").expect("failed to finish term");
    }

    fn compile_expression(&mut self) {
        writeln!(self.writer, "<expression>").expect("failed to start expression");
        self.compile_term();
        if let Some(Token::Symbol(c)) = self.tokens.peek() {
            match c {
                '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' => {
                    self.consume(&Token::Symbol(*c));
                    self.compile_term();
                }
                _ => self.errors.push(CompilationError::UnexpectedToken(Symbol(*c))),
            }
        } else {
            self.errors.push(CompilationError::UnexpectedEndOfTokens);
        }
        writeln!(self.writer, "</expression>").expect("failed to finish expression");
    }

    fn compile_expression_list(&mut self) {
        writeln!(self.writer, "<expressionList>").expect("failed to start expression list");
        while self.tokens.peek() != Some(&&Token::Symbol(')')) {
            self.compile_expression();
            if self.tokens.peek() == Some(&&Token::Symbol(',')) {
                self.consume(&Token::Symbol(','));
            }
        }
        writeln!(self.writer, "</expressionList>").expect("failed to finish expression list");
    }
}

