use crate::bcp::conflict::Conflict;
use crate::bcp::trail::{Reason, Step, Trail};
use crate::bcp::{trail, AddedClause, BcpContext};
use crate::literal::Literal;

/// Temporary data during conflict analysis
#[derive(Default, Debug)]
pub struct ConflictAnalysis {
    /// maps a step index to true if the literal is in the current clause
    conflict_literals: Vec<bool>,

    /// The derived clause, 1-UIP
    derived_clause: Vec<Literal>,

    current_level_lit_count: usize,
}

/// analyzes a  conflict
pub fn analyze(conflict: Conflict, analysis: &mut ConflictAnalysis, bcp: &mut BcpContext) {
    assert_ne!(
        bcp.trail.current_decision_level(),
        trail::TOP_DECISION_LEVEL
    );

    let new_len = analysis
        .conflict_literals
        .len()
        .max(bcp.trail.steps().len());
    analysis.conflict_literals.resize(new_len, false);

    // derive the first UIP
    derive_1_uip(conflict, analysis, bcp);

    let target_decision_level = prepare_for_backtracking(analysis, bcp);

    trail::backtrack(bcp, target_decision_level);
    learn_and_assign(analysis, bcp);
}

/// derives the first unique implication point clause from given implication graph and conflict
pub fn derive_1_uip(conflict: Conflict, analysis: &mut ConflictAnalysis, bcp: &mut BcpContext) {
    analysis.derived_clause.clear();

    for &literal in conflict.get_literals(bcp) {
        add_literal(analysis, &bcp.trail, literal)
    }

    for step_index in (0..bcp.trail.steps().len()).rev() {
        if !std::mem::replace(&mut analysis.conflict_literals[step_index], false) {
            continue;
        }

        let step = &bcp.trail.steps()[step_index];

        analysis.current_level_lit_count -= 1;

        if analysis.current_level_lit_count == 0 {
            // last literal at current decision level -> found a 1-UIP
            for &literal in &analysis.derived_clause {
                let step_index = bcp.trail.step_index(literal.variable());
                analysis.conflict_literals[step_index] = false;
            }
            analysis.derived_clause.push(!step.assigned_literal);
            break;
        } else {
            // add asserting literals (that caused propagation) to get resolvent
            for &asserting_literal in step.reason.get_falsified_literals(bcp) {
                add_literal(analysis, &bcp.trail, asserting_literal);
            }
        }
    }

    assert_eq!(analysis.current_level_lit_count, 0);
}

fn add_literal(analysis: &mut ConflictAnalysis, trail: &Trail, literal: Literal) {
    let step_index = trail.step_index(literal.variable());
    let lit_decision_level = trail.steps()[step_index].decision_level;

    // If the literal is assigned at level zero, it is always falsified and we can directly
    // remove it.
    if lit_decision_level == trail::TOP_DECISION_LEVEL {
        return;
    }

    // If the literal is already added, don't add it a second time.
    if std::mem::replace(&mut analysis.conflict_literals[step_index], true) {
        return;
    }

    if lit_decision_level == trail.current_decision_level() {
        // If the literal is assigned at the current decision level, we may want
        // to resolve on it.
        analysis.current_level_lit_count += 1;
    } else {
        // If the literal is assigned at a non-zero decision level, we will not
        // resolve on it so it will be part of the derived clause.
        analysis.derived_clause.push(literal);
    }
}

fn prepare_for_backtracking(conflict: &mut ConflictAnalysis, bcp: &mut BcpContext) -> u32 {
    let clause_length = conflict.derived_clause.len();
    conflict.derived_clause.swap(0, clause_length - 1);
    let mut backtrack_level = trail::TOP_DECISION_LEVEL;

    if clause_length > 1 {
        let mut max_step_index = bcp.trail.step_index(conflict.derived_clause[1].variable());
        for i in 2..clause_length {
            let step_index = bcp.trail.step_index(conflict.derived_clause[i].variable());
            if step_index > max_step_index {
                max_step_index = step_index;
                conflict.derived_clause.swap(1, i);
            }
        }

        backtrack_level = bcp.trail.steps()[max_step_index].decision_level;
    }

    backtrack_level
}

