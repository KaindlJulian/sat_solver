use clap::Parser;
use sat_lib::cnf::CNF;
use sat_lib::solver::Solver;
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
        let cnf = CNF::from_file(path);
        let mut solver = Solver::from_cnf(cnf);
        println!("{}", solver.solve());
    }
}
