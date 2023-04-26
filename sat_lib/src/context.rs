use crate::clause::Clause;
use crate::cnf::CNF;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    clauses: Box<[Clause]>,
}

impl Context {
    pub fn new(cnf: CNF) -> Context {
        Context {
            clauses: cnf
                .into_clauses()
                .into_iter()
                .map(|c| Clause::from_lit_vec(c))
                .collect(),
        }
    }

    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }
}
