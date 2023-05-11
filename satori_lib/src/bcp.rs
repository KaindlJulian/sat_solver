use crate::assignment::{AssignedValue, VariableAssignment};
use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::bcp::trail::{AssignmentCause, Step, Trail};
use crate::bcp::watch::Watchlists;
use crate::literal::Literal;

pub mod binary_clauses;
pub mod long_clauses;
pub mod trail;
pub mod watch;

/// data for bcp and backtracking
#[derive(Default)]
pub struct BcpContext {
    pub is_unsat: bool,
    pub assignment: VariableAssignment,
    pub binary_clauses: BinaryClauses,
    pub long_clauses: LongClauses,
    pub watch: Watchlists,
    pub trail: Trail,
}

/// Execute one run of BCP
pub fn propagate(bcp: &mut BcpContext) -> Result<(), ()> {
    while let Some(literal) = bcp.trail.next_unpropagated_literal() {
        bcp_binary_clauses(bcp, literal)?;
        bcp_long_clauses(bcp, literal)?;
    }

    Ok(())
}

fn bcp_binary_clauses(bcp: &mut BcpContext, literal: Literal) -> Result<(), ()> {
    // look at all clauses containing !literal
    let not_literal = !literal;

    for &implied_literal in bcp.binary_clauses.get_clauses(not_literal) {
        match bcp.assignment.get_literal_value(implied_literal) {
            // clause is in conflict
            AssignedValue::False => return Err(()),
            // clause is already satisfied
            AssignedValue::True => {
                continue;
            }
            // clause is unit, propagate the implied literal
            AssignedValue::Unknown => {
                let step = Step {
                    assigned_literal: implied_literal,
                    decision_level: bcp.trail.current_decision_level(),
                    cause: AssignmentCause::Binary(not_literal),
                };
                trail::assign(&mut bcp.assignment, &mut bcp.trail, step);
            }
        }
    }

    Ok(())
}

fn bcp_long_clauses(bcp: &mut BcpContext, literal: Literal) -> Result<(), ()> {
    // we look for clauses with !literal
    let not_literal = !literal;

    let mut watches = bcp.watch.take_watchlist(not_literal);
    let mut result = Ok(());

    'watches: for watch in watches.iter_mut() {
        let clause = bcp.long_clauses.find_clause_mut(watch.clause_index);
        let literals = clause.literals();

        let other_watched_literal = match not_literal == literals[0] {
            true => literals[1],
            false => literals[0],
        };

        // the clause is already satisfied by the other watched literal
        if bcp.assignment.get_literal_value(other_watched_literal) == AssignedValue::True {
            continue;
        }

        // search other clauses to find watched literal replacement
        for i in 2..literals.len() {
            let current_lit = literals[i];
            // check for non-false literal
            match bcp.assignment.get_literal_value(current_lit) {
                AssignedValue::True | AssignedValue::Unknown => {
                    // watch current literal
                    // change literal order to keep the watched literals at index 0 and 1
                }
                _ => {}
            }
            continue 'watches;
        }

        // did not find a non-false non-watched literal
        match bcp.assignment.get_literal_value(other_watched_literal) {
            // clause is unit
            AssignedValue::True | AssignedValue::Unknown => {
                // propagate other watched literal
            }
            // conflict, all literals are false
            AssignedValue::False => {
                result = Err(());
                break;
            }
        }
    }

    return result;
}
