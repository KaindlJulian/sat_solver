use crate::clause::Clause;
use crate::literal::{Literal, Variable};
use crate::parse::parse_dimacs_cnf;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct CNF {
    clauses: Vec<Clause>,
    variables: Vec<Variable>,
}

impl CNF {
    pub fn empty() -> CNF {
        CNF::default()
    }

    pub fn from_dimacs(input: &str) -> CNF {
        let clauses = parse_dimacs_cnf(input).expect("parsing error").1;
        CNF::from_clauses(clauses)
    }

    pub fn from_clauses(clauses: Vec<Vec<i32>>) -> CNF {
        let max_var = clauses
            .iter()
            .flat_map(|c| c.iter())
            .map(|l| l.abs())
            .max()
            .unwrap_or(0) as u32;

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
            variables: (0..max_var)
                .map(|i| Variable::from_index(i))
                .collect::<Vec<Variable>>(),
        }
    }

    pub fn variables(&self) -> Vec<Variable> {
        self.variables.clone()
    }

    pub fn clauses(&self) -> Vec<Clause> {
        self.clauses.clone()
    }
}
