use crate::assignment::VariableAssignment;
use crate::literal::Literal;

use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::resize::Resize;
use std::cmp::Ordering;

/// Implements the Dynamic Largest Individual Sum (DLIS) decision heuristic.
///
/// Approach: choose literal that satisfies most unresolved clauses
///  - for each variable x, calculate
///     - C(x): number of unresolved clauses with x
///     - C(-x): number of unresolved clauses with -x
///  - select two variables x and y that maximize these two metrics from all the variables
///  - if C(x) > C(-y) set x to true, else set y to false
#[derive(Default, Debug)]
pub struct Dlis {
    // maps a literal to its number of clauses
    clauses_by_lit: Vec<usize>,
}

impl Resize for Dlis {
    fn resize(&mut self, variable_count: usize) {
        self.clauses_by_lit.resize(variable_count * 2, 0);
    }
}

impl Dlis {
    pub fn build_dlis_entries(
        &mut self,
        binary_clauses: &BinaryClauses,
        long_clauses: &LongClauses,
    ) {
        for (code, count) in self.clauses_by_lit.iter_mut().enumerate() {
            *count += binary_clauses.clauses_count(Literal::from_code(code));
        }
        for clause in long_clauses.clauses().iter() {
            for literal in clause.literals() {
                self.clauses_by_lit[literal.as_index()] += 1;
            }
        }
    }

    fn decide(&self, unassigned: Vec<Literal>) -> Option<Literal> {
        let mut x_max = 0;
        let mut y_max = 0;
        let mut x = None;
        let mut y = None;

        for code in 0..self.clauses_by_lit.len() {
            let literal = Literal::from_code(code);

            if !unassigned.contains(&literal) {
                continue;
            }

            let count = self.clauses_by_lit[code];

            if literal.is_positive() && count > x_max {
                x_max = count;
                x = Some(literal);
            }

            if literal.is_negative() && count > y_max {
                y_max = count;
                y = Some(literal);
            }
        }

        match (x, y) {
            (Some(x), Some(y)) => {
                match self.clauses_by_lit[x.as_index()].cmp(&self.clauses_by_lit[y.as_index()]) {
                    Ordering::Greater => Some(x),
                    Ordering::Equal => Some(y),
                    Ordering::Less => Some(y),
                }
            }
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}

/// Returns the next decision literal according to DLIS or `None` if no variables are unassigned.
pub fn dlis(dlis: &mut Dlis, assignment: &VariableAssignment) -> Option<Literal> {
    let unassigned_literals = assignment
        .unassigned()
        .iter()
        .flat_map(|v| [true, false].iter().map(|s| Literal::from_variable(v, *s)))
        .collect::<Vec<_>>();

    if unassigned_literals.is_empty() {
        return None;
    }

    dlis.decide(unassigned_literals)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_literal_is_positive() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        dlis.clauses_by_lit[Literal::from_dimacs(1).as_index()] = 2;
        dlis.clauses_by_lit[Literal::from_dimacs(-1).as_index()] = 1;

        let decision = dlis
            .decide(vec![Literal::from_dimacs(1), Literal::from_dimacs(-1)])
            .unwrap();
        assert_eq!(decision, Literal::from_dimacs(1));
    }

    #[test]
    fn test_literal_is_negative() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        dlis.clauses_by_lit[Literal::from_dimacs(1).as_index()] = 1;
        dlis.clauses_by_lit[Literal::from_dimacs(-1).as_index()] = 2;

        let decision = dlis
            .decide(vec![Literal::from_dimacs(1), Literal::from_dimacs(-1)])
            .unwrap();
        assert_eq!(decision, Literal::from_dimacs(-1));
    }

    #[test]
    fn test_no_literals_left() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        dlis.clauses_by_lit[Literal::from_dimacs(1).as_index()] = 1;
        dlis.clauses_by_lit[Literal::from_dimacs(-1).as_index()] = 2;

        let decision = dlis.decide(vec![]);
        dbg!(decision);
        assert!(decision.is_none());
    }
}
