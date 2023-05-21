use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::clause::Clause;
use crate::literal::{Literal, Variable};
use crate::search::heuristic::HeuristicCallbacks;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem::transmute;

#[derive(Default, Debug)]
struct DlisEntry {
    /// the number of unresolved clauses that a literal is in
    clause_count: u32,
    /// `true` if the variable is assigned, else `false`
    is_assigned: bool,
}

/// Implements the  Dynamic Largest Individual Sum (DLIS) decision heuristic.
/// Approach: choose literal that satisfies most unresolved clauses
///  - for each variable x, calculate
///     - C+: number of unresolved clauses with x
///     - C-: number of unresolved clauses with -x
///  - select two variables x and y that maximize these two metrics from all the variables
///  - if C+(x) > C-(y) set x to true, else set y to false
#[derive(Default, Debug)]
pub struct DLIS {
    data: HashMap<Literal, DlisEntry>,
}

impl HeuristicCallbacks for DLIS {
    fn assign(&mut self, variable: Variable) {
        // the map stores both literals individually
        for assignment in [true, false].iter() {
            if let Some(entry) = self
                .data
                .get_mut(&Literal::from_index(variable.index(), *assignment))
            {
                entry.is_assigned = true;
            }
        }
    }

    fn unassign(&mut self, variable: Variable) {
        // the map stores both literals individually
        for assignment in [true, false].iter() {
            if let Some(entry) = self
                .data
                .get_mut(&Literal::from_index(variable.index(), *assignment))
            {
                entry.is_assigned = false;
            }
        }
    }

    fn resolved(&mut self, clause: Clause) {
        clause.literals().iter().for_each(|l| self.decrement(*l));
    }
}

impl DLIS {
    /// Returns the optimal next decision literal according to DLIS or `None` if no variables are unassigned
    pub fn decide(&self) -> Option<Literal> {
        let mut unassigned_vars = self
            .data
            .iter()
            .filter(|(_, v)| !v.is_assigned)
            .collect::<Vec<_>>();

        if unassigned_vars.is_empty() {
            return None;
        }

        unassigned_vars.sort_by(|a, b| b.1.clause_count.cmp(&a.1.clause_count));

        let x = unassigned_vars.iter().find(|x| x.0.is_positive());
        let y = unassigned_vars.iter().find(|y| y.0.is_negative());

        match (x, y) {
            (Some(x), Some(y)) => match x.1.clause_count.cmp(&y.1.clause_count) {
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
        let entry = self.data.entry(literal).or_insert(DlisEntry::default());
        entry.clause_count += 1;
    }

    pub fn decrement(&mut self, literal: Literal) {
        let entry = self.data.entry(literal).or_insert(DlisEntry::default());
        entry.clause_count = entry.clause_count.saturating_sub(1);
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
        dlis.data.insert(
            Literal::from_dimacs(1),
            DlisEntry {
                is_assigned: false,
                clause_count: 2,
            },
        );
        dlis.data.insert(
            Literal::from_dimacs(-1),
            DlisEntry {
                is_assigned: false,
                clause_count: 1,
            },
        );

        assert_eq!(dlis.decide().unwrap(), Literal::from_dimacs(1));
    }

    #[test]
    fn test_literal_is_negative() {
        let mut dlis = DLIS::default();
        dlis.data.insert(
            Literal::from_dimacs(1),
            DlisEntry {
                is_assigned: false,
                clause_count: 1,
            },
        );
        dlis.data.insert(
            Literal::from_dimacs(-1),
            DlisEntry {
                is_assigned: false,
                clause_count: 2,
            },
        );

        assert_eq!(dlis.decide().unwrap(), Literal::from_dimacs(-1));

        dlis.decrement(Literal::from_dimacs(-1));
        assert_eq!(dlis.decide().unwrap(), Literal::from_dimacs(-1));
    }

    #[test]
    fn test_no_literals_left() {
        let mut dlis = DLIS::default();
        dlis.data.insert(
            Literal::from_dimacs(1),
            DlisEntry {
                is_assigned: true,
                clause_count: 0,
            },
        );
        dlis.data.insert(
            Literal::from_dimacs(-1),
            DlisEntry {
                is_assigned: true,
                clause_count: 0,
            },
        );

        assert!(dlis.decide().is_none());
    }
}
