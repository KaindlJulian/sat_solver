use crate::literal::{Literal, Variable};
use std::collections::HashMap;

/// Holds assignments to variables. Assignments are represented as `Option<bool>` for the variable
/// values:
/// ```
/// Some(true); // true
/// Some(false); // false
/// None; // unknown
/// ```
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct VariableAssignment {
    partial: HashMap<Variable, Option<bool>>,
}

impl VariableAssignment {
    pub fn from_variables(vars: &Vec<Variable>) -> VariableAssignment {
        VariableAssignment {
            partial: vars.iter().map(|v| (v.clone(), None)).collect(),
        }
    }

    pub fn set_true(&mut self, lit: Literal) {
        self.partial.insert(lit.variable(), Some(lit.is_positive()));
    }

    pub fn set_false(&mut self, lit: Literal) {
        self.set_true(!lit);
    }

    pub fn set_unknown(&mut self, var: Variable) {
        self.partial.insert(var, None);
    }

    pub fn is_assigned(&self, var: Variable) -> bool {
        self.partial.get(&var).unwrap_or(&None).is_some()
    }

    pub fn is_true(&self, lit: Literal) -> bool {
        lit.is_positive()
            && self
                .partial
                .get(&lit.variable())
                .unwrap_or(&Some(false))
                .unwrap_or(false)
    }

    pub fn is_false(&self, lit: Literal) -> bool {
        lit.is_negative()
            && self
                .partial
                .get(&lit.variable())
                .unwrap_or(&Some(false))
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignments() {
        let mut assignments = VariableAssignment::default();
        let a = Variable::from_index(0);
        assignments.set_true(Literal::from_variable(a, false)); // a = true
        assert_eq!(true, assignments.is_true(Literal::from_variable(a, false))); // a should be true
        assert_eq!(false, assignments.is_true(Literal::from_variable(a, true))); // !a should be false
        assignments.set_unknown(a); // a = ?
        assert_eq!(false, assignments.is_assigned(a))
    }
}
