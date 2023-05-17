use crate::assignment::AssignedValue;
use crate::bcp::conflict::Conflict;
use crate::bcp::trail::{Reason, Step, StepIndex, Trail};
use crate::bcp::{trail, AddedClause, BcpContext};
use crate::literal::Literal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Temporary data for analyzing conflicts
#[derive(Default)]
pub struct ConflictAnalysis {
    conflicting_assignment: HashMap<StepIndex, AssignedValue>,

    /// The derived clause, 1-UIP
    uip: Vec<Literal>,

    current_level_lit_count: usize,
}

pub fn analyze(conflict: Conflict, analysis: &mut ConflictAnalysis, bcp: &mut BcpContext) {
    assert_ne!(
        bcp.trail.current_decision_level(),
        trail::TOP_DECISION_LEVEL
    );

    derive_1_uip(conflict, analysis, bcp);

    let target_decision_level = prepare_for_backtracking(analysis, bcp);

    trail::backtrack(bcp, target_decision_level);
    learn_and_assign(analysis, bcp);
}

/// derives the first unique implication point clause from given implication graph and conflict
pub fn derive_1_uip(conflict: Conflict, analysis: &mut ConflictAnalysis, bcp: &mut BcpContext) {
    analysis.uip.clear();

    for &literal in conflict.get_literals(bcp) {
        add_literal(analysis, &bcp.trail, literal)
    }

    for step_index in 0..bcp.trail.steps().len() {
        match analysis.conflicting_assignment.entry(step_index) {
            Entry::Occupied(mut e) => {
                let was_false = e.get() == &AssignedValue::False;
                e.insert(AssignedValue::True);
                if was_false {
                    continue;
                }
            }
            Entry::Vacant(e) => {
                e.insert(AssignedValue::True);
            }
        }

        let step = &bcp.trail.steps()[step_index];

        analysis.current_level_lit_count -= 1;

        if analysis.current_level_lit_count == 0 {
            for &literal in &analysis.uip {
                let step_index = bcp.trail.step_index(literal.variable());
                analysis
                    .conflicting_assignment
                    .insert(step_index, AssignedValue::False);
            }
            analysis.uip.push(!step.assigned_literal);
            break;
        } else {
            for &asserting_literal in step.reason.get_false_literals(bcp) {
                add_literal(analysis, &bcp.trail, asserting_literal);
            }
        }
    }

    assert_eq!(analysis.current_level_lit_count, 0);
}

fn add_literal(conflict: &mut ConflictAnalysis, trail: &Trail, literal: Literal) {
    let step_index = trail.step_index(literal.variable());
    let lit_decision_level = trail.steps()[step_index].decision_level;
    // If the literal is assigned at level zero, it is always falsified and we can directly
    // remove it.
    if lit_decision_level == trail::TOP_DECISION_LEVEL {
        return;
    }

    // If the literal is already added, don't add it a second time.
    if conflict
        .conflicting_assignment
        .get(&step_index)
        .map(|v| *v == AssignedValue::True)
        .unwrap_or(false)
    {
        return;
    }

    if lit_decision_level == trail.current_decision_level() {
        // If the literal is assigned at the current decision level, we may want
        // to resolve on it.
        conflict.current_level_lit_count += 1;
    } else {
        // If the literal is assigned at a non-zero decision level, we will not
        // resolve on it so it will be part of the derived clause.
        conflict.uip.push(literal);
    }
}

fn prepare_for_backtracking(conflict: &mut ConflictAnalysis, bcp: &mut BcpContext) -> u32 {
    let uip_len = conflict.uip.len();
    conflict.uip.swap(0, uip_len - 1);
    let mut backtrack_level = trail::TOP_DECISION_LEVEL;

    if uip_len > 1 {
        let mut max_step_index = bcp.trail.step_index(conflict.uip[1].variable());
        for i in 2..uip_len {
            let trail_index = bcp.trail.step_index(conflict.uip[i].variable());
            if trail_index > max_step_index {
                max_step_index = trail_index;
                conflict.uip.swap(1, i);
            }
        }

        backtrack_level = bcp.trail.steps()[max_step_index].decision_level;
    }

    backtrack_level
}

/// adds the asserting clause to the formula and assigns the newly asserted literal
fn learn_and_assign(conflict: &mut ConflictAnalysis, bcp: &mut BcpContext) {
    let reason = match bcp.add_clause(&conflict.uip) {
        AddedClause::Binary([_, b]) => Some(Reason::Binary(b)),
        AddedClause::Long(clause_index) => Some(Reason::Long(clause_index)),
        _ => None, //TODO maybe panic here
    };

    if let Some(reason) = reason {
        let step = Step {
            assigned_literal: conflict.uip[0],
            decision_level: bcp.trail.current_decision_level(),
            reason,
        };

        trail::assign(&mut bcp.assignment, &mut bcp.trail, step)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bcp::{propagate, BcpContext};
    use crate::cnf::CNF;
    use crate::literal::Variable;

    #[test]
    fn test_learn_unit_clause() {
        let mut bcp = BcpContext::default();
        let cnf = CNF::from_str("-1 2 0\n-1 3 0\n-2 -3 0\n-4 1 0\n");
        let mut analysis = ConflictAnalysis::default();

        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }

        bcp.init();
        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(4));

        let conflict = propagate(&mut bcp).unwrap_err();
        analyze(conflict, &mut analysis, &mut bcp);

        assert_eq!(analysis.uip, vec![Literal::from_dimacs(-1)]);

        propagate(&mut bcp).unwrap();

        assert_eq!(
            bcp.assignment.get_value(Variable::from_dimacs(1)),
            AssignedValue::False
        );
        assert_eq!(
            bcp.trail
                .get_step_for_variable(Variable::from_dimacs(1))
                .reason,
            Reason::Unit
        );
        assert_eq!(
            bcp.assignment.get_value(Variable::from_dimacs(4)),
            AssignedValue::False
        );
    }
}
