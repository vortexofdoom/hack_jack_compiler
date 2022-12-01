use std::collections::HashSet;

pub struct Names {
    classes: HashSet<String>,
    class_vars: HashSet<String>,
    subroutines: HashSet<String>,
    vars: HashSet<String>,
}

impl Names {
    pub fn new() -> Self {
        Names {
            classes: HashSet::new(),
            class_vars: HashSet::new(),
            subroutines: HashSet::new(),
            vars: HashSet::new(),
        }
    }

    pub fn get(&mut self, set: NameSet) -> &mut HashSet<String> {
        match set {
            NameSet::Classes => &mut self.classes,
            NameSet::ClassVars => &mut self.class_vars,
            NameSet::Subroutines => &mut self.subroutines,
            NameSet::Vars => &mut self.vars,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NameSet {
    Classes,
    ClassVars,
    Subroutines,
    Vars,
}
