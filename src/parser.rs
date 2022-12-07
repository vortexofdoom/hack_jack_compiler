use crate::{
    symbol_table::{Kind, SymbolTable},
    token_type::{TokenType, ValidToken},
    tokenizer::Tokenizer,
    tokens::{
        Keyword::{self, *},
        Token,
    },
    vm_writer::{CodeWriter, Comparison::*, VmCommand, VmWriter, MemSegment as Mem},
    xml_writer::XMLWriter,
};
use std::path::PathBuf;

pub struct CompilationEngine {
    writer: VmWriter,
    tokenizer: Tokenizer,
    curr_token: Option<Token>,
    errors: Vec<(CompilationError, Option<Token>)>,
    //symbol_table: SymbolTable,
}

#[derive(Debug, Clone)]
pub enum CompilationError {
    DuplicateIdentifier,
    UnexpectedToken,
    InvalidInt,
    UnrecognizedToken,
    UndeclaredIdentifier,
    UnexpectedEndofTokens,
}

use crate::token_type::TokenType::*;
impl CompilationEngine {
    pub fn new() -> Self {
        CompilationEngine {
            writer: VmWriter::default(),
            tokenizer: Tokenizer::default(),
            curr_token: None,
            errors: vec![],
            //symbol_table: SymbolTable::default(),
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

    pub fn parse(&mut self, file: PathBuf) -> Result<(), Vec<(CompilationError, Option<Token>)>> {
        let filename = file.as_path().to_str().expect("could not conver to str");
        let tokenizer = Tokenizer::new(std::fs::read_to_string(&file).expect("failed to read"));
        self.writer = VmWriter::new(filename);
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
        if self.curr_token.is_none() {
            self.throw_error(CompilationError::UnexpectedEndofTokens);
        } else if !self.curr_token_is(requested) {
            self.throw_error(CompilationError::UnexpectedToken);
        }
        // leaving this here until we're past xml
        else {
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
                self.handle_class_var_dec();
            } else {
                break;
            }
        }
        loop {
            if self.curr_token_is(TokenType::SubroutineDec) {
                self.handle_subroutine_dec();
            } else {
                break;
            }
        }
        self.consume('}');
        self.writer.finish("class");
    }

    fn handle_class_var_dec(&mut self) {
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
            self.writer.compile_var(kind, typ, name);
            while self.curr_token_is(',') {
                self.consume(',');
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.writer.compile_var(kind, typ, name);
                }
            }
            self.consume(';');
            self.writer.finish("classVarDec");
        }
    }

    fn handle_subroutine_dec(&mut self) {
        self.writer.start_subroutine();
        if let (
            Token::Keyword(k @ (Constructor | Function | Method)),
            typ,
            Token::Identifier(name),
        ) = (
            self.consume(TokenType::SubroutineDec),
            self.consume(TokenType::ReturnType),
            self.consume(TokenType::Name),
        ) {
            let _type_of = match typ {
                Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
                Token::Identifier(s) => s,
                _ => String::from("shouldn't be any other var type here"),
            };
            self.consume('(');
            self.handle_parameter_list();
            self.consume(')');


            match k {
                Constructor => {
                    self.writer.write(VmCommand::Push(
                        Mem::Constant, 
                        self.symbol_table.var_count(Kind::Arg)
                    ));
                    self.writer.write(VmCommand::Call(
                        "Memory.alloc", 
                        1
                    ));
                }
            }
            let lcl = match k {
                Constructor => self.symbol_table.var_count(Kind::Field),
                Function => self.symbol_table.var_count(Kind::Local),
                Method => todo!(),
                _ => 0,
            };
            self.writer.write(VmCommand::Function(&name, lcl));
            self.handle_subroutine_body();
        }
        //self.writer.finish("subroutineDec")
    }

    fn handle_parameter_list(&mut self) {
        //self.writer.start("parameterList");
        //self.writer.
        while !self.curr_token_is(')') {
            if let (typ, Token::Identifier(name)) =
                (self.consume(TokenType::Type), self.consume(TokenType::Name))
            {
                self.writer.
                let type_of = match typ {
                    Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
                    Token::Identifier(s) => s,
                    _ => String::from("shouldn't be any other var type here"),
                };
                self.symbol_table
                    .define(Kind::Arg, &type_of, name)
                    .map_err(|e| self.throw_error(e))
                    .unwrap();
            }
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
        self.writer.finish("parameterList");
    }

    fn handle_subroutine_body(&mut self) {
        self.writer.start("subroutineBody");
        self.consume('{');
        while self.curr_token_is(Keyword::Var) {
            self.handle_var_dec();
        }
        self.handle_statements();
        self.consume('}');
        self.writer.finish("subroutineBody");
    }

    fn handle_var_dec(&mut self) {
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
            self.symbol_table
                .define(Kind::Local, &type_of, name)
                .map_err(|e| self.throw_error(e))
                .unwrap();
            while self.curr_token_is(',') {
                self.consume(',');
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.symbol_table
                        .define(Kind::Local, &type_of, name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                }
            }
            self.consume(';');
        }
        self.writer.finish("varDec");
    }

    fn handle_statements(&mut self) {
        self.writer.start("statements");
        while self.curr_token_is(TokenType::Statement) {
            match self.curr_token.as_ref() {
                Some(Token::Keyword(Let)) => self.handle_let(),
                Some(Token::Keyword(If)) => self.handle_if(),
                Some(Token::Keyword(While)) => self.handle_while(),
                Some(Token::Keyword(Do)) => self.handle_do(),
                Some(Token::Keyword(Return)) => self.handle_return(),
                _ => break,
            }
        }
        self.writer.finish("statements");
    }

    fn handle_let(&mut self) {
        self.writer.start("letStatement");
        self.consume(Let);
        if let Token::Identifier(name) = self.consume(TokenType::Name) {
            if self.symbol_table.get(&name).is_some() {
                let _kind_id = self.symbol_table.index_of(&name).unwrap();
            } else {
                self.throw_error(CompilationError::UndeclaredIdentifier);
            }
        }
        if self.curr_token_is('[') {
            self.consume('[');
            self.handle_expression();
            self.consume(']');
        }
        self.consume('=');
        self.handle_expression();
        self.consume(';');
        self.writer.finish("letStatement");
    }

    fn handle_while(&mut self) {
        self.writer.start("whileStatement");
        self.consume(While);
        self.consume('(');

        self.handle_expression();

        self.consume(')');
        self.consume('{');
        self.handle_statements();
        self.consume('}');
        self.writer.finish("whileStatement");
    }

    fn handle_if(&mut self) {
        self.writer.start("ifStatement");
        self.consume(If);
        self.consume('(');
        self.handle_expression();
        self.consume(')');
        self.consume('{');
        self.handle_statements();
        self.consume('}');
        if self.curr_token_is(Else) {
            self.consume(Else);
            if self.curr_token_is(If) {
                self.handle_if();
            } else {
                self.consume('{');
                self.handle_statements();
                self.consume('}');
            }
        }
        self.writer.finish("ifStatement");
    }

    fn handle_do(&mut self) {
        self.writer.start("doStatement");
        self.consume(Do);
        if let Token::Identifier(name) = self.consume(TokenType::Name) {
            self.handle_subroutine_call(name);
        }
        self.consume(';');
        self.writer.finish("doStatement");
    }

    fn handle_return(&mut self) {
        self.writer.start("returnStatement");
        self.consume(Return);
        if !self.curr_token_is(';') {
            self.handle_expression();
        }
        self.consume(';');
        self.writer.finish("returnStatement");
    }

    fn handle_subroutine_call(&mut self, name: String) {
        let name = if self.curr_token_is('.') {
            self.consume('.');
            if let Token::Identifier(sr) = self.consume(TokenType::Name) {
                format!("{name}.{sr}")
            } else {
                String::from("error")
            }
        } else {
            name
        };
        self.consume('(');
        self.handle_expression_list();
        self.consume(')');
        let args = self.symbol_table.var_count(Kind::Arg);
        self.writer.write(VmCommand::Call(&name, args));
    }

    fn handle_term(&mut self) {
        self.writer.start("term");
        let op = if self.curr_token_is(TokenType::UnaryOp) {
            match self.consume(TokenType::UnaryOp) {
                Token::Symbol('-') => Some(VmCommand::Neg),
                Token::Symbol('~') => Some(VmCommand::Not),
                _ => None,
            }
        } else {
            None
        };
        if self.curr_token_is('(') {
            self.consume('(');
            self.handle_expression();
            self.consume(')');
        } else if self.curr_token_is(TokenType::Constant) {
            match self.consume(Constant) {
                Token::Keyword(True) => {}
                Token::Keyword(False) => {}
                Token::Keyword(This) => {}
                Token::Keyword(Null) => {}
                Token::IntConstant(i) => {}
                Token::StringConstant(s) => {}
                _ => {
                    "this is not a constant";
                }
            }
        } else if let Token::Identifier(name) = self.consume(TokenType::Name) {
            if self.curr_token_is('(') | self.curr_token_is('.') {
                self.handle_subroutine_call(name);
            } else if self.curr_token_is('[') {
                if let Some(e) = self.symbol_table.get(&name) {}
                self.consume('[');
                self.handle_expression();
                self.consume(']');
            }
        }
        if op.is_some() {
            self.writer.write(op.unwrap());
        }
        self.writer.finish("term");
    }

    // TODO: maybe add a label for operator priority to get a feel for it
    // could return a tuple (Option<Term>, Option<Term>, Option<Term>)
    // with a vector of said tuples that gets appended recursively
    // until the top-level expression is complete
    // first things first though
    fn handle_expression(&mut self) {
        self.writer.start("expression");
        self.handle_term();
        if self.curr_token_is(TokenType::BinaryOp) {
            let op = self.consume(TokenType::BinaryOp);
            self.handle_term();
            let op_cmd = match op {
                Token::Symbol('+') => VmCommand::Add,
                Token::Symbol('-') => VmCommand::Sub,
                Token::Symbol('&') => VmCommand::And,
                Token::Symbol('|') => VmCommand::Or,
                Token::Symbol('=') => VmCommand::Compare(Eq),
                Token::Symbol('>') => VmCommand::Compare(GT),
                Token::Symbol('<') => VmCommand::Compare(LT),
                Token::Symbol('*') => VmCommand::Call("Math.multiply", 2),
                Token::Symbol('/') => VmCommand::Call("Math.divide", 2),
                _ => VmCommand::Label("not a binary op"),
            };
            self.writer.write(op_cmd);
        }
        self.writer.finish("expression");
    }

    fn handle_expression_list(&mut self) {
        self.writer.start("expressionList");
        while !self.curr_token_is(')') {
            self.handle_expression();
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
        let _st = SymbolTable::default();
        //st.define(Kind::Static, "int", )
    }
}
