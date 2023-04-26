use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    index: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    code: u32,
}

impl Literal {
    // create literal from a 0-based index
    pub fn from_index(index: u32, negative: bool) -> Literal {
        Literal {
            code: index << 1 | (negative as u32),
        }
    }

    pub fn from_variable(var: Variable, negative: bool) -> Literal {
        Literal::from_index(var.index, negative)
    }

    pub fn from_dimacs(value: i32) -> Literal {
        Literal::from_index((value.abs() - 1) as u32, value < 0)
    }

    pub fn index(&self) -> usize {
        self.code as usize >> 1
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn variable(&self) -> Variable {
        Variable {
            index: self.code >> 1,
        }
    }

    pub fn is_positive(self) -> bool {
        (self.code & 1) == 0
    }

    pub fn is_negative(self) -> bool {
        !self.is_positive()
    }
}

/// Uses the 1-based dimacs encoding.
impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.is_positive() { 1 } else { -1 };
        write!(f, "{}", (self.index() as i32 + 1) * sign)
    }
}

/// Uses the 1-based dimacs encoding.
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.is_positive() { 1 } else { -1 };
        write!(f, "{}", (self.index() as i32 + 1) * sign)
    }
}
