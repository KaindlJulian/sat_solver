use crate::clause::ClauseIndex;
use crate::literal::Literal;
use crate::resize::Resize;

#[derive(Debug, Copy, Clone)]
pub struct LiteralWatch {
    pub clause_index: ClauseIndex,
    pub satisfying_literal: Literal,
}

/// For every literal, keeps a list of clauses watched by this literal
/// The watched literals will always be the first two, at index 0 and 1
#[derive(Default, Debug)]
pub struct Watchlists {
    watches_by_lit: Vec<Vec<LiteralWatch>>,
}

impl Resize for Watchlists {
    fn resize(&mut self, var_count: usize) {
        self.watches_by_lit.resize(var_count * 2, vec![]);
    }
}

impl Watchlists {
    /// create new watches for a clause and add them
    pub fn watch_clause(&mut self, clause_index: ClauseIndex, literals: [Literal; 2]) {
        for i in 0..2 {
            let watched_literal = literals[i];
            let satisfying_literal = literals[i ^ 1];
            let watch = LiteralWatch {
                clause_index,
                satisfying_literal,
            };
            self.add_watch(watched_literal, watch);
        }
    }

    pub fn add_watch(&mut self, lit: Literal, watch: LiteralWatch) {
        self.watches_by_lit[lit.as_index()].push(watch);
    }

    /// Take ownership of a literals watchlist
    pub fn take_watchlist(&mut self, lit: Literal) -> Vec<LiteralWatch> {
        std::mem::take(&mut self.watches_by_lit[lit.as_index()])
    }

    /// Return ownership of a literals watchlist
    pub fn place_watchlist(&mut self, lit: Literal, watchlist: Vec<LiteralWatch>) {
        self.watches_by_lit[lit.as_index()] = watchlist;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watchlists_from_long_clause() {
        let mut watches = Watchlists::default();
        watches.resize(2);
        watches.watch_clause(0, [Literal::from_dimacs(1), Literal::from_dimacs(2)]);
        let list = watches.take_watchlist(Literal::from_dimacs(1));
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].clause_index, 0);
    }
}
