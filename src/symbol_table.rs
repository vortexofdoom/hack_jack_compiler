use std::{collections::HashMap, fmt::Display};

use crate::{
    parser::CompilationError,
    tokens::{
        Keyword::{self, *},
        Token,
    }, vm_writer::MemSegment,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Local,
}
impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Kind::Static => "static",
            Kind::Field => "this",
            Kind::Arg => "argument",
            Kind::Local => "local",
        };
        write!(f, "{s}")
    }
}

#[derive(Default)]
pub struct SymbolTable {
    static_count: u16,
    field_count: u16,
    arg_count: u16,
    local_count: u16,

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

    pub fn var_count(&self, kind: Kind) -> u16 {
        match kind {
            Kind::Static => self.static_count,
            Kind::Field => self.field_count,
            Kind::Arg => self.arg_count,
            Kind::Local => self.local_count,
        }
    }

    pub fn get(&self, name: &str) -> Option<&SymbolEntry> {
        if let Some(e) = self.class_lvl_table.get(name) {
            Some(e)
        } else if let Some(e) = self.subroutine_lvl_table.get(name) {
            Some(e)
        } else {
            None
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_lvl_table.clear();
        self.arg_count = 0;
        self.local_count = 0;
    }

    pub fn index_of(&self, name: &str) -> Option<(Kind, u16)> {
        if let Some(e) = self.class_lvl_table.get(name) {
            Some(e.kind_id)
        } else {
            None
        }
    }
}

pub struct SymbolEntry {
    var_type: String,
    kind_id: (Kind, u16),
}
