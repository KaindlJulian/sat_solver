use crate::assignment::AssignedValue;
use crate::cnf::CNF;
use crate::literal::Literal;
use crate::resize::Resize;
use crate::search::{search, SearchContext};

#[derive(Default, Debug)]
pub struct Solver {
    search: SearchContext,
    variable_count: usize,
}

impl Solver {
    pub fn from_cnf(cnf: CNF) -> Solver {
        let mut solver = Solver {
            search: Default::default(),
            variable_count: cnf.variable_count(),
        };

        solver.search.resize(solver.variable_count);

        for c in cnf.clauses().iter() {
            solver.add_clause(c.literals());
        }

        solver.search.dlis.build_dlis_entries(
            &solver.search.bcp.binary_clauses,
            &solver.search.bcp.long_clauses,
        );

        solver
    }

    pub fn from_clauses(clauses: Vec<Vec<i32>>) -> Solver {
        Self::from_cnf(CNF::from_clauses(&clauses))
    }

    pub fn with_dlis(mut self) -> Self {
        self.search.use_dlis = true;
        self
    }

    /// Adds a clause to the formula, can break invariants if adding new variables
    pub fn add_clause(&mut self, clause: &[Literal]) {
        self.search.bcp.add_clause(clause);
    }

    /// Check satisfiability of the formula
    pub fn solve(&mut self) -> bool {
        loop {
            if let Some(result) = search(&mut self.search) {
                return result;
            }
        }
    }

    /// Returns the (partial) assignment
    pub fn assignment(&self) -> Vec<Literal> {
        self.search.bcp.assignment.partial()
    }

    /// Returns the value assigned to a literal
    pub fn value(&self, literal: Literal) -> Option<bool> {
        match self.search.bcp.assignment.literal_value(literal) {
            AssignedValue::True => Some(true),
            AssignedValue::False => Some(false),
            AssignedValue::Unknown => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_formula() {
        let file = "../test_formulas/add32.unsat";
        let mut solver = Solver::from_cnf(CNF::from_file_str(file));
        assert_eq!(solver.solve(), file.contains(".sat"));
    }

    #[test]
    fn test_all_formulas() {
        for entry in fs::read_dir(PathBuf::from("../test_formulas")).unwrap() {
            let file = entry.unwrap();
            dbg!(file.file_name());
            let mut solver = Solver::from_cnf(CNF::from_file(file.path()));
            let sat = solver.solve();
            dbg!(sat);
            assert_eq!(sat, file.file_name().to_str().unwrap().contains(".sat"));
        }
    }
}
