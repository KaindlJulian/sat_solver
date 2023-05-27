use crate::assignment::{AssignedValue, VariableAssignment};
use crate::bcp::binary_clauses::BinaryClauses;
use crate::bcp::conflict::Conflict;
use crate::bcp::long_clauses::LongClauses;
use crate::bcp::trail::{Reason, Step, Trail};
use crate::bcp::watch::Watchlists;
use crate::clause::ClauseIndex;
use crate::literal::Literal;
use crate::resize::Resize;

pub mod binary_clauses;
pub mod conflict;
pub mod long_clauses;
pub mod trail;
mod watch;

/// Reference to an added clause
pub enum AddedClause {
    Empty,
    Unit(Literal),
    Binary([Literal; 2]),
    Long(ClauseIndex),
}

/// data for bcp and backtracking
#[derive(Default, Debug)]
pub struct BcpContext {
    pub is_unsat: bool,
    pub assignment: VariableAssignment,
    pub binary_clauses: BinaryClauses,
    pub long_clauses: LongClauses,
    pub watch: Watchlists,
    pub trail: Trail,
}

impl Resize for BcpContext {
    fn resize(&mut self, var_count: usize) {
        self.assignment.resize(var_count);
        self.binary_clauses.resize(var_count);
    }
}

impl BcpContext {
    pub fn init(&mut self) {
        self.watch.build_watchlists(&self.long_clauses);
    }

    pub fn add_clause(&mut self, literals: &[Literal]) -> AddedClause {
        match *literals {
            [] => {
                self.is_unsat = true;
                AddedClause::Empty
            }
            [a] => {
                if self.assignment.literal_value(a) == AssignedValue::False {
                    self.is_unsat = true;
                    return AddedClause::Empty;
                }
                let step = Step {
                    assigned_literal: a,
                    decision_level: trail::TOP_DECISION_LEVEL,
                    reason: Reason::Unit,
                };
                trail::assign(&mut self.assignment, &mut self.trail, step);
                AddedClause::Unit(a)
            }
            [a, b] => {
                self.binary_clauses.add_clause([a, b]);
                AddedClause::Binary([a, b])
            }
            [a, b, ..] => {
                let index = self.long_clauses.add_clause(literals);
                self.watch.watch_clause(index, [a, b]);
                AddedClause::Long(index)
            }
        }
    }
}

/// Repeatedly execute BCP until a fixpoint or conflict is reached
pub fn propagate(bcp: &mut BcpContext) -> Result<(), Conflict> {
    while let Some(literal) = bcp.trail.next_unpropagated_literal() {
        bcp_binary_clauses(bcp, literal)?;
        bcp_long_clauses(bcp, literal)?;
        bcp.trail.increase_propagated();
    }

    // fixpoint
    Ok(())
}

fn bcp_binary_clauses(bcp: &mut BcpContext, literal: Literal) -> Result<(), Conflict> {
    // look at all clauses containing !literal
    let not_literal = !literal;

    for mut entry in bcp.binary_clauses.clauses_mut(!literal) {
        match bcp.assignment.literal_value(entry.other_literal) {
            // the other literal is true -> already satisfied
            AssignedValue::True => {
                entry.header.is_resolved = true;
                continue;
            }
            // the other literal is false -> conflict
            AssignedValue::False => {
                entry.header.is_resolved = false;
                return Err(Conflict::BinaryClause([not_literal, entry.other_literal]));
            }
            // the other literal is unassigned -> clause became unit, propagate the other literal
            AssignedValue::Unknown => {
                let step = Step {
                    assigned_literal: entry.other_literal,
                    decision_level: bcp.trail.current_decision_level(),
                    reason: Reason::Binary(not_literal),
                };
                trail::assign(&mut bcp.assignment, &mut bcp.trail, step);
                entry.header.is_resolved = true;
            }
        }
    }

    Ok(())
}

