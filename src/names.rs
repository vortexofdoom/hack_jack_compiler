use std::collections::HashSet;

use crate::{parser::CompilationError, tokens::Token};

//use crate::validation::TokenType;

pub struct NameSet {
    classes: HashSet<String>,
    class_vars: HashSet<String>,
    subroutines: HashSet<String>,
    vars: HashSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Names {
    Classes,
    ClassVars,
    Subroutines,
    Vars,
}

impl NameSet {
    pub fn new() -> Self {
        NameSet {
            classes: HashSet::new(),
            class_vars: HashSet::new(),
            subroutines: HashSet::new(),
            vars: HashSet::new(),
        }
    }

    pub fn get(&self, set: Names) -> &HashSet<String> {
        match set {
            Names::Classes => &self.classes,
            Names::ClassVars => &self.class_vars,
            Names::Subroutines => &self.subroutines,
            Names::Vars => &self.vars,
        }
    }
    
    pub fn get_mut(&mut self, set: Names) -> &mut HashSet<String> {
        match set {
            Names::Classes => &mut self.classes,
            Names::ClassVars => &mut self.class_vars,
            Names::Subroutines => &mut self.subroutines,
            Names::Vars => &mut self.vars,
        }
    }

    pub fn contains(&mut self, token: Option<&Token>) -> bool {
        if let Some(Token::Identifier(name)) = token {
            self.classes.contains(name)
                || self.class_vars.contains(name)
                || self.vars.contains(name)
                || self.subroutines.contains(name)
        } else {
            false
        }
    }

    pub fn is_valid(&mut self, set: Names, token: Option<&Token>) -> bool {
        if let Some(Token::Identifier(s)) = token {
            self.get(set).contains(s)
        } else {
            false
        }
    }

    pub fn check_duplicate(
        &mut self,
        set: Names,
        token: Option<&Token>,
    ) -> Result<(), CompilationError> {
        if let Some(Token::Identifier(name)) = token {
            if !self.get_mut(set).insert(name.to_string()) {
                Ok(())
            } else {
                Err(CompilationError::DuplicateIdentifier)
            }
        } else {
            Err(CompilationError::UnexpectedToken)
        }
    }
}
