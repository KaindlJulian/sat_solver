use crate::clause::Clause;
use crate::literal::{Literal, Variable};
use crate::parse::parse_dimacs_cnf;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct CNF {
    clauses: Vec<Clause>,
    variables: HashSet<Variable>,
}

impl CNF {
    pub fn empty() -> CNF {
        CNF::default()
    }

    pub fn from_dimacs(input: &str) -> CNF {
        let clauses = parse_dimacs_cnf(input).unwrap().1;
        CNF::from_clauses(clauses)
    }

    pub fn from_clauses(clauses: Vec<Vec<i32>>) -> CNF {
        let variables: HashSet<Variable> = HashSet::from_iter(
            clauses
                .iter()
                .flat_map(|c| c.iter().copied())
                .map(|l| Variable::from_index(l as u32)),
        );
        CNF {
            clauses: clauses
                .iter()
                .map(|c| {
                    Clause::from_lit_vec(
                        c.iter()
                            .map(|l| Literal::from_dimacs(*l))
                            .collect::<Vec<Literal>>(),
                    )
                })
                .collect(),
            variables,
        }
    }

    pub fn variables(&self) -> HashSet<Variable> {
        self.variables.clone()
    }

    pub fn clauses(&self) -> Vec<Clause> {
        self.clauses.clone()
    }
}
