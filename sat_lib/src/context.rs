use crate::clause::Clause;
use crate::cnf::CNF;
use crate::literal::Variable;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    clauses: Vec<Clause>, //TODO: split into binary and long (3+ lits) clauses
    variables: HashSet<Variable>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            clauses: vec![],
            variables: HashSet::new(),
        }
    }

    pub fn from_cnf(cnf: CNF) -> Context {
        Context {
            variables: cnf.variables(),
            clauses: cnf.clauses(),
        }
    }

    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }
}
