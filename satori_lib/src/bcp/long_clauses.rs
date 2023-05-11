use crate::clause::Clause;

pub type ClauseIndex = usize;

#[derive(Default)]
pub struct LongClauses {
    pub clauses: Vec<Clause>,
}

impl LongClauses {
    pub fn find_clause_mut(&mut self, index: ClauseIndex) -> &mut Clause {
        self.clauses.get_mut(index).expect("no clause found")
    }
}
