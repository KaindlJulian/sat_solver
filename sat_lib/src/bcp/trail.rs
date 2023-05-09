use crate::literal::{Literal, Variable};
use std::collections::HashMap;

pub struct Step {
    pub assigned_literal: Literal,
}

#[derive(Default)]
pub struct Trail {
    steps: Vec<Step>,
    step_by_var: HashMap<Variable, usize>,
    propagated: usize,
}

impl Trail {
    pub fn propagated_lit_count(&self) -> usize {
        self.propagated
    }

    pub fn next_unpropagated_literal(&self) -> Option<Literal> {
        self.steps
            .get(self.propagated)
            .map(|step| step.assigned_literal)
    }
}
