use crate::bcp::long_clauses::LongClauses;
use crate::clause::ClauseIndex;
use crate::literal::Literal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct LiteralWatch {
    /// index of the clause being watched
    pub clause_index: ClauseIndex,
}

/// For every literal, keeps a list of clauses watched by this literal
/// The watched literals will always be the first two, at index 0 and 1
#[derive(Default, Debug)]
pub struct Watchlists {
    watches: HashMap<Literal, Vec<LiteralWatch>>,
}

impl Watchlists {
    /// create new watches for a clause
    pub fn watch_clause(&mut self, clause_index: ClauseIndex, lits: [Literal; 2]) {
        for i in 0..2 {
            let watch = LiteralWatch { clause_index };

            match self.watches.entry(lits[i]) {
                Entry::Occupied(mut e) => e.get_mut().push(watch),
                Entry::Vacant(e) => {
                    e.insert(vec![watch]);
                }
            }
        }
    }

    pub fn add_watch(&mut self, lit: Literal, watch: LiteralWatch) {
        match self.watches.entry(lit) {
            Entry::Occupied(mut e) => e.get_mut().push(watch),
            Entry::Vacant(e) => {
                e.insert(vec![watch]);
            }
        }
    }

    /// Take ownership of a literals watchlist
    pub fn take_watchlist(&mut self, lit: Literal) -> Vec<LiteralWatch> {
        self.watches.remove(&lit).unwrap_or(vec![])
    }

    pub fn place_watchlist(&mut self, lit: Literal, watchlist: Vec<LiteralWatch>) {
        self.watches.insert(lit, watchlist);
    }

    pub fn clear(&mut self) {
        self.watches.clear();
    }

    pub fn build_watchlists(&mut self, long_clauses: &LongClauses) {
        self.clear();
        for (index, clause) in long_clauses.clauses().iter().enumerate() {
            let literals = clause.literals();
            self.watch_clause(index, [literals[0], literals[1]]);
        }
    }
}
