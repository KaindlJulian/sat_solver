use crate::literal::Literal;
use crate::parse::parse_dimacs_cnf;

#[derive(Default, Debug)]
pub struct CNF {
    clauses: Vec<Vec<Literal>>,
    variable_count: usize,
}

impl CNF {
    pub fn new(clauses: Vec<Vec<Literal>>, variable_count: usize) -> CNF {
        CNF {
            clauses,
            variable_count,
        }
    }

    pub fn empty() -> CNF {
        CNF::default()
    }

    pub fn from_dimacs(input: &str) -> CNF {
        parse_dimacs_cnf(input).unwrap().1
    }

    pub fn into_clauses(self) -> Vec<Vec<Literal>> {
        self.clauses
    }
}
