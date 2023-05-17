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

impl Not for AssignedValue {
    type Output = AssignedValue;

    fn not(self) -> Self::Output {
        match self {
            AssignedValue::True => AssignedValue::False,
            AssignedValue::False => AssignedValue::True,
            AssignedValue::Unknown => AssignedValue::Unknown,
        }
    }
}

/// Holds assignments to variables
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct VariableAssignment {
    partial: HashMap<Variable, AssignedValue>,
}

impl VariableAssignment {
    pub fn assign_true(&mut self, lit: Literal) {
        self.partial.insert(
            lit.variable(),
            if lit.is_positive() {
                AssignedValue::True
            } else {
                AssignedValue::False
            },
        );
    }

    pub fn assign_false(&mut self, lit: Literal) {
        self.partial.insert(
            lit.variable(),
            if lit.is_negative() {
                AssignedValue::True
            } else {
                AssignedValue::False
            },
        );
    }

    pub fn assign_unknown(&mut self, var: Variable) {
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

    pub fn value(&self, var: Variable) -> AssignedValue {
        self.partial
            .get(&var)
            .map(|v| *v)
            .unwrap_or(AssignedValue::Unknown)
    }

    pub fn literal_value(&self, lit: Literal) -> AssignedValue {
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

    pub fn partial(&self) -> Vec<Literal> {
        self.partial
            .iter()
            .filter(|(k, v)| **v != AssignedValue::Unknown)
            .map(|(k, v)| (k, *v == AssignedValue::False))
            .map(|(k, v)| Literal::from_index(k.index, v))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_for_variable() {
        let mut assignments = VariableAssignment::default();
        let lit = Literal::from_dimacs(1);
        let var = lit.variable();

        assert_eq!(AssignedValue::Unknown, assignments.value(var));
        assignments.assign_true(lit);
        assert_eq!(AssignedValue::True, assignments.value(var));
        assignments.assign_true(!lit);
        assert_eq!(AssignedValue::False, assignments.value(var));
        assignments.assign_unknown(var);
        assert_eq!(AssignedValue::Unknown, assignments.value(var));
    }

    #[test]
    fn test_get_value_for_literal() {
        let mut assignments = VariableAssignment::default();
        let a = Literal::from_dimacs(1);

        // a = ?
        assert_eq!(AssignedValue::Unknown, assignments.literal_value(a));
        assert_eq!(AssignedValue::Unknown, assignments.literal_value(!a));
        // a = true
        assignments.assign_true(a);
        assert_eq!(AssignedValue::True, assignments.literal_value(a));
        assert_eq!(AssignedValue::False, assignments.literal_value(!a));
        // !a = true
        assignments.assign_true(!a);
        assert_eq!(AssignedValue::False, assignments.literal_value(a));
        assert_eq!(AssignedValue::True, assignments.literal_value(!a));
        // a = false
        assignments.assign_false(a);
        assert_eq!(AssignedValue::False, assignments.literal_value(a));
        assert_eq!(AssignedValue::True, assignments.literal_value(!a));
        // !a = false
        assignments.assign_false(!a);
        assert_eq!(AssignedValue::True, assignments.literal_value(a));
        assert_eq!(AssignedValue::False, assignments.literal_value(!a));
    }
}
