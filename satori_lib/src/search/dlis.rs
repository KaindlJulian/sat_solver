use crate::assignment::VariableAssignment;
use crate::literal::Literal;

use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::resize::Resize;
use std::cmp::Ordering;

/// Implements Dynamic Largest Individual Sum (DLIS).
///
/// Approach: choose literal that satisfies most unresolved clauses
///  - for each variable x, calculate
///     - C(x): number of unresolved clauses with x
///     - C(-x): number of unresolved clauses with -x
///  - select two variables x and y that maximize these two metrics from all the variables
///  - if C(x) > C(-y) set x to true, else set y to false
#[derive(Default, Debug)]
pub struct Dlis {
    // maps a literal (indexed by the literal code) to its number of unresolved clauses
    entries: Vec<usize>,
}

impl Resize for Dlis {
    fn resize(&mut self, variable_count: usize) {
        self.entries.resize(variable_count * 2, 0);
    }
}

impl Dlis {
    fn clear(&mut self) {
        self.entries.resize(self.entries.len(), 0);
    }

    fn decide(&self) -> Option<Literal> {
        let x = &self
            .entries
            .iter()
            .enumerate()
            .filter(|(code, _)| Literal::from_code(*code).is_positive())
            .filter(|(_, count)| **count > 0)
            .max_by_key(|(_, count)| *count)
            .map(|(code, _)| Literal::from_code(code));

        let y = &self
            .entries
            .iter()
            .enumerate()
            .filter(|(code, _)| Literal::from_code(*code).is_negative())
            .filter(|(_, count)| **count > 0)
            .max_by_key(|(_, count)| *count)
            .map(|(code, _)| Literal::from_code(code));

        match (x, y) {
            (Some(x), Some(y)) => match self.entries[x.as_index()].cmp(&self.entries[y.as_index()])
            {
                Ordering::Greater => Some(*x),
                Ordering::Equal => Some(*y),
                Ordering::Less => Some(*y),
            },
            (Some(x), None) => Some(*x),
            (None, Some(y)) => Some(*y),
            (None, None) => None,
        }
    }
}

/// Returns the next decision literal according to DLIS or `None` if no variables are unassigned.
pub fn dlis(
    dlis: &mut Dlis,
    assignment: &VariableAssignment,
    long: &LongClauses,
    binary: &BinaryClauses,
) -> Option<Literal> {
    let unassigned_variables = assignment.unassigned();
    if unassigned_variables.is_empty() {
        return None;
    }

    //return unassigned_variables.first().map(|v| Literal::from_variable(v, true));

    dlis.clear();

    for v in unassigned_variables {
        for sign in [true, false] {
            let lit = Literal::from_variable(&v, sign);
            let unresolved_clauses_count = long.unresolved(lit) + binary.unresolved(lit);
            dlis.entries[lit.as_index()] = unresolved_clauses_count;
        }
    }

    dlis.decide()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_is_positive() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        dlis.entries[Literal::from_dimacs(1).as_index()] = 2;
        dlis.entries[Literal::from_dimacs(-1).as_index()] = 1;

        let decision = dlis.decide().unwrap();
        assert_eq!(decision, Literal::from_dimacs(1));
    }

    #[test]
    fn test_literal_is_negative() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        dlis.entries[Literal::from_dimacs(1).as_index()] = 1;
        dlis.entries[Literal::from_dimacs(-1).as_index()] = 2;

        let decision = dlis.decide().unwrap();
        assert_eq!(decision, Literal::from_dimacs(-1));
    }

    #[test]
    fn test_no_literals_left() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        dlis.entries[Literal::from_dimacs(1).as_index()] = 0;
        dlis.entries[Literal::from_dimacs(-1).as_index()] = 0;

        let decision = dlis.decide();
        dbg!(decision);
        assert!(decision.is_none());
    }
}
