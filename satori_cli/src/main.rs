use clap::Parser;
use satori_lib::cnf::CNF;
use satori_lib::solver::Solver;
use std::path::PathBuf;

#[derive(Parser)]
#[group(required = true)]
struct Args {
    /// dimacs cnf file(s)
    file: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    for path in args.file {
        print!("{:?}: ", path.file_name().unwrap());
        let dimacs = std::fs::read_to_string(path).expect("error reading file");
        let cnf = CNF::from_dimacs(dimacs.as_str());
        let solver = Solver::from_cnf(cnf);
        println!("{}", solver.solve());
    }
}