/// adds the asserting clause to the formula and assigns the newly asserted literal
fn learn_and_assign(conflict: &mut ConflictAnalysis, bcp: &mut BcpContext) {
    let reason = match bcp.add_clause(&conflict.derived_clause) {
        AddedClause::Binary([_, b]) => Some(Reason::Binary(b)),
        AddedClause::Long(clause_index) => Some(Reason::Long(clause_index)),
        _ => None,
    };

    if let Some(reason) = reason {
        let step = Step {
            assigned_literal: conflict.derived_clause[0],
            decision_level: bcp.trail.current_decision_level(),
            reason,
        };

        trail::assign(&mut bcp.assignment, &mut bcp.trail, step)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assignment::AssignedValue;
    use crate::bcp::trail::decide_and_assign;
    use crate::bcp::{propagate, BcpContext};
    use crate::cnf::CNF;
    use crate::literal::Variable;
    use crate::resize::Resize;

    #[test]
    fn test_learn_unit_clause() {
        let cnf = CNF::from_dimacs("-1 2 0\n-1 3 0\n-2 -3 0\n-4 1 0\n");

        let mut analysis = ConflictAnalysis::default();
        let mut bcp = BcpContext::default();
        bcp.resize(cnf.variable_count());
        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }
        bcp.init();

        decide_and_assign(&mut bcp, Literal::from_dimacs(4));

        let conflict = propagate(&mut bcp).unwrap_err();
        analyze(conflict, &mut analysis, &mut bcp);

        assert_eq!(analysis.derived_clause, vec![Literal::from_dimacs(-1)]);

        propagate(&mut bcp).unwrap();

        assert_eq!(
            bcp.assignment.value(Variable::from_dimacs(1)),
            AssignedValue::False
        );
        assert_eq!(
            bcp.trail
                .get_step_for_variable(Variable::from_dimacs(1))
                .reason,
            Reason::Unit
        );
        assert_eq!(
            bcp.assignment.value(Variable::from_dimacs(4)),
            AssignedValue::False
        );
    }

    #[test]
    fn long_clause() {
        let cnf = CNF::from_dimacs("-1 2 0\n-1 3 0\n-2 -3 -4 -5 0\n-6 7 0\n-7 4 0\n-7 5 0\n");

        let mut analysis = ConflictAnalysis::default();
        let mut bcp = BcpContext::default();
        bcp.resize(cnf.variable_count());
        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }
        bcp.init();

        decide_and_assign(&mut bcp, Literal::from_dimacs(1));

        propagate(&mut bcp).unwrap();

        decide_and_assign(&mut bcp, Literal::from_dimacs(6));

        let conflict = propagate(&mut bcp).unwrap_err();

        analyze(conflict, &mut analysis, &mut bcp);

        propagate(&mut bcp).unwrap();

        assert_eq!(
            bcp.assignment.literal_value(Literal::from_dimacs(-7)),
            AssignedValue::True
        );

        if let Reason::Long(clause) = bcp
            .trail
            .get_step_for_variable(Variable::from_dimacs(7))
            .reason
        {
            assert_eq!(bcp.long_clauses.literals(clause), analysis.derived_clause);
            analysis.derived_clause.sort_unstable();
            assert_eq!(
                analysis.derived_clause,
                vec![
                    Literal::from_dimacs(-2),
                    Literal::from_dimacs(-3),
                    Literal::from_dimacs(-7)
                ]
            );
        } else {
            panic!("expected a long clause")
        }
        assert_eq!(
            bcp.assignment.literal_value(Literal::from_dimacs(-6)),
            AssignedValue::True
        );
    }

    #[test]
    fn binary_clause() {
        let cnf = CNF::from_dimacs("-1 2 0\n-1 3 0\n-2 -4 -5 0\n-6 7 0\n-7 4 0\n-7 5 0\n");

        let mut analysis = ConflictAnalysis::default();
        let mut bcp = BcpContext::default();
        bcp.resize(cnf.variable_count());
        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }
        bcp.init();

        decide_and_assign(&mut bcp, Literal::from_dimacs(1));

        propagate(&mut bcp).unwrap();

        decide_and_assign(&mut bcp, Literal::from_dimacs(6));

        let conflict = propagate(&mut bcp).unwrap_err();

        analyze(conflict, &mut analysis, &mut bcp);

        propagate(&mut bcp).unwrap();

        assert_eq!(
            bcp.assignment.literal_value(Literal::from_dimacs(-7)),
            AssignedValue::True
        );
        assert_eq!(
            bcp.trail
                .get_step_for_variable(Variable::from_dimacs(7))
                .reason,
            Reason::Binary(Literal::from_dimacs(-2))
        );

        analysis.derived_clause.sort_unstable(); // not used below, we can clobber it
        assert_eq!(
            analysis.derived_clause,
            vec![Literal::from_dimacs(-2), Literal::from_dimacs(-7)]
        );
        assert_eq!(
            bcp.assignment.literal_value(Literal::from_dimacs(-6)),
            AssignedValue::True
        );
    }
}
