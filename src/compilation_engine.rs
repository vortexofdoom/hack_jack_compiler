use crate::{
    names::*,
    tokenizer::Tokenizer,
    tokens::{
        Identifier,
        Keyword::{self, *},
        Token, ValidToken,
    },
    token_type::TokenType,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub struct CompilationEngine {
    writer: BufWriter<File>,
    tokenizer: Tokenizer,
    curr_token: Token,
    errors: Vec<CompilationError>,
    names: NameSet,
}
pub enum CompilationError {
    DuplicateIdentifier,
    UnexpectedToken,
    UnrecognizedIdentifier,
}

use crate::token_type::TokenType::*;
impl CompilationEngine {
    pub fn compile(filename: &str, mut tokenizer: Tokenizer) -> Result<(), Vec<CompilationError>> {
        let output =
            File::create(Path::new(filename).with_extension("xml")).expect("failed to create file");
        let token = tokenizer.next().expect("unexpected end of tokens");
        let mut engine = CompilationEngine {
            writer: BufWriter::new(output),
            tokenizer,
            curr_token: token,
            errors: vec![],
            names: NameSet::new(),
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

    fn consume<T: ValidToken + PartialEq<Token>>(&mut self, requested: T) {
        if requested == self.curr_token {
            writeln!(self.writer, "{}", self.curr_token).expect("failed to write token");
            if let Some(t) = self.tokenizer.next() {
                self.curr_token = t;
            } else {
                self.errors.push(CompilationError::UnexpectedToken);
            }
        } else {
            self.errors.push(CompilationError::UnexpectedToken);
        }
    }

    fn check_for_duplicate(&mut self, set: Names) {
        let name_set = self.names.get(set);
        if let Some(Identifier(s)) = self.curr_token.identifier() {
            if name_set.insert(s.clone()) {
                self.consume(TokenType::Name);
            } else {
                self.errors.push(CompilationError::DuplicateIdentifier);
            }
        } else {
            self.errors.push(CompilationError::UnexpectedToken);
        }
    }

    fn construct_class(&mut self) {
        writeln!(self.writer, "<class>").expect("failed to start writing class");
        self.consume(Class);
        self.check_for_duplicate(Names::Classes);
        self.consume('{');
        while self.curr_token == TokenType::ClassVarDec {
            self.compile_class_var_dec();
        }
        while self.curr_token == TokenType::SubroutineDec {
            self.compile_subroutine_dec();
        }
        self.consume('}');
        writeln!(self.writer, "</class>").expect("failed to finish writing class");
    }

    fn compile_class_var_dec(&mut self) {
        writeln!(self.writer, "<classVarDec>").expect("failed to start class var declaration");
        self.consume(TokenType::ClassVarDec);
        self.consume(TokenType::Type);
        self.check_for_duplicate(Names::ClassVars);
        writeln!(self.writer, "</classVarDec>").expect("failed to finish class var declaration");
    }

    fn compile_subroutine_dec(&mut self) {
        writeln!(self.writer, "<classVarDec>").expect("failed to start class var declaration");
        self.consume(ClassVarDec);
        self.check_for_type();
        self.check_for_duplicate(Names::Subroutines);
        self.consume('(');
        if self.curr_token != ')' {
            self.compile_parameter_list();
        }
        self.consume(')');
        self.compile_subroutine_body();
        writeln!(self.writer, "</classVarDec>").expect("failed to finish class var declaration");
    }

    fn compile_parameter_list(&mut self) {
        self.names.get(Names::Vars).clear();
        while self.curr_token != ')' {
            self.check_for_type();
            self.check_for_duplicate(Names::Vars);
            if self.curr_token == ',' {
                self.consume(',');
            }
        }
    }

    fn compile_subroutine_body(&mut self) {
        self.names.get(Names::Vars).clear();
        writeln!(self.writer, "<subroutineBody>").expect("failed to start subroutine body");
        self.consume('{');
        while self.curr_token == Keyword::Var {
            self.compile_var_dec();
        }
        self.compile_statements();
        self.consume('}');
        writeln!(self.writer, "</subroutineBody>").expect("failed to finish subroutine body");
    }

    fn compile_var_dec(&mut self) {
        writeln!(self.writer, "<varDec>").expect("failed to start var declaration");
        self.consume(Var);
        self.consume(TokenType::Type);
        if let Some(Identifier(s)) = self.curr_token.identifier() {
            if self.names.get(Names::Vars).insert(s.clone()) {
                self.consume(TokenType::Name);
            } else {
                self.errors.push(CompilationError::DuplicateIdentifier);
            }
        } else {
            self.errors.push(CompilationError::UnexpectedToken);
        }
        self.consume(';');
        writeln!(self.writer, "</varDec>").expect("failed to finish var declaration");
    }

    fn compile_statements(&mut self) {
        writeln!(self.writer, "<statements>").expect("failed to start statements");
        while self.curr_token == TokenType::Statement {
            match self.curr_token.keyword() {
                Some(Let) => self.compile_let(),
                Some(If) => self.compile_if(),
                Some(While) => self.compile_while(),
                Some(Do) => self.compile_do(),
                Some(Return) => self.compile_return(),
                _ => break,
            }
        }
        writeln!(self.writer, "<statements>").expect("failed to finish statements");
    }

    fn compile_let(&mut self) {
        writeln!(self.writer, "<letStatement>").expect("failed to start let statement");
        self.consume(Let);
        if let Some(Identifier(s)) = self.curr_token.identifier() {
            if self.names.get(Names::Classes).contains(s)
                || self.names.get(Names::Subroutines).contains(s)
            {
                self.errors.push(CompilationError::UnexpectedToken);
            } else if !self.names.contains(s) {
                self.errors.push(CompilationError::UnrecognizedIdentifier);
            } else {
                self.consume(TokenType::Name);
            }
        }
        if self.curr_token == '[' {
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
        if self.curr_token == Else {
            self.consume(Else);
            if self.curr_token == If {
                self.compile_if();
            } else {
                self.consume('{');
                self.compile_statements();
                self.consume('}');
            }
        }
        writeln!(self.writer, "</ifStatement>").expect("failed to finish if statement");
    }

    fn compile_do(&mut self) {
        writeln!(self.writer, "<doStatement>").expect("failed to start do statement");
        self.consume(Do);
        self.compile_subroutine_call();
        self.consume(';');
        writeln!(self.writer, "</doStatement>").expect("failed to finish do statement");
    }

    fn compile_return(&mut self) {
        writeln!(self.writer, "<returnStatement>").expect("failed to start return statement");
        self.consume(Return);
        if self.curr_token != ';' {
            self.compile_expression();
        }
        self.consume(';');
        writeln!(self.writer, "</returnStatement>").expect("failed to finish return statement");
    }

    fn compile_subroutine_call(&mut self) {
        if let Some(Identifier(s)) = self.curr_token.identifier() {
            let (name, subroutine_name) = (
                self.names.contains(s),
                self.names.get(Names::Subroutines).contains(s),
            );
            if name {
                self.consume(TokenType::Name);
                if !subroutine_name {
                    self.consume('.');
                    if let Some(Identifier(s)) = self.curr_token.identifier() {
                        if self.names.get(Names::Subroutines).contains(s) {
                            self.consume(TokenType::Name);
                        } else {
                            self.errors.push(CompilationError::UnexpectedToken);
                        }
                    } else {
                        self.errors.push(CompilationError::UnexpectedToken);
                    }
                    self.consume('(');
                    self.compile_expression_list();
                    self.consume(')');
                } else {
                    self.errors.push(CompilationError::UnrecognizedIdentifier);
                }
            } else {
                self.errors.push(CompilationError::UnexpectedToken);
            }
        }
    }

    fn compile_term(&mut self) {
        writeln!(self.writer, "<term>").expect("failed to start term");
        if self.curr_token == '(' {
            self.consume('(');
            self.compile_expression();
            self.consume(')');
        } else {
            if self.curr_token == TokenType::UnaryOp {
                self.consume(TokenType::UnaryOp);
            }
            if self.curr_token == TokenType::Constant {
                self.consume(Constant);
            } else if let Some(Identifier(s)) = self.curr_token.identifier() {
                if self.names.get(Names::Classes).contains(s)
                    || self.names.get(Names::Subroutines).contains(s)
                {
                    self.compile_subroutine_call();
                } else {
                    self.consume(TokenType::Name);
                    if self.curr_token == '[' {
                        self.consume('[');
                        self.compile_expression();
                        self.consume(']');
                    }
                }
            }
        }
        writeln!(self.writer, "</term>").expect("failed to finish term");
    }

    fn compile_expression(&mut self) {
        writeln!(self.writer, "<expression>").expect("failed to start expression");
        self.compile_term();
        if self.curr_token == TokenType::BinaryOp {
            self.consume(TokenType::BinaryOp);
            self.compile_term();
        }
        writeln!(self.writer, "</expression>").expect("failed to finish expression");
    }

    fn compile_expression_list(&mut self) {
        writeln!(self.writer, "<expressionList>").expect("failed to start expression list");
        while self.curr_token != ')' {
            self.compile_expression();
            if self.curr_token == ',' {
                self.consume(',');
            }
        }
        writeln!(self.writer, "</expressionList>").expect("failed to finish expression list");
    }

    fn check_for_type(&mut self) {
        todo!()
    }
}
