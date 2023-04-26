use crate::literal::Literal;

/// Contains clause metadata
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ClauseHeader {
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

    pub fn literals(&self) -> &[Literal] {
        &self.literals
    }
}
