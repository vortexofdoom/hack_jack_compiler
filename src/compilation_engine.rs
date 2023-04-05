use crate::{
    symbol_table::*,
    token_type::{TokenType, ValidToken},
    tokenizer::Tokenizer,
    tokens::{
        Keyword::{self, *},
        Token,
    },
    vm_writer::{CodeWriter, Comparison::*, MemSegment as Mem, VmCommand, VmWriter},
    //xml_writer::XMLWriter,
};
use std::path::PathBuf;

pub struct CompilationEngine {
    writer: VmWriter,
    tokenizer: Tokenizer,
    class_name: String,
    curr_token: Option<Token>,
    symbol_table: SymbolTable,
    errors: Vec<(CompilationError, Option<Token>)>,
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
            class_name: String::new(),
            symbol_table: SymbolTable::default(),
            curr_token: None,
            errors: vec![],
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

    pub fn compile(&mut self, file: PathBuf) -> Result<(), &[(CompilationError, Option<Token>)]> {
        let filename = file.as_path().to_str().expect("could not convert to str");
        let tokenizer = Tokenizer::new(std::fs::read_to_string(&file).expect("failed to read"));

        self.writer = VmWriter::new(filename);
        self.tokenizer = tokenizer;
        self.curr_token = self.tokenizer.advance();
        self.symbol_table = SymbolTable::default();

        self.construct_class();
        self.writer.flush();

        let errors = &self.errors;
        if !errors.is_empty() {
            Err(errors)
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
        let mut token = self.tokenizer.advance();
        std::mem::swap(&mut self.curr_token, &mut token);
        // return the last token in case it's wanted
        // using it is situational, and if it's not needed essentially discards it anyway
        token.unwrap_or(Token::Symbol('?'))
    }

    fn construct_class(&mut self) {
        self.consume(Class);
        if let Token::Identifier(name) = self.consume(TokenType::Name) {
            self.class_name = name;
        }
        self.consume('{');
        while self.curr_token_is(TokenType::ClassVarDec) {
            self.handle_class_var_dec();
        }
        while self.curr_token_is(TokenType::SubroutineDec) {
            self.handle_subroutine_dec();
        }
        self.consume('}');
    }

    fn handle_class_var_dec(&mut self) {
        // validate syntax and bind relevant elements to variables
        if let (Token::Keyword(k @ (Static | Field)), type_of, Token::Identifier(name)) = (
            self.consume(TokenType::ClassVarDec),
            self.consume(TokenType::Type),
            self.consume(TokenType::Name),
        ) {
            let kind = if k == Static {
                Kind::Static
            } else {
                Kind::Field
            };
            let type_str = type_of.as_type();
            // Add the newly declared variable to the symbol table
            self.symbol_table
                .define(kind, &type_str, name)
                .map_err(|e| self.throw_error(e))
                .unwrap();

            // Support multiple declarations of the same type before a semicolon
            while self.curr_token_is(',') {
                self.consume(',');
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.symbol_table
                        .define(kind, &type_str, name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                }
            }
            self.consume(';');
        }
    }

    fn handle_subroutine_dec(&mut self) {
        // Clear the subroutine symbol table and reset the arg/var counts
        self.symbol_table.start_subroutine();

        // Validate syntax and bind relevant elements to variables
        if let (
            Token::Keyword(func_type @ (Constructor | Function | Method)),
            _return_type,
            Token::Identifier(name),
        ) = (
            self.consume(TokenType::SubroutineDec),
            self.consume(TokenType::ReturnType),
            self.consume(TokenType::Name),
        ) {
            // Jack methods include "this" as their first unspoken argument
            if func_type == Method {
                self.symbol_table
                    .define(Kind::Arg, &self.class_name, String::from("this"))
                    .map_err(|e| self.throw_error(e))
                    .unwrap();
            }
            self.consume('(');
            // Add 0 or more arguments to the symbol table
            self.handle_parameter_list();
            self.consume(')');
            self.handle_subroutine_body(func_type, name);
        }
    }

    fn handle_parameter_list(&mut self) {
        while !self.curr_token_is(')') {
            if let (type_of, Token::Identifier(name)) =
                (self.consume(TokenType::Type), self.consume(TokenType::Name))
            {
                self.symbol_table
                    .define(Kind::Arg, &type_of.as_type(), name)
                    .map_err(|e| self.throw_error(e))
                    .unwrap();
            }
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
    }

    fn handle_subroutine_body(&mut self, func_type: Keyword, name: String) {
        self.consume('{');

        // Add 0 or more local variables to the symbol table
        while self.curr_token_is(Keyword::Var) {
            self.handle_var_dec();
        }

        // Declare function now that the symbol table is complete
        self.writer.write(VmCommand::Function(
            &format!("{}.{}", self.class_name, name),
            self.symbol_table.var_count(Kind::Var),
        ));

        if func_type == Constructor {
            // Constructors require allocating enough memory for all fields
            self.writer.write(VmCommand::Push(
                Mem::Constant,
                self.symbol_table.var_count(Kind::Field),
            ));
            self.writer.write(VmCommand::Call("Memory.alloc", 1));
            self.writer.write(VmCommand::Pop(Mem::Pointer, 0));
        } else if func_type == Method {
            // Methods require a pointer to the current object
            self.writer.write(VmCommand::Push(Mem::Argument, 0));
            self.writer.write(VmCommand::Pop(Mem::Pointer, 0));
        }
        self.handle_statements();
        self.consume('}');
    }

    fn handle_var_dec(&mut self) {
        if let (Token::Keyword(_k @ Var), type_of, Token::Identifier(name)) = (
            self.consume(Var),
            self.consume(TokenType::Type),
            self.consume(TokenType::Name),
        ) {
            self.symbol_table
                .define(Kind::Var, &type_of.as_type(), name)
                .map_err(|e| self.throw_error(e))
                .unwrap();
            while self.curr_token_is(',') {
                self.consume(',');
                if let Token::Identifier(name) = self.consume(TokenType::Name) {
                    self.symbol_table
                        .define(Kind::Var, &type_of.as_type(), name)
                        .map_err(|e| self.throw_error(e))
                        .unwrap();
                }
            }
            self.consume(';');
        }
    }

    fn handle_statements(&mut self) {
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
    }

    fn handle_let(&mut self) {
        self.consume(Let);
        if let Token::Identifier(name) = self.consume(TokenType::Name) {
            let (mut seg, mut id) = if let Some(entry) = self.symbol_table.get(&name) {
                (
                    match entry.get_kind() {
                        Kind::Static => Mem::Static,
                        Kind::Field => Mem::This,
                        Kind::Arg => Mem::Argument,
                        Kind::Var => Mem::Local,
                    },
                    entry.get_id(),
                )
            } else {
                self.throw_error(CompilationError::UndeclaredIdentifier);
                (Mem::Constant, 0)
            };
            let arr = if self.curr_token_is('[') {
                self.consume('[');
                self.handle_expression();
                self.consume(']');
                self.writer.write(VmCommand::Push(seg, id));
                self.writer.write(VmCommand::Add);
                (seg, id) = (Mem::That, 0);
                true
            } else {
                false
            };
            self.consume('=');
            self.handle_expression();
            if arr {
                self.writer.write(VmCommand::Pop(Mem::Temp, 0));
                self.writer.write(VmCommand::Pop(Mem::Pointer, 1));
                self.writer.write(VmCommand::Push(Mem::Temp, 0));
            }
            self.writer.write(VmCommand::Pop(seg, id));
            self.consume(';');
        }
    }

    fn handle_while(&mut self) {
        self.consume(While);
        self.consume('(');

        let start_label = self.writer.generate_label("while");
        let end_label = self.writer.generate_label("while");

        // Place the starting label just prior to evaluating the condition
        self.writer.write(VmCommand::Label(&start_label));
        self.handle_expression();

        // Bypass loop if negated condition is true
        self.writer.write(VmCommand::Not);
        self.writer.write(VmCommand::IfGoto(&end_label));
        self.consume(')');
        self.consume('{');

        // Inside loop and jump to start
        self.handle_statements();
        self.writer.write(VmCommand::Goto(&start_label));
        self.consume('}');

        // Label at the end of loop
        self.writer.write(VmCommand::Label(&end_label));
    }

    fn handle_if(&mut self) {
        self.consume(If);

        self.consume('(');
        self.handle_expression();
        self.consume(')');

        // Negate for simpler if-goto
        self.writer.write(VmCommand::Not);

        let label1 = self.writer.generate_label("if");
        let label2 = self.writer.generate_label("if");

        self.writer.write(VmCommand::IfGoto(&label1));

        self.consume('{');
        self.handle_statements();
        self.consume('}');

        self.writer.write(VmCommand::Goto(&label2));
        self.writer.write(VmCommand::Label(&label1));

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

        self.writer.write(VmCommand::Label(&label2));
    }

    fn handle_do(&mut self) {
        self.consume(Do);
        if let Token::Identifier(name) = self.consume(TokenType::Name) {
            if let Some(Token::Symbol(c @ ('.' | '('))) = self.curr_token {
                self.handle_subroutine_call(name, c);
            }
        }
        self.consume(';');

        // All "do" statements in Jack are "void" function calls
        // which require discarding the return value that the VM implementation requires
        self.writer.write(VmCommand::Pop(Mem::Temp, 0));
    }

    fn handle_return(&mut self) {
        self.consume(Return);
        if !self.curr_token_is(';') {
            self.handle_expression();
        } else {
            // The VM requires that a value is returned even if the type is void
            self.writer.write(VmCommand::Push(Mem::Constant, 0));
        }
        self.writer.write(VmCommand::Return);
        self.consume(';');
    }

    fn handle_subroutine_call(&mut self, name: String, next: char) {
        // Easy way to add an extra argument if we determine the subroutine is a method and requires 'this'
        let mut method = false;
        let func_label: String;

        self.consume(next);
        if next == '.' {
            let token = self.consume(Name);
            self.consume('(');

            // If the name is in the table we get its class for the label and push it so the method can be called
            // Otherwise, it's simply a class function on its own
            match (self.symbol_table.get(&name), token) {
                (Some(entry), Token::Identifier(f)) => {
                    self.writer.write(VmCommand::Push(
                        entry.get_kind().to_mem_seg(),
                        entry.get_id(),
                    ));
                    func_label = format!("{}.{}", entry.get_type(), f);
                    method = true;
                }
                (None, Token::Identifier(f)) => func_label = format!("{}.{}", name, f),
                _ => func_label = String::from("error"),
            }
        } else {
            // Any calls without a '.' will be called from within this class
            // so we can simply use the class name
            self.writer.write(VmCommand::Push(Mem::Pointer, 0));
            method = true;
            func_label = format!("{}.{}", self.class_name, name);
        }

        let args = self.handle_expression_list();
        self.consume(')');

        self.writer
            .write(VmCommand::Call(&func_label, args + method as i16));
    }

    fn handle_term(&mut self) {
        // Check for unary operators
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
            let token = self.consume(Constant);
            self.writer.write_constant(token);
        } else if let Token::Identifier(name) = self.consume(TokenType::Name) {
            // Check whether we are evaluating as a subroutine call or as a value
            match (self.symbol_table.get(&name), &self.curr_token) {
                // Subroutine
                (_, Some(Token::Symbol(c @ ('.' | '(')))) => self.handle_subroutine_call(name, *c),
                // Value
                (Some(entry), _) => {
                    let (kind, id) = (entry.get_kind().to_mem_seg(), entry.get_id());
                    if self.curr_token_is('[') {
                        self.consume('[');
                        self.handle_expression();
                        self.consume(']');
                        self.writer.write(VmCommand::Push(kind, id));
                        self.writer.write(VmCommand::Add);
                        self.writer.write(VmCommand::Pop(Mem::Pointer, 1));
                        self.writer.write(VmCommand::Push(Mem::That, 0));
                    } else {
                        self.writer.write(VmCommand::Push(kind, id));
                    }
                }
                (None, _) => self.throw_error(CompilationError::UndeclaredIdentifier),
            }
        }

        // Use the unary operator if it exists
        if let Some(o) = op {
            self.writer.write(o);
        }
    }

    // TODO: maybe add a label for operator priority to get a feel for it
    // could return a tuple (Option<Term>, Option<Term>, Option<Term>)
    // with a vector of said tuples that gets appended recursively
    // until the top-level expression is complete
    // first things first though
    fn handle_expression(&mut self) {
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
                Token::Symbol('%') => VmCommand::Call("Math.modulo", 2),
                _ => VmCommand::Label("not a binary op"),
            };
            self.writer.write(op_cmd);
        }
    }

    // Evaluates the expressions and returns the total number of arguments for the function caller
    fn handle_expression_list(&mut self) -> i16 {
        let mut count: i16 = 0;
        while !self.curr_token_is(')') {
            self.handle_expression();
            count += 1;
            if self.curr_token_is(',') {
                self.consume(',');
            }
        }
        count
    }
}
