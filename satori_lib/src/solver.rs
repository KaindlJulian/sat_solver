use crate::assignment::AssignedValue;
use crate::bcp::BcpContext;
use crate::cnf::CNF;
use crate::literal::Literal;

#[derive(Default)]
pub struct Solver {
    is_init: bool,
    bcp: BcpContext,
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
        self.bcp.add_clause(clause);
    }

    pub fn init(&mut self) {
        self.is_init = true;
        self.bcp.init();
    }

    /// Check satisfiability of the formula
    pub fn solve(&mut self) -> bool {
        if !self.is_init {
            self.init();
        }
        return false;
    }

    /// Returns the current partial assignment
    pub fn get_model(&self) -> &[Literal] {
        todo!()
    }

    /// Returns the value assigned to a literal
    pub fn value(&self, literal: Literal) -> AssignedValue {
        todo!()
    }
}
