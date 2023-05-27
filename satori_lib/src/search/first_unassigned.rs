use crate::{assignment::VariableAssignment, literal::Literal};

/// Returns the first unassigned variable or `None` if all variables are assigned.
#[allow(dead_code)]
pub fn first_unassigned(assignment: &VariableAssignment) -> Option<Literal> {
    assignment
        .unassigned()
        .first()
        .map(|v| Literal::from_variable(v, true))
}
