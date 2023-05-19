use crate::assignment::AssignedValue;
use crate::cnf::CNF;
use crate::literal::Literal;
use crate::search::{search, SearchContext};

#[derive(Default)]
pub struct Solver {
    is_init: bool,
    search: SearchContext,
}

impl Solver {
    pub fn from_cnf(cnf: CNF) -> Solver {
        let mut solver = Solver::default();

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
        self.search.dlis.init(
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
