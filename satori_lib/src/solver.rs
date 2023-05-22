use crate::assignment::AssignedValue;
use crate::cnf::CNF;
use crate::literal::Literal;
use crate::search::{search, SearchContext};

#[derive(Default, Debug)]
pub struct Solver {
    is_init: bool,
    search: SearchContext,
    variable_count: usize,
}

impl Solver {
    pub fn from_cnf(cnf: CNF) -> Solver {
        let mut solver = Solver {
            is_init: false,
            search: Default::default(),
            variable_count: cnf.variables().len(),
        };
        solver.search.dlis.resize(solver.variable_count);

        for c in cnf.clauses().iter() {
            solver.add_clause(c.literals());
        }

        solver
    }

    pub fn from_clauses(clauses: Vec<Vec<i32>>) -> Solver {
        Self::from_cnf(CNF::from_clauses(&clauses))
    }

    /// Adds a clause to the formula
    pub fn add_clause(&mut self, clause: &[Literal]) {
        if self.is_init {
            panic!("must be uninitialized to add clauses");
        }
        self.search.bcp.add_clause(clause);
    }

    pub fn init(&mut self) {
        self.is_init = true;
        self.search.bcp.init();
        self.search.dlis.build_dlis_entries(
            &self.search.bcp.binary_clauses,
            &self.search.bcp.long_clauses,
        )
    }

    /// Check satisfiability of the formula
    pub fn solve(&mut self) -> bool {
        if !self.is_init {
            self.init();
        }

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
        let file = "../test_formulas/add32R.unsat";
        let mut solver = Solver::from_cnf(CNF::from_file_str(file));
        let sat = solver.solve();
        assert_eq!(sat, file.contains(".sat"));
    }

    #[test]
    fn test_all_formulas() {
        let excluded = vec!["add64.unsat", "add128.unsat"];
        for entry in fs::read_dir(PathBuf::from("../test_formulas")).unwrap() {
            let file = entry.unwrap();
            if excluded.contains(&&file.file_name().to_str().unwrap()) {
                continue;
            }
            dbg!(file.file_name());
            let mut solver = Solver::from_cnf(CNF::from_file(file.path()));
            let sat = solver.solve();
            dbg!(sat);
            assert_eq!(sat, file.file_name().to_str().unwrap().contains(".sat"));
        }
    }
}
