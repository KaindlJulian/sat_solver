use crate::literal::Literal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct LiteralWatch {
    /// index of the clause being watched in `context.clauses()`
    pub clause_index: usize,
    /// index of the watched literal in `clause.literals()`
    pub literal_index: usize,
}

/// For every literal, keeps a list of clauses watched by this literal
#[derive(Default)]
pub struct Watchlists {
    watches: HashMap<Literal, Vec<LiteralWatch>>,
}

impl Watchlists {
    /// watch a clause with 2 instances of [`LiteralWatch`]
    pub fn watch_clause(
        &mut self,
        clause_index: usize,
        lit_indexes: [usize; 2],
        lits: [Literal; 2],
    ) {
        for i in 0..2 {
            let watch = LiteralWatch {
                clause_index,
                literal_index: lit_indexes[i],
            };

            match self.watches.entry(lits[i]) {
                Entry::Occupied(mut e) => e.get_mut().push(watch),
                Entry::Vacant(e) => {
                    e.insert(vec![watch]);
                }
            }
        }
    }

    /// Find watches for given literal
    pub fn get_watchlist(&self, lit: &Literal) -> &Vec<LiteralWatch> {
        &self.watches.get(lit).unwrap()
    }
}
