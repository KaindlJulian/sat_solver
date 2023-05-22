use crate::assignment::VariableAssignment;
use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::clause::Clause;
use crate::literal::Literal;
use crate::search::heuristic::HeuristicCallbacks;
use std::cmp::Ordering;

/// Implements the  Dynamic Largest Individual Sum (DLIS) decision heuristic.
///
/// Approach: choose literal that satisfies most unresolved clauses
///  - for each variable x, calculate
///     - C(x): number of unresolved clauses with x
///     - C(-x): number of unresolved clauses with -x
///  - select two variables x and y that maximize these two metrics from all the variables
///  - if C(x) > C(-y) set x to true, else set y to false
#[derive(Default, Debug)]
pub struct Dlis {
    // maps a literal (indexed by the literal code) to its number of clauses
    entries: Vec<u32>,
}

impl HeuristicCallbacks for Dlis {
    fn resolved(&mut self, clause: Clause) {
        clause.literals().iter().for_each(|l| self.decrement(*l));
    }
}

impl Dlis {
    pub fn increment(&mut self, literal: Literal) {
        self.entries[literal.as_index()] += 1;
    }

    pub fn decrement(&mut self, literal: Literal) {
        self.entries[literal.as_index()] -= 1;
    }

    pub fn resize(&mut self, var_count: usize) {
        self.entries.resize(2 * var_count, 0);
    }

    pub fn build_dlis_entries(
        &mut self,
        binary_clauses: &BinaryClauses,
        long_clauses: &LongClauses,
    ) {
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

/// Returns the optimal next decision literal according to DLIS or `None` if no variables are unassigned
pub fn decide(dlis: &Dlis, assignment: &VariableAssignment) -> Option<Literal> {
    let unassigned_literals = dlis
        .entries
        .iter()
        .enumerate()
        .map(|(i, _)| Literal::from_code(i as u32))
        .filter(|l| !assignment.is_assigned(l.variable()))
        .collect::<Vec<_>>();

    if unassigned_literals.is_empty() {
        return None;
    }

    let x = unassigned_literals
        .iter()
        .filter(|x| x.is_positive())
        .max_by_key(|x| dlis.entries[x.as_index()]);
    let y = unassigned_literals
        .iter()
        .filter(|y| y.is_negative())
        .max_by_key(|y| dlis.entries[y.as_index()]);

    match (x, y) {
        (Some(x), Some(y)) => match dlis.entries[x.as_index()].cmp(&dlis.entries[y.as_index()]) {
            Ordering::Greater => Some(*x),
            Ordering::Equal => Some(*y),
            Ordering::Less => Some(*y),
        },
        (Some(x), None) => Some(*x),
        (None, Some(y)) => Some(*y),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_is_positive() {
        let mut dlis = Dlis::default();
        dlis.resize(1);
        let assignment = VariableAssignment::default();

        dlis.entries[Literal::from_dimacs(1).as_index()] = 2;
        dlis.entries[Literal::from_dimacs(-1).as_index()] = 1;

        dbg!(&dlis.entries);

        assert_eq!(decide(&dlis, &assignment).unwrap(), Literal::from_dimacs(1));
    }

    #[test]
    fn test_literal_is_negative() {
        let mut dlis = Dlis::default();
        dlis.resize(1);
        let assignment = VariableAssignment::default();

        dlis.entries[Literal::from_dimacs(1).as_index()] = 1;
        dlis.entries[Literal::from_dimacs(-1).as_index()] = 2;

        assert_eq!(
            decide(&dlis, &assignment).unwrap(),
            Literal::from_dimacs(-1)
        );
        dlis.decrement(Literal::from_dimacs(-1));
        assert_eq!(
            decide(&dlis, &assignment).unwrap(),
            Literal::from_dimacs(-1)
        );
    }

    #[test]
    fn test_no_literals_left() {
        let mut dlis = Dlis::default();
        dlis.resize(1);

        let mut assignment = VariableAssignment::default();
        assignment.assign_true(Literal::from_dimacs(1));

        dlis.entries[Literal::from_dimacs(1).as_index()] = 2;
        dlis.entries[Literal::from_dimacs(-1).as_index()] = 1;

        assert!(decide(&dlis, &assignment).is_none());
    }
}
