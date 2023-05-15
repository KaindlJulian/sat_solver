use crate::bcp::BcpContext;
use crate::clause::ClauseIndex;
use crate::literal::Literal;

/// Set of literals that make the formula unsat when conjunctively added to the formula
pub enum Conflict {
    BinaryClause([Literal; 2]),
    LongClause(ClauseIndex),
}

impl Conflict {
    pub fn get_literals<'a>(&'a self, context: &'a BcpContext) -> &[Literal] {
        match self {
            Conflict::BinaryClause(literals) => literals,
            Conflict::LongClause(clause_index) => context.long_clauses.get_literals(*clause_index),
        }
    }
}
