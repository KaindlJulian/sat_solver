use crate::assignment::{AssignedValue, VariableAssignment};
use crate::clause::{Clause, ClauseIndex};
use crate::literal::Literal;

/// Holds all long clauses during propagation
#[derive(Default, Debug)]
pub struct LongClauses {
    clauses: Vec<Clause>,
}

impl LongClauses {
    pub fn clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }

    pub fn add_clause(&mut self, literals: &[Literal]) -> usize {
        let clause = Clause::from_literals(literals);
        self.clauses.push(clause);
        self.clauses.len() - 1
    }

    pub fn find_clause_mut(&mut self, index: ClauseIndex) -> &mut Clause {
        self.clauses.get_mut(index).expect("no clause found")
    }

    pub fn literals(&self, index: ClauseIndex) -> &[Literal] {
        self.clauses[index].literals()
    }

    pub fn is_resolved(&self, index: ClauseIndex, assignment: &VariableAssignment) -> bool {
        self.clauses[index]
            .literals()
            .iter()
            .any(|l| assignment.literal_value(*l) == AssignedValue::True)
    }

    pub fn unresolved(&self, literal: Literal, assignment: &VariableAssignment) -> usize {
        self.clauses
            .iter()
            .enumerate()
            .filter(|(_, c)| c.literals().contains(&literal))
            .filter(|(i, _)| self.is_resolved(*i, assignment))
            .count()
    }
}
