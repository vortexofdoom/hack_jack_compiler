use std::collections::HashSet;

//use crate::validation::TokenType;

pub struct NameSet {
    classes: HashSet<String>,
    class_vars: HashSet<String>,
    subroutines: HashSet<String>,
    vars: HashSet<String>,
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

    pub fn get(&mut self, set: Names) -> &mut HashSet<String> {
        match set {
            Names::Classes => &mut self.classes,
            Names::ClassVars => &mut self.class_vars,
            Names::Subroutines => &mut self.subroutines,
            Names::Vars => &mut self.vars,
        }
    }

    pub fn contains(&mut self, name: &str) -> bool {
        self.classes.contains(name)
            || self.class_vars.contains(name)
            || self.vars.contains(name)
            || self.subroutines.contains(name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Names {
    Classes,
    ClassVars,
    Subroutines,
    Vars,
}

// impl Names {
//     pub fn is_valid(&self, name_set: &NameSet, name: &str) -> bool {
//         name_set.get(*self).contains(name)
//     }
// }
