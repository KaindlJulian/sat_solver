use crate::assignment::{AssignedValue, VariableAssignment};
use crate::clause::ClauseMeta;
use crate::literal::Literal;
use crate::resize::Resize;

#[derive(Debug, Copy, Clone)]
pub struct BinaryClause {
    pub other_literal: Literal,
    pub header: ClauseMeta,
}

#[derive(Default, Debug)]
pub struct BinaryClauses {
    /// maps a literal to the other literals it forms a binary clause with for fast lookup
    /// indexed by the literal code
    literal_lookup: Vec<Vec<BinaryClause>>,
}

impl Resize for BinaryClauses {
    fn resize(&mut self, variable_count: usize) {
        self.literal_lookup.resize(variable_count * 2, vec![]);
    }
}

impl BinaryClauses {
    pub fn add_clause(&mut self, clause: [Literal; 2]) {
        let max = clause[0].as_index().max(clause[1].as_index());
        if self.literal_lookup.len() <= max {
            //self.resize(max + 2);
        }

        for i in 0..2 {
            self.literal_lookup[clause[i].as_index()].push(BinaryClause {
                other_literal: clause[i ^ 1],
                header: Default::default(),
            });
        }
    }

    /// Returns all binary clauses that contain the given literal
    pub fn clauses_mut(&mut self, literal: Literal) -> &mut Vec<BinaryClause> {
        &mut self.literal_lookup[literal.as_index()]
    }

    pub fn unresolved(&self, literal: Literal, assignment: &VariableAssignment) -> usize {
        self.literal_lookup[literal.as_index()]
            .iter()
            .filter(|c| {
                assignment.literal_value(literal) == AssignedValue::True
                    || assignment.literal_value(c.other_literal) == AssignedValue::True
            })
            .count()
    }
}
