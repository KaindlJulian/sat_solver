use crate::assignment::VariableAssignment;
use crate::literal::Literal;

/// Returns the first unassigned variable or `None` if all variables are assigned.
pub fn first_unassigned(assignment: &VariableAssignment) -> Option<Literal> {
    assignment
        .unassigned()
        .first()
        .map(|v| Literal::from_variable(v, true))
}
