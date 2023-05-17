use crate::literal::Literal;

// TODO only unresolved clauses?

/// Implements the  Dynamic Largest Individual Sum (DLIS) decision heuristic.
/// Approach:
///  - choose literal that satisfies most clauses
///  - for each variable x, calculate
///     - C+: number of clauses with x
///     - C-: number of clauses with -x
///  - select two variables x and y that maximize these two metrics from all the variables
///  - if C+(x) > C-(y) set x to true, else set y to false
#[derive(Default)]
pub struct DLIS {}

impl DLIS {
    pub fn decide(&self) -> Option<Literal> {
        // no unassigned variable
        None
    }
}
