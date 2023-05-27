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

    pub fn unresolved(&self, literal: Literal) -> usize {
        self.clauses
            .iter()
            .filter(|c| !c.header().is_resolved)
            .filter(|c| c.literals().contains(&literal))
            .count()
    }
}
