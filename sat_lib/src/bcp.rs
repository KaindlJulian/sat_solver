use crate::assignment::VariableAssignment;
use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::bcp::trail::Trail;
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
pub fn propagate(context: &mut Context, bcp: &mut BcpContext) -> Result<(), ()> {
    while let Some(literal) = bcp.trail.next_unpropagated_literal() {
        bcp_binary_clauses(bcp, literal)?;
        bcp_long_clauses(bcp, literal)?;
    }

    Ok(())
}

fn bcp_binary_clauses(bcp: &mut BcpContext, literal: Literal) -> Result<(), ()> {
    let negated_literal = !literal;

    let &other_lits =

    Ok(())
}

fn bcp_long_clauses(bcp: &mut BcpContext, literal: Literal) -> Result<(), ()> {
    todo!()
}
