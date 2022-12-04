use crate::{
    names::*,
    token_type::{TokenType, ValidToken},
    tokenizer::Tokenizer,
    tokens::{
        Keyword::{self, *},
        Token,
    },
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

pub struct Parser {
    writer: Option<BufWriter<File>>,
    tokenizer: Tokenizer,
    curr_token: Option<Token>,
    errors: Vec<(CompilationError, Option<Token>)>,
    names: NameSet,
}

#[derive(Debug, Clone)]
pub enum CompilationError {
    DuplicateIdentifier,
    UnexpectedToken,
    UnrecognizedIdentifier,
    UnexpectedEndofTokens,
}

use crate::token_type::TokenType::*;
impl Parser {
    pub fn new() -> Self {
        Parser {
            writer: None,
            tokenizer: Tokenizer::default(),
            curr_token: None,
            errors: vec![],
            names: NameSet::new(),
        }
    }

    pub fn throw_error(&mut self, err: CompilationError) {
        let token = self.curr_token.as_ref();
        self.errors.push((err, Option::<&Token>::cloned(token)));
        self.curr_token = self.tokenizer.advance();
    }

    pub fn curr_token_is<T: ValidToken + PartialEq<Token>>(&self, other: T) -> bool {
        if let Some(t) = self.curr_token.as_ref() {
            other == *t
        } else {
            false
        }
    }

    pub fn compile(&mut self, file: PathBuf) -> Result<(), Vec<(CompilationError, Option<Token>)>> {
        let filename = file
            .as_path()
            .file_stem()
            .expect("could not find file name")
            .to_str()
            .expect("could not conver to str");
        let tokenizer = Tokenizer::new(std::fs::read_to_string(&file).expect("failed to read"));
        let output =
            File::create(Path::new(filename).with_extension("xml")).expect("failed to create file");
        self.writer = Some(BufWriter::new(output));
        self.tokenizer = tokenizer;
        self.curr_token = self.tokenizer.advance();
        // //temp
        // while let Some(_) = self.curr_token {
        //     self.curr_token = self.tokenizer.advance();
        //     println!("{}", self.curr_token.as_ref().unwrap());
        // }
        // Ok(())
        writeln!(self.writer.as_mut().expect("no writer"), "<tokens>")
            .expect("failed to start writing tokens");
        self.construct_class();
        write!(self.writer.as_mut().expect("no writer"), "</tokens>")
            .expect("failed to finish writing tokens");
        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    fn consume<T: ValidToken + PartialEq<Token> + Copy>(&mut self, requested: T) {
        if self.curr_token_is(requested) {
            writeln!(
                self.writer.as_mut().expect("no writer"),
                "{}",
                self.curr_token.as_ref().unwrap()
            )
            .expect("failed to write token");
            self.curr_token = self.tokenizer.advance();
        } else {
            self.throw_error(CompilationError::UnexpectedToken);
        }
    }

    // fn check_for_duplicate(&mut self, set: Names) {
    //     let name_set = self.names.get(set);
    //     if let Some(Token::Identifier(s)) = self.curr_token {
    //         if name_set.insert(s.clone()) {
    //             self.consume(TokenType::Name);
    //         } else {
    //             self.throw_error(CompilationError::DuplicateIdentifier);
    //         }
    //     } else {
    //         self.throw_error(CompilationError::UnexpectedToken);
    //     }
    // }

    fn construct_class(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<class>")
            .expect("failed to start writing class");
        write!(self.writer.as_mut().expect("no writer"), "\t")
            .expect("failed to write");
        self.consume(Class);
        self.consume(TokenType::Name);
        // the world is not yet ready for validation, let's get parsing handled first
        // match self
        //     .names
        //     .check_duplicate(Names::Vars, self.curr_token.as_ref())
        // {
        //     Ok(()) => self.consume(Name),
        //     Err(e) => self.throw_error(e),
        // }
        self.consume('{');
        while self.curr_token_is(TokenType::ClassVarDec) {
            self.compile_class_var_dec();
        }
        while self.curr_token_is(TokenType::SubroutineDec) {
            self.compile_subroutine_dec();
        }
        self.consume('}');
        writeln!(self.writer.as_mut().expect("no writer"), "</class>")
            .expect("failed to finish writing class");
    }

    fn compile_class_var_dec(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<classVarDec>")
            .expect("failed to start class var declaration");
        write!(self.writer.as_mut().expect("no writer"), "\t")
            .expect("failed to write");
        self.consume(TokenType::ClassVarDec);
        self.consume(TokenType::Type);
        //placeholder
        self.consume(TokenType::Name);
        // match self
        //     .names
        //     .check_duplicate(Names::ClassVars, self.curr_token.as_ref())
        // {
        //     Ok(()) => self.consume(TokenType::Name),
        //     Err(e) => self.throw_error(e),
        // }
        while self.curr_token_is(',') {
            self.consume(',');
            self.consume(TokenType::Name);
        }
        self.consume(';');
        writeln!(self.writer.as_mut().expect("no writer"), "</classVarDec>")
            .expect("failed to finish class var declaration");
    }

    fn compile_subroutine_dec(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<classVarDec>")
            .expect("failed to start class var declaration");
        self.consume(ClassVarDec);
        self.consume(TokenType::ReturnType);
        self.consume(TokenType::Name);
        // match self
        //     .names
        //     .check_duplicate(Names::Subroutines, self.curr_token.as_ref())
        // {
        //     Ok(()) => self.consume(TokenType::Name),
        //     Err(e) => self.throw_error(e),
        // }
        self.consume('(');
        if !self.curr_token_is(')') {
            self.compile_parameter_list();
        }
        self.consume(')');
        self.compile_subroutine_body();
        writeln!(self.writer.as_mut().expect("no writer"), "</classVarDec>")
            .expect("failed to finish class var declaration");
    }

    fn compile_parameter_list(&mut self) {
        self.names.get_mut(Names::Vars).clear();
        while !self.curr_token_is(')') {
            self.consume(TokenType::Type);
            //self.check_for_duplicate(Names::Vars);
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
    }

    fn compile_subroutine_body(&mut self) {
        self.names.get_mut(Names::Vars).clear();
        writeln!(self.writer.as_mut().expect("no writer"), "<subroutineBody>")
            .expect("failed to start subroutine body");
        self.consume('{');
        while self.curr_token_is(Keyword::Var) {
            self.compile_var_dec();
        }
        self.compile_statements();
        self.consume('}');
        writeln!(
            self.writer.as_mut().expect("no writer"),
            "</subroutineBody>"
        )
        .expect("failed to finish subroutine body");
    }

    fn compile_var_dec(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<varDec>")
            .expect("failed to start var declaration");
        self.consume(Var);
        self.consume(TokenType::Type);
        self.consume(TokenType::Name);
        // match self
        //     .names
        //     .check_duplicate(Names::Vars, self.curr_token.as_ref())
        // {
        //     Ok(()) => self.consume(Name),
        //     Err(e) => self.throw_error(e),
        // }
        self.consume(';');
        writeln!(self.writer.as_mut().expect("no writer"), "</varDec>")
            .expect("failed to finish var declaration");
    }

    fn compile_statements(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<statements>")
            .expect("failed to start statements");
        while self.curr_token_is(TokenType::Statement) {
            match self.curr_token.as_ref() {
                Some(Token::Keyword(Let)) => self.compile_let(),
                Some(Token::Keyword(If)) => self.compile_if(),
                Some(Token::Keyword(While)) => self.compile_while(),
                Some(Token::Keyword(Do)) => self.compile_do(),
                Some(Token::Keyword(Return)) => self.compile_return(),
                _ => break,
            }
        }
        writeln!(self.writer.as_mut().expect("no writer"), "</statements>")
            .expect("failed to finish statements");
    }

    fn compile_let(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<letStatement>")
            .expect("failed to start let statement");
        self.consume(Let);
        self.consume(TokenType::Name);
        // let token = self.curr_token.as_ref();
        // if self.names.is_valid(Names::Classes, token)
        //     || self.names.is_valid(Names::Subroutines, token)
        // {
        //     self.throw_error(CompilationError::UnexpectedToken);
        // } else if !self.names.contains(token) {
        //     self.throw_error(CompilationError::UnrecognizedIdentifier);
        // } else {
        //     self.consume(TokenType::Name);
        // }
        if self.curr_token_is('[') {
            self.consume('[');
            self.compile_expression();
            self.consume(']');
        }
        self.consume('=');
        self.compile_expression();
        self.consume(';');
        writeln!(self.writer.as_mut().expect("no writer"), "</letStatement>")
            .expect("failed to finish let statement");
    }

    fn compile_while(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<whileStatement>")
            .expect("failed to start if statement");
        self.consume(While);
        self.consume('(');
        self.compile_expression();
        self.consume(')');
        self.consume('{');
        self.compile_statements();
        self.consume('}');
        writeln!(
            self.writer.as_mut().expect("no writer"),
            "</whileStatement>"
        )
        .expect("failed to finish if statement");
    }

    fn compile_if(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<ifStatement>")
            .expect("failed to start if statement");
        self.consume(If);
        self.consume('(');
        self.compile_expression();
        self.consume(')');
        self.consume('{');
        self.compile_statements();
        self.consume('}');
        if self.curr_token_is(Else) {
            self.consume(Else);
            if self.curr_token_is(If) {
                self.compile_if();
            } else {
                self.consume('{');
                self.compile_statements();
                self.consume('}');
            }
        }
        writeln!(self.writer.as_mut().expect("no writer"), "</ifStatement>")
            .expect("failed to finish if statement");
    }

    fn compile_do(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<doStatement>")
            .expect("failed to start do statement");
        self.consume(Do);
        self.compile_subroutine_call();
        self.consume(';');
        writeln!(self.writer.as_mut().expect("no writer"), "</doStatement>")
            .expect("failed to finish do statement");
    }

    fn compile_return(&mut self) {
        writeln!(
            self.writer.as_mut().expect("no writer"),
            "<returnStatement>"
        )
        .expect("failed to start return statement");
        self.consume(Return);
        if !self.curr_token_is(';') {
            self.compile_expression();
        }
        self.consume(';');
        writeln!(
            self.writer.as_mut().expect("no writer"),
            "</returnStatement>"
        )
        .expect("failed to finish return statement");
    }

    fn compile_subroutine_call(&mut self) {
        self.consume(TokenType::Name);
        if self.curr_token_is('.') {
            self.consume('.');
            self.consume(TokenType::Name);
        }
        self.consume('(');
        if !self.curr_token_is(')') {
            self.compile_expression_list();
        }
        self.consume(')');

        //let token = self.curr_token.as_ref();
        // if self.names.contains(token) {
        //     let subroutine_name = self.names.is_valid(Names::Subroutines, token);
        //     self.consume(TokenType::Name);
        //     if !subroutine_name {
        //         self.consume('.');
        //         if self
        //             .names
        //             .is_valid(Names::Subroutines, self.curr_token.as_ref())
        //         {
        //             self.consume(TokenType::Name);
        //         } else {
        //             self.throw_error(CompilationError::UnexpectedToken);
        //         }
        //     }
        //     self.consume('(');
        //     self.compile_expression_list();
        //     self.consume(')');
        // } else {
        //     self.throw_error(CompilationError::UnrecognizedIdentifier);
        // }
    }

    fn compile_term(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<term>").expect("failed to start term");
        if self.curr_token_is('(') {
            self.consume('(');
            self.compile_expression();
            self.consume(')');
        } else {
            if self.curr_token_is(TokenType::UnaryOp) {
                self.consume(TokenType::UnaryOp);
            }
            if self.curr_token_is(TokenType::Constant) {
                self.consume(Constant);
            } else {
                self.consume(TokenType::Name);
                if self.curr_token_is('(') {
                    self.consume('(');
                    self.compile_expression_list();
                    self.consume(')');
                } else if self.curr_token_is('.') {
                    self.consume('.');
                    self.consume(TokenType::Name);
                    self.consume('(');
                    self.compile_expression_list();
                    self.consume(')');
                } else if self.curr_token_is('[') {
                    self.consume('[');
                    self.compile_expression();
                    self.consume(']');
                }
            }
            // } else {
            //     let token = self.curr_token.as_ref();
            //     if self.names.is_valid(Names::Classes, token)
            //         || self.names.is_valid(Names::Classes, token)
            //     {
            //         self.compile_subroutine_call()
            //     } else {
            //         self.consume(TokenType::Name);
            //         if self.curr_token_is('[') {
            //             self.consume('[');
            //             self.compile_expression();
            //             self.consume(']');
            //         }
            //     }
            // }
        }
        writeln!(self.writer.as_mut().expect("no writer"), "</term>")
            .expect("failed to finish term");
    }

    fn compile_expression(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<expression>")
            .expect("failed to start expression");
        self.compile_term();
        if self.curr_token_is(TokenType::BinaryOp) {
            self.consume(TokenType::BinaryOp);
            self.compile_term();
        }
        writeln!(self.writer.as_mut().expect("no writer"), "</expression>")
            .expect("failed to finish expression");
    }

    fn compile_expression_list(&mut self) {
        writeln!(self.writer.as_mut().expect("no writer"), "<expressionList>")
            .expect("failed to start expression list");
        while !self.curr_token_is(')') {
            self.compile_expression();
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
        writeln!(
            self.writer.as_mut().expect("no writer"),
            "</expressionList>"
        )
        .expect("failed to finish expression list");
    }
}
