use sat_lib::cnf::CNF;

fn main() {
    let file = include_str!("../../test-formulas/eq1.in");
    let cnf = CNF::from_clauses(vec![vec![-1, 2, 3], vec![1]]);

    dbg!(cnf);
}
