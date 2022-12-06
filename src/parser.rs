use crate::{
    symbol_table::{Kind, SymbolTable},
    token_type::{TokenType, ValidToken},
    tokenizer::Tokenizer,
    tokens::{
        Keyword::{self, *},
        Token,
    },
    xml_writer::XMLWriter, vm_writer::CodeWriter,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

pub struct Parser {
    writer: XMLWriter,
    tokenizer: Tokenizer,
    curr_token: Option<Token>,
    errors: Vec<(CompilationError, Option<Token>)>,
    symbol_table: SymbolTable,
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
            writer: XMLWriter::default(),
            tokenizer: Tokenizer::default(),
            curr_token: None,
            errors: vec![],
            symbol_table: SymbolTable::default(),
        }
    }

    pub fn throw_error(&mut self, err: CompilationError) {
        let token = self.curr_token.as_ref();
        self.errors.push((err, Option::<&Token>::cloned(token)));
    }

    pub fn curr_token_is<T: ValidToken + PartialEq<Token>>(&self, other: T) -> bool {
        if let Some(t) = self.curr_token.as_ref() {
            other == *t
        } else {
            false
        }
    }

    pub fn compile(&mut self, file: PathBuf) -> Result<(), Vec<(CompilationError, Option<Token>)>> {
        let filename = file.as_path().to_str().expect("could not conver to str");
        let tokenizer = Tokenizer::new(std::fs::read_to_string(&file).expect("failed to read"));
        self.writer = XMLWriter::new(filename);
        self.tokenizer = tokenizer;
        self.curr_token = self.tokenizer.advance();
        self.construct_class();
        if !self.errors.is_empty() {
            Err(self.errors.clone())
        } else {
            Ok(())
        }
    }

    fn consume<T: ValidToken + PartialEq<Token> + Copy>(&mut self, requested: T) -> Token {
        if !self.curr_token_is(requested) {
            self.throw_error(CompilationError::UnexpectedToken);
        } else {
            // leaving this here until we're past xml
            self.writer
                .write(self.curr_token.as_ref().expect("no token"));
        }
        let mut token = self.tokenizer.advance();
        std::mem::swap(&mut self.curr_token, &mut token);
        // return the last token in case it's wanted
        // using it is situational, and if it's not needed this essentially discards it anyway
        token.unwrap_or(Token::Symbol('#'))
    }

    fn construct_class(&mut self) {
        self.writer.start("class");
        self.consume(Class);
        self.consume(TokenType::Name);
        self.consume('{');
        loop {
            if self.curr_token_is(TokenType::ClassVarDec) {
                self.compile_class_var_dec();
            } else {
                break;
            }
        }
        loop {
            if self.curr_token_is(TokenType::SubroutineDec) {
                self.compile_subroutine_dec();
            } else {
                break;
            }
        }
        self.consume('}');
        self.writer.finish("class");
    }

    fn compile_class_var_dec(&mut self) {
        self.writer.start("classVarDec");
        if let (Token::Keyword(k @ (Static | Field)), typ, Token::Identifier(name)) = (
            self.consume(TokenType::ClassVarDec),
            self.consume(TokenType::Type),
            self.consume(TokenType::Name),
        ) {
            let kind = if k == Static {
                Kind::Static
            } else {
                Kind::Field
            };
            let type_of = match typ {
                Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
                Token::Identifier(s) => s,
                _ => String::from("shouldn't be any other var type here"),
            };
            self.symbol_table.define(kind, &type_of, name)
                .map_err(|e| self.throw_error(e))
                .unwrap();
            while self.curr_token_is(',') {
                self.consume(',');
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.symbol_table.define(kind, &type_of, name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                }
            }
            self.consume(';');
            self.writer.finish("classVarDec");
        }
    }

    fn compile_subroutine_dec(&mut self) {
        self.writer.start("subroutineDec");
        self.symbol_table.start_subroutine();
        self.consume(TokenType::SubroutineDec);
        self.consume(TokenType::ReturnType);
        self.consume(TokenType::Name);
        self.consume('(');
        self.compile_parameter_list();
        self.consume(')');
        self.compile_subroutine_body();
        self.writer.finish("subroutineDec")
    }

    fn compile_parameter_list(&mut self) {
        self.writer.start("parameterList");
        while !self.curr_token_is(')') {
            if let (typ, Token::Identifier(name)) = (
                self.consume(TokenType::Type),
                self.consume(TokenType::Name)
            ) {
                let type_of = match typ {
                    Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
                    Token::Identifier(s) => s,
                    _ => String::from("shouldn't be any other var type here"),
                };
                self.symbol_table.define(Kind::Arg, &type_of, name)
                    .map_err(|e| self.throw_error(e))
                    .unwrap();
            }
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
        self.writer.finish("parameterList");
    }

    fn compile_subroutine_body(&mut self) {
        self.writer.start("subroutineBody");
        self.consume('{');
        while self.curr_token_is(Keyword::Var) {
            self.compile_var_dec();
        }
        self.compile_statements();
        self.consume('}');
        self.writer.finish("subroutineBody");
    }

    fn compile_var_dec(&mut self) {
        self.writer.start("varDec");
        if let (Token::Keyword(_k @ Var), typ, Token::Identifier(name)) = (
            self.consume(Var),
            self.consume(TokenType::Type),
            self.consume(TokenType::Name),
        ) {
            let type_of = match typ {
                Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
                Token::Identifier(s) => s,
                _ => String::from("shouldn't be any other var type here"),
            };
            self.symbol_table.define(Kind::Local, &type_of, name)
                .map_err(|e| self.throw_error(e))
                .unwrap();
            while self.curr_token_is(',') {
                self.consume(',');
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.symbol_table.define(Kind::Local, &type_of, name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                }
            }
            self.consume(';');
        }
        self.writer.finish("varDec");
    }

    fn compile_statements(&mut self) {
        self.writer.start("statements");
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
        self.writer.finish("statements");
    }

    fn compile_let(&mut self) {
        self.writer.start("letStatement");
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
        self.writer.finish("letStatement");
    }

    fn compile_while(&mut self) {
        self.writer.start("whileStatement");
        self.consume(While);

        self.consume('(');

        self.compile_expression();

        self.consume(')');
        self.consume('{');
        self.compile_statements();
        self.consume('}');
        self.writer.finish("whileStatement");
    }

    fn compile_if(&mut self) {
        self.writer.start("ifStatement");
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
        self.writer.finish("ifStatement");
    }

    fn compile_do(&mut self) {
        self.writer.start("doStatement");
        self.consume(Do);
        self.compile_subroutine_call();
        self.consume(';');
        self.writer.finish("doStatement");
    }

    fn compile_return(&mut self) {
        self.writer.start("returnStatement");
        self.consume(Return);
        if !self.curr_token_is(';') {
            self.compile_expression();
        }
        self.consume(';');
        self.writer.finish("returnStatement");
    }

    fn compile_subroutine_call(&mut self) {
        self.consume(TokenType::Name);
        if self.curr_token_is('.') {
            self.consume('.');
            self.consume(TokenType::Name);
        }
        self.consume('(');
        self.compile_expression_list();
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
        self.writer.start("term");
        if self.curr_token_is('(') {
            self.consume('(');
            self.compile_expression();
            self.consume(')');
        } else {
            if self.curr_token_is(TokenType::UnaryOp) {
                self.consume(TokenType::UnaryOp);
                self.compile_term();
            }
            if self.curr_token_is(TokenType::Constant) {
                self.consume(Constant);
            } else if self.curr_token_is(TokenType::Name) {
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
        self.writer.finish("term");
    }

    fn compile_expression(&mut self) {
        self.writer.start("expression");
        self.compile_term();
        if self.curr_token_is(TokenType::BinaryOp) {
            self.consume(TokenType::BinaryOp);
            self.compile_term();
        }
        self.writer.finish("expression");
    }

    fn compile_expression_list(&mut self) {
        self.writer.start("expressionList");
        while !self.curr_token_is(')') {
            self.compile_expression();
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
        self.writer.finish("expressionList");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_symbol_table_class_var_dec() {
        let parser = Parser::new();
    }
}
