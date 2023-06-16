use std::fmt;
use std::ops::Not;

/// Variables are represented as numbers starting from 0
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    index: u32,
}

impl Variable {
    pub fn from_index(index: u32) -> Variable {
        Variable { index }
    }

    pub fn from_dimacs(number: i32) -> Variable {
        Variable {
            index: number.unsigned_abs() - 1,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn as_index(&self) -> usize {
        self.index as usize
    }
}

/// Literals are represented as numbers starting from 0, where a literal is calculated from the
/// variable as follows:
/// ```
/// let variable = 5;
/// let positive_lit =  5 * 2;      // 10
/// let negative_lit =  5 * 2 + 1;  // 11
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal {
    code: u32,
}

impl Literal {
    pub fn from_code(code: usize) -> Literal {
        Literal { code: code as u32 }
    }

    pub fn from_index(index: u32, positive: bool) -> Literal {
        Literal {
            code: index << 1 | (!positive as u32),
        }
    }

    pub fn from_variable(var: &Variable, positive: bool) -> Literal {
        Literal::from_index(var.index, positive)
    }

    pub fn from_dimacs(value: i32) -> Literal {
        assert_ne!(value, 0, "invalid dimacs literal");
        Literal::from_index((value.abs() - 1) as u32, value > 0)
    }

    pub fn as_code(&self) -> u32 {
        self.code
    }

    pub fn as_index(&self) -> usize {
        self.as_code() as usize
    }

    pub fn as_dimacs_integer(&self) -> i32 {
        let index = self.code as i32 >> 1;
        if self.is_positive() {
            index + 1
        } else {
            0 - (index + 1)
        }
    }

    pub fn variable(&self) -> Variable {
        Variable {
            index: self.code >> 1,
        }
    }

    pub fn is_positive(self) -> bool {
        (self.code & 1) == 0 // even codes represent positive literals
    }

    pub fn is_negative(self) -> bool {
        !self.is_positive()
    }
}

impl Not for Literal {
    type Output = Literal;
    fn not(self) -> Self::Output {
        Literal {
            code: self.code ^ 1, // swap lsb
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.as_dimacs_integer()))
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.as_dimacs_integer()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_literal_mapping() {
        let var_index = 3;
        let var = Variable { index: var_index };

        assert_eq!(var, Literal::from_index(var_index, true).variable());
        assert_eq!(var, Literal::from_index(var_index, false).variable());

        assert_eq!(var_index * 2, Literal::from_index(3, true).code);
        assert_eq!(var_index * 2 + 1, Literal::from_index(3, false).code);
    }

    #[test]
    fn test_literal_negation() {
        let code = 0;
        let literal = Literal { code };

        assert!(literal.is_positive());
        assert!(!literal.is_negative());
        assert_eq!(code + 1, (!literal).code);
        assert_eq!(code, (!(!literal)).code);
    }

    #[test]
    fn test_display() {
        let literal = Literal::from_index(0, true);
        assert_eq!(1, literal.as_dimacs_integer());
        assert_eq!(-1, (!literal).as_dimacs_integer());
    }
}
