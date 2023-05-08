use crate::clause::Clause;
use crate::cnf::CNF;
use crate::literal::Variable;

/// Outer container that holds all solver data
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    clauses: Vec<Clause>,
    variables: Vec<Variable>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            clauses: vec![],
            variables: vec![],
        }
    }

    pub fn from_cnf(cnf: CNF) -> Context {
        Context {
            clauses: cnf.clauses(),
            variables: cnf.variables(),
        }
    }

    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }
}
