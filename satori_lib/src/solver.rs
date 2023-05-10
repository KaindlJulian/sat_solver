use crate::cnf::CNF;
use crate::context::Context;

#[derive(Default)]
pub struct Solver {
    context: Context,
}

impl Solver {
    pub fn from_cnf(cnf: CNF) -> Solver {
        Solver {
            context: Context::from_cnf(cnf),
        }
    }

    pub fn from_clauses(clauses: Vec<Vec<i32>>) -> Solver {
        Self::from_cnf(CNF::from_clauses(clauses))
    }

    pub fn solve(&self) -> bool {
        return false;
    }
}
