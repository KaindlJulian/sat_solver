use crate::assignment::VariableAssignment;
use crate::literal::Literal;

use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::resize::Resize;

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
    // maps a literal to its number of unresolved clauses
    lit_scores: Vec<u32>,
}

impl Resize for Dlis {
    fn resize(&mut self, variable_count: usize) {
        self.lit_scores.resize(variable_count * 2, 0);
    }
}

impl Dlis {
    /// Returns the next decision literal according to DLIS or `None` if no variables are unassigned.
    pub fn decide(
        &mut self,
        assignment: &VariableAssignment,
        long: &LongClauses,
        binary: &BinaryClauses,
    ) -> Option<Literal> {
        let unassigned_literals = assignment
            .unassigned()
            .iter()
            .flat_map(|v| [true, false].iter().map(|s| Literal::from_variable(v, *s)))
            .collect::<Vec<_>>();

        if unassigned_literals.is_empty() {
            return None;
        }

        // clear scores
        self.lit_scores.iter_mut().for_each(|s| *s = 0);

        for l in unassigned_literals {
            self.lit_scores[l.as_index()] += binary.clauses_count(l);
        }

        for c in long.clauses() {
            if c.literals().iter().any(|l| assignment.literal_is_true(*l)) {
                continue;
            }
            for l in c.literals() {
                if assignment.literal_is_unknown(*l) {
                    self.lit_scores[l.as_index()] += 1;
                }
            }
        }

        let mut max_score: u32 = 0;
        let mut max_lit_code = 0;
        for (lit_code, score) in self.lit_scores.iter().enumerate() {
            if *score > max_score {
                max_score = *score;
                max_lit_code = lit_code;
            }
        }

        if max_score > 0 {
            Some(Literal::from_code(max_lit_code as usize))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bcp::BcpContext;

    use super::*;

    #[test]
    fn test_literal_is_positive() {
        let mut bcp = BcpContext::default();
        bcp.resize(2);
        bcp.add_clause(&[Literal::from_dimacs(1), Literal::from_dimacs(-1)]);
        bcp.add_clause(&[Literal::from_dimacs(1), Literal::from_dimacs(-2)]);

        let mut dlis = Dlis::default();
        dlis.resize(2);

        let decision = dlis
            .decide(&bcp.assignment, &bcp.long_clauses, &bcp.binary_clauses)
            .unwrap();

        assert_eq!(decision, Literal::from_dimacs(1));
    }

    #[test]
    fn test_literal_is_negative() {
        let mut bcp = BcpContext::default();
        bcp.resize(2);
        bcp.add_clause(&[Literal::from_dimacs(-1), Literal::from_dimacs(1)]);
        bcp.add_clause(&[Literal::from_dimacs(-1), Literal::from_dimacs(2)]);

        let mut dlis = Dlis::default();
        dlis.resize(2);

        let decision = dlis
            .decide(&bcp.assignment, &bcp.long_clauses, &bcp.binary_clauses)
            .unwrap();

        assert_eq!(decision, Literal::from_dimacs(-1));
    }

    #[test]
    fn test_no_literals_left() {
        let mut bcp = BcpContext::default();
        bcp.resize(2);
        bcp.add_clause(&[Literal::from_dimacs(-1), Literal::from_dimacs(1)]);
        bcp.add_clause(&[Literal::from_dimacs(-1), Literal::from_dimacs(2)]);
        bcp.assignment.assign_true(Literal::from_dimacs(1));
        bcp.assignment.assign_true(Literal::from_dimacs(2));

        let mut dlis = Dlis::default();
        dlis.resize(2);

        let decision = dlis.decide(&bcp.assignment, &bcp.long_clauses, &bcp.binary_clauses);

        assert!(decision.is_none());
    }
}
