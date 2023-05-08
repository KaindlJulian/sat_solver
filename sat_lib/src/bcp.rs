use crate::assignment::VariableAssignment;
use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::long_clauses::LongClauses;
use crate::bcp::watch::Watchlists;
use crate::context::Context;

pub mod binary_clauses;
pub mod long_clauses;
pub mod watch;

/// data for bcp and backtracking
#[derive(Default)]
pub struct BCP {
    pub is_unsat: bool,
    pub assignment: VariableAssignment,
    pub binary_clauses: BinaryClauses,
    pub non_binary: LongClauses,
    pub watch: Watchlists,
}

/// Execute one step of BCP
pub fn propagate(context: &mut Context, bcp: &mut BCP) -> Result<(), ()> {
    Ok(())
}
