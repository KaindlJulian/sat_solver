use crate::literal::Literal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct BinaryClauses {
    /// maps a literal to the other literals it forms a binary clause with
    literal_lookup: HashMap<Literal, Vec<Literal>>,
}

impl BinaryClauses {
    pub fn add_clause(&mut self, clause: [Literal; 2]) {
        for i in 0..2 {
            match self.literal_lookup.entry(clause[i]) {
                Entry::Occupied(mut e) => e.get_mut().push(clause[i ^ 1]),
                Entry::Vacant(e) => {
                    e.insert(vec![clause[i ^ 1]]);
                }
            }
        }
    }

    /// Returns all clauses that contain the given literal
    pub fn get_clauses(&self, literal: Literal) -> &[Literal] {
        &self
            .literal_lookup
            .get(&literal)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}
