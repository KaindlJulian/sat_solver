use crate::literal::Literal;
use std::borrow::Borrow;

/// Contains metadata for a clause
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClauseHeader {
    deleted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Clause {
    header: ClauseHeader,
    literals: Box<[Literal]>,
}

impl Clause {
    pub fn from_lit_vec(literals: Vec<Literal>) -> Clause {
        Clause {
            header: ClauseHeader { deleted: false },
            literals: literals.into_boxed_slice(),
        }
    }

    pub fn header(&self) -> &ClauseHeader {
        self.header.borrow()
    }

    pub fn literals(&self) -> &[Literal] {
        &self.literals
    }
}
