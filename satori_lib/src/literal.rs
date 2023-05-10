use std::fmt;
use std::ops::Not;

/// Variables are represented as numbers starting from 0
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub index: u32,
}

impl Variable {
    pub fn from_index(index: u32) -> Variable {
        Variable { index }
    }
}

/// Literals are represented as numbers starting from 0, where a literal is calculated from the
/// variable as follows:
/// ```
/// let variable = 5;
/// let positive_lit =  5 * 2;      // 10
/// let negative_lit =  5 * 2 + 1;  // 11
/// ```
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

impl Not for Literal {
    type Output = Literal;
    fn not(self) -> Self::Output {
        Literal {
            code: self.code ^ 1,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.is_positive() { 1 } else { -1 }; // 1-based dimacs encoding
        write!(f, "{}", (self.index() as i32 + 1) * sign)
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.is_positive() { 1 } else { -1 }; // 1-based dimacs encoding
        write!(f, "{}", (self.index() as i32 + 1) * sign)
    }
}
