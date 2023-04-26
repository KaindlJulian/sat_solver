use sat_lib::parse::parse_dimacs_cnf;

fn main() {
    let file = include_str!("../../test-formulas/add2.in");
    let cnf = parse_dimacs_cnf(file).unwrap().1;

    print!("{:?}", cnf);
}
