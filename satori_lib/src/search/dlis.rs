use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::literal::Literal;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Implements the  Dynamic Largest Individual Sum (DLIS) decision heuristic.
/// Approach: choose literal that satisfies most unresolved clauses
///  - for each variable x, calculate
///     - C+: number of unresolved clauses with x
///     - C-: number of unresolved clauses with -x
///  - select two variables x and y that maximize these two metrics from all the variables
///  - if C+(x) > C-(y) set x to true, else set y to false
#[derive(Default)]
pub struct DLIS {
    /// for each literal, stores the number of unresolved clauses
    clause_count: HashMap<Literal, u32>,
}

impl DLIS {
    /// Returns the optimal next decision literal according to DLIS or `None` if no variables are unassigned
    pub fn decide(&self) -> Option<Literal> {
        let mut count = self.clause_count.iter().collect::<Vec<_>>();
        count.sort_by(|a, b| b.1.cmp(a.1));

        let x = count.iter().find(|x| x.0.is_positive() && *x.1 > 0);
        let y = count.iter().find(|y| y.0.is_negative() && *y.1 > 0);

        match (x, y) {
            (Some(x), Some(y)) => match x.1.cmp(y.1) {
                Ordering::Greater => Some(*x.0),
                Ordering::Equal => Some(*y.0),
                Ordering::Less => Some(*y.0),
            },
            (Some(x), None) => Some(*x.0),
            (None, Some(y)) => Some(*y.0),
            (None, None) => None,
        }
    }

    pub fn increment(&mut self, literal: Literal) {
        let counter = self.clause_count.entry(literal).or_insert(0);
        *counter += 1;
    }

    pub fn decrement(&mut self, literal: Literal) {
        let counter = self.clause_count.entry(literal).or_insert(0);
        *counter = counter.saturating_sub(1);
    }

    pub fn init(&mut self, binary_clauses: &BinaryClauses, long_clauses: &LongClauses) {
        for clause in binary_clauses
            .clauses()
            .iter()
            .chain(long_clauses.clauses().iter())
        {
            for literal in clause.literals() {
                self.increment(*literal);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_literal_is_positive() {
        let mut dlis = DLIS::default();
        dlis.clause_count.insert(Literal::from_dimacs(1), 2);
        dlis.clause_count.insert(Literal::from_dimacs(-1), 1);

        assert_eq!(dlis.decide().unwrap(), Literal::from_dimacs(1));
    }

    #[test]
    fn test_literal_is_negative() {
        let mut dlis = DLIS::default();
        dlis.clause_count.insert(Literal::from_dimacs(1), 1);
        dlis.clause_count.insert(Literal::from_dimacs(-1), 2);

        assert_eq!(dlis.decide().unwrap(), Literal::from_dimacs(-1));

        dlis.decrement(Literal::from_dimacs(-1));
        assert_eq!(dlis.decide().unwrap(), Literal::from_dimacs(-1));
    }

    #[test]
    fn test_no_literals_left() {
        let mut dlis = DLIS::default();
        dlis.clause_count.insert(Literal::from_dimacs(1), 0);
        dlis.clause_count.insert(Literal::from_dimacs(-1), 0);

        assert!(dlis.decide().is_none());
    }
}
