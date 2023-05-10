use crate::assignment::{AssignedValue, VariableAssignment};
use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::bcp::trail::{AssignmentCause, Step, Trail};
use crate::bcp::watch::Watchlists;
use crate::context::Context;
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
    pub non_binary: LongClauses,
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
    let not_literal = !literal;

    let watches = bcp.watch.get_watchlist(&not_literal);

    Ok(())
}
