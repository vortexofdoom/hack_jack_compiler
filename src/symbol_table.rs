use std::collections::HashMap;

use crate::{
    token_type::{TokenType, ValidToken},
    tokens::{
        Keyword::{self, *},
        Token,
    }, parser::CompilationError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Local,
}

pub enum IdentifierError {
    DuplicateIdentifier,
    InvalidIdentifier,
    UnknownIdentifier,
}

#[derive(Default)]
pub struct SymbolTable {
    static_count: i16,
    field_count: i16,
    arg_count: i16,
    local_count: i16,

    class_lvl_table: HashMap<String, SymbolEntry>,
    subroutine_lvl_table: HashMap<String, SymbolEntry>,
}

impl SymbolTable {
    pub fn define(
        &mut self,
        kind: Kind,
        type_of: &str,
        name: String,
    ) -> Result<(), CompilationError> {
        let (table, counter) = match kind {
            Kind::Static => (&mut self.class_lvl_table, &mut self.static_count),
            Kind::Field => (&mut self.class_lvl_table, &mut self.field_count),
            Kind::Arg => (&mut self.subroutine_lvl_table, &mut self.arg_count),
            Kind::Local => (&mut self.subroutine_lvl_table, &mut self.local_count),
        };
        if !table.contains_key(&name) {
            table.insert(
                name,
                SymbolEntry {
                    var_type: type_of.to_string(),
                    kind_id: (kind, *counter),
                },
            );
            *counter += 1;
            Ok(())
        } else {
            Err(CompilationError::DuplicateIdentifier)
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_lvl_table.clear()
    }
}

pub struct SymbolEntry {
    var_type: String,
    kind_id: (Kind, i16),
}
