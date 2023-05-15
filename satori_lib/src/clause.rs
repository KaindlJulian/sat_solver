use crate::literal::Literal;

/// Type wrapper for better type safety. The index of the clause in `LongClauses.clauses`
pub type ClauseIndex = usize;

/// Contains metadata for a clause
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClauseMeta {
    deleted: bool,
}

/// Representation of one long clause (3+ literals) in the propagation datastructure
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Clause {
    header: ClauseMeta,
    literals: Box<[Literal]>,
}

impl Clause {
    pub fn from_lit_vec(literals: Vec<Literal>) -> Clause {
        Clause {
            header: ClauseMeta { deleted: false },
            literals: literals.into_boxed_slice(),
        }
    }

    pub fn from_literals(literals: &[Literal]) -> Clause {
        Self::from_lit_vec(literals.to_vec())
    }

    pub fn header(&self) -> &ClauseMeta {
        &self.header
    }

    pub fn literals(&self) -> &[Literal] {
        &self.literals
    }

    pub fn literals_mut(&mut self) -> &mut [Literal] {
        &mut self.literals
    }
}
