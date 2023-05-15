use crate::assignment::VariableAssignment;
use crate::literal::{Literal, Variable};
use std::collections::HashMap;
use crate::bcp::BcpContext;
use crate::clause::ClauseIndex;

pub enum Reason {
    /// Decided by the solver/heuristic
    SolverDecision,
    /// Implied by a unit clause
    Unit,
    /// Implied by a binary clause because the given other literal of the binary clause is false.
    Binary(Literal),
    /// Implied by a long clause because all but its first literal are false.
    Long(ClauseIndex),
}

pub struct Step {
    pub assigned_literal: Literal,
    pub decision_level: u32,
    pub reason: Reason,
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

    pub fn propagated(&self) -> usize {
        self.propagated
    }

    pub fn increase_propagated(&mut self) {
        self.propagated += 1;
    }

    pub fn next_unpropagated_literal(&self) -> Option<Literal> {
        self.steps
            .get(self.propagated)
            .map(|step| step.assigned_literal)
    }

    pub fn current_decision_level(&self) -> u32 {
        self.decisions.len() as u32 - 1
    }

    pub fn step_history(&self) -> &Vec<Step> {
        &self.steps
    }
}

/// adds given step to the trail, assigning the literal
pub fn assign(values: &mut VariableAssignment, trail: &mut Trail, step: Step) {
    trail
        .step_index_by_var
        .insert(step.assigned_literal.variable(), trail.steps.len());
    values.set_true(step.assigned_literal);
    trail.steps.push(step);
}

/// adds a solver decision to the trail, assigning the literal
pub fn decide_and_assign(bcp: &mut BcpContext, literal: Literal) {
    bcp.trail.decisions.push(bcp.trail.steps.len() as u32);
    let step = Step {
        assigned_literal: literal,
        decision_level: bcp.trail.current_decision_level(),
        reason: Reason::SolverDecision,
    };
    assign(&mut bcp.assignment, &mut bcp.trail, step);
}
