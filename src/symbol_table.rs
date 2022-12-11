use std::{collections::HashMap, fmt::Display};

use crate::{compilation_engine::CompilationError, vm_writer::MemSegment};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Var,
}
impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Kind::Static => "static",
            Kind::Field => "this",
            Kind::Arg => "argument",
            Kind::Var => "local",
        };
        write!(f, "{s}")
    }
}

impl Kind {
    pub fn to_mem_seg(self) -> MemSegment {
        match self {
            Kind::Static => MemSegment::Static,
            Kind::Field => MemSegment::This,
            Kind::Arg => MemSegment::Argument,
            Kind::Var => MemSegment::Local,
        }
    }
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
            Kind::Var => (&mut self.subroutine_lvl_table, &mut self.local_count),
        };
        if table.get(&name).is_none() {
            table.insert(
                name,
                SymbolEntry {
                    var_type: String::from(type_of),
                    kind,
                    id: *counter,
                },
            );
            *counter += 1;
            Ok(())
        } else {
            Err(CompilationError::DuplicateIdentifier)
        }
    }

    pub fn var_count(&self, kind: Kind) -> i16 {
        match kind {
            Kind::Static => self.static_count,
            Kind::Field => self.field_count,
            Kind::Arg => self.arg_count,
            Kind::Var => self.local_count,
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
}

pub struct SymbolEntry {
    var_type: String,
    kind: Kind,
    id: i16,
}

impl SymbolEntry {
    pub fn get_type(&self) -> &str {
        &self.var_type
    }
    pub fn get_kind(&self) -> Kind {
        self.kind
    }
    pub fn get_id(&self) -> i16 {
        self.id
    }
}