fn bcp_long_clauses(bcp: &mut BcpContext, literal: Literal) -> Result<(), Conflict> {
    let mut result = Ok(());

    let watched_literal_1 = !literal;

    let mut watches = bcp.watch.take_watchlist(watched_literal_1);
    let mut removed_watch_indices: Vec<usize> = vec![];

    'watches: for (watch_index, watch) in watches.iter().enumerate() {
        let clause = bcp.long_clauses.find_clause_mut(watch.clause_index);
        let literals = clause.literals_mut();

        // get the other watched literal
        let watched_literal_2 = if watched_literal_1 == literals[0] {
            literals[1]
        } else {
            literals[0]
        };

        // the clause is already satisfied by the other watched literal
        if bcp.assignment.literal_value(watched_literal_2) == AssignedValue::True {
            clause.header_mut().is_resolved = true;
            continue;
        }

        // search a non-false non-watched literal to replace watched_literal_1
        for i in 2..literals.len() {
            let current_literal = literals[i];
            match bcp.assignment.literal_value(current_literal) {
                AssignedValue::True | AssignedValue::Unknown => {
                    // change the watches
                    removed_watch_indices.push(watch_index);
                    bcp.watch.add_watch(current_literal, *watch);
                    // change the clauses literal order
                    literals[0] = current_literal;
                    literals[1] = watched_literal_2;
                    literals[i] = watched_literal_1;
                    continue 'watches;
                }
                _ => {}
            }
        }

        // did not find a non-false non-watched literal
        match bcp.assignment.literal_value(watched_literal_2) {
            // clause became unit, propagate `watched_literal_2`
            AssignedValue::True | AssignedValue::Unknown => {
                literals[0] = watched_literal_2;
                literals[1] = watched_literal_1;

                let step = Step {
                    assigned_literal: watched_literal_2,
                    decision_level: bcp.trail.current_decision_level(),
                    reason: Reason::Long(watch.clause_index),
                };

                trail::assign(&mut bcp.assignment, &mut bcp.trail, step);
                clause.header_mut().is_resolved = true;
            }
            // all literals are false, conflict
            AssignedValue::False => {
                clause.header_mut().is_resolved = false;
                result = Err(Conflict::LongClause(watch.clause_index));
                break;
            }
        }
    }

    // remove the invalidated watches
    removed_watch_indices.into_iter().rev().for_each(|i| {
        watches.remove(i);
    });

    bcp.watch.place_watchlist(!literal, watches);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cnf::CNF;

    #[test]
    fn test_basic_bcp() {
        let mut bcp = BcpContext::default();
        let cnf = CNF::from_dimacs("-1 2 0\n-2 3 0\n-2 -3 -4 0\n6 7\n");
        bcp.resize(cnf.variable_count());

        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }

        bcp.init();
        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(1));

        assert!(propagate(&mut bcp).is_ok());
        let assignment = bcp
            .trail
            .steps()
            .iter()
            .map(|s| s.assigned_literal)
            .map(|l| l.as_dimacs_integer())
            .collect::<Vec<_>>();

        assert_eq!(assignment, vec![1, 2, 3, -4]);
    }

    #[test]
    fn test_basic_conflict() {
        let cnf = CNF::from_dimacs("-1 2 0\n-1 3 0\n-2 -3 0\n");

        let mut bcp = BcpContext::default();
        bcp.resize(cnf.variable_count());
        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }
        bcp.init();

        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(1));

        match propagate(&mut bcp) {
            Err(Conflict::BinaryClause(literals)) => {
                assert_eq!(literals, cnf.clauses()[2].literals());
            }
            _ => panic!("expected a conflict"),
        };
    }

    #[test]
    fn test_exercise_5_conflict() {
        let cnf = CNF::from_dimacs("-1 2 0\n-1 3 9 0\n-2 -3 4 0\n-4 5 10 0\n-4 6 11 0\n-5 -6 0\n1 7 -12 0\n1 8 0\n-7 -8 -13 0\n");

        let mut bcp = BcpContext::default();
        bcp.resize(cnf.variable_count());
        for c in cnf.clauses().iter() {
            bcp.add_clause(c.literals());
        }
        bcp.init();

        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(-9));
        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(-10));
        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(-11));
        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(12));
        trail::decide_and_assign(&mut bcp, Literal::from_dimacs(1));

        match propagate(&mut bcp) {
            Err(Conflict::BinaryClause(literals)) => {
                // expect conflict with clause c6: [-5, -6]
                assert_eq!(literals, cnf.clauses()[5].literals());
            }
            _ => panic!("expected a conflict from binary clause"),
        };
    }

    #[test]
    fn test_exercise_6_failed_literals() {
        for test_lit in [-1, 3, 4, 1, -2] {
            let cnf = CNF::from_dimacs("-1 3 2 0\n-1 3 -2 0\n4 1 0\n-4 1 0\n");

            let mut bcp = BcpContext::default();
            bcp.resize(cnf.variable_count());
            for c in cnf.clauses().iter() {
                bcp.add_clause(c.literals());
            }
            bcp.init();

            trail::decide_and_assign(&mut bcp, Literal::from_dimacs(test_lit));

            match propagate(&mut bcp) {
                Ok(_) => {
                    println!("OK");
                }
                Err(conflict) => {
                    dbg!(conflict);
                }
            }
        }
    }
}
