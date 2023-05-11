use crate::literal::Literal;
use std::borrow::Borrow;

/// Contains metadata for a clause
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClauseMeta {
    deleted: bool,
}

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

    pub fn header(&self) -> &ClauseMeta {
        self.header.borrow()
    }

    pub fn literals(&self) -> &[Literal] {
        &self.literals
    }
}
