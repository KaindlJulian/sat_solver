use crate::assignment::VariableAssignment;
use crate::clause::Clause;
use crate::literal::{Literal, Variable};
use std::collections::HashMap;

pub enum AssignmentCause {
    /// Guessed by the solver
    Guess,
    /// Implied by a unit clause
    Unit,
    /// Implied by a binary clause because the given literal is false.
    Binary(Literal),
    /// Implied by a long clause because all but its first literal are false.
    Long(Clause),
}

pub struct Step {
    pub assigned_literal: Literal,
    pub decision_level: u32,
    pub cause: AssignmentCause,
}

#[derive(Default)]
pub struct Trail {
    steps: Vec<Step>,
    step_index_by_var: HashMap<Variable, usize>,
    propagated: usize,
    decisions: Vec<u32>,
}

impl Trail {
    pub fn trail_index(&self, variable: Variable) -> u32 {
        *self
            .step_index_by_var
            .get(&variable)
            .expect("variable is not set") as u32
    }

    pub fn propagated_lit_count(&self) -> usize {
        self.propagated
    }

    pub fn next_unpropagated_literal(&self) -> Option<Literal> {
        self.steps
            .get(self.propagated)
            .map(|step| step.assigned_literal)
    }

    pub fn current_decision_level(&self) -> u32 {
        self.decisions.len() as u32 - 1
    }
}

/// add step to trail and assign the literal
pub fn assign(values: &mut VariableAssignment, trail: &mut Trail, step: Step) {
    trail
        .step_index_by_var
        .insert(step.assigned_literal.variable(), trail.steps.len());
    values.set_true(step.assigned_literal);
    trail.steps.push(step);
}
