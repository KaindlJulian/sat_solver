use crate::literal::{Literal, Variable};
use std::collections::HashMap;
use std::ops::Not;

/// Possible assignment values for a variable
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AssignedValue {
    True,
    False,
    Unknown,
}

impl Default for AssignedValue {
    fn default() -> Self {
        AssignedValue::Unknown
    }
}

impl Not for AssignedValue {
    type Output = AssignedValue;

    fn not(self) -> Self::Output {
        match self {
            AssignedValue::True => AssignedValue::False,
            AssignedValue::False => AssignedValue::False,
            AssignedValue::Unknown => AssignedValue::Unknown,
        }
    }
}

/// Holds assignments to variables. Assignments are represented as `Option<bool>` for the variable
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct VariableAssignment {
    partial: HashMap<Variable, AssignedValue>,
}

impl VariableAssignment {
    pub fn from_variables(vars: &Vec<Variable>) -> VariableAssignment {
        VariableAssignment {
            partial: vars
                .iter()
                .map(|v| (v.clone(), AssignedValue::default()))
                .collect(),
        }
    }

    pub fn set_true(&mut self, lit: Literal) {
        self.partial.insert(
            lit.variable(),
            if lit.is_positive() {
                AssignedValue::True
            } else {
                AssignedValue::False
            },
        );
    }

    pub fn set_false(&mut self, lit: Literal) {
        self.partial.insert(
            lit.variable(),
            if lit.is_negative() {
                AssignedValue::True
            } else {
                AssignedValue::False
            },
        );
    }

    pub fn set_unknown(&mut self, var: Variable) {
        self.partial.insert(var, AssignedValue::Unknown);
    }

    pub fn is_assigned(&self, var: Variable) -> bool {
        self.partial
            .get(&var)
            .map(|a| match a {
                AssignedValue::True | AssignedValue::False => true,
                AssignedValue::Unknown => false,
            })
            .unwrap_or(false)
    }

    pub fn get_value(&self, var: Variable) -> &AssignedValue {
        self.partial.get(&var).unwrap_or(&AssignedValue::Unknown)
    }

    pub fn get_literal_value(&self, lit: Literal) -> AssignedValue {
        self.partial
            .get(&lit.variable())
            .map(|value| {
                if lit.is_positive() {
                    *value
                } else {
                    value.not()
                }
            })
            .unwrap_or(AssignedValue::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignments() {
        let mut assignments = VariableAssignment::default();
        let lit_a = Literal::from_dimacs(1);
        assignments.set_true(lit_a); // a = true
        assert_eq!(AssignedValue::True, assignments.get_literal_value(lit_a)); // a should be true
        assert_eq!(AssignedValue::False, assignments.get_literal_value(!lit_a)); // !a should be false
        assignments.set_unknown(lit_a.variable()); // a = ?
        assert_eq!(assignments.is_assigned(lit_a.variable()), false) // a should not be assigned
    }
}
