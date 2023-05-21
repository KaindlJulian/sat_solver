use crate::clause::Clause;
use crate::literal::{Literal, Variable};

/// Callbacks to update heuristic data during propagation and conflict analysis
pub trait HeuristicCallbacks {
    /// Called when a`variable` is assigned true or false
    fn assign(&mut self, _variable: Variable) {}
    /// Called when undoing an assignment for `variable`
    fn unassign(&mut self, _variable: Variable) {}
    /// Called when `clause` is marked as resolved
    fn resolved(&mut self, _clause: Clause) {}
}

impl HeuristicCallbacks for () {}
