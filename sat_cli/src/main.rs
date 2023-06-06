use clap::Parser;
use sat_lib::cnf::CNF;
use sat_lib::solver::Solver;
use std::path::PathBuf;

#[derive(Parser)]
#[group(required = true)]
struct Args {
    /// dimacs cnf file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let cnf = CNF::from_file(args.file);
    let mut solver = Solver::from_cnf(cnf).with_dlis();

    if solver.solve() {
        println!("SATIFIABLE");
        std::process::exit(10);
    } else {
        println!("UNSATIFIABLE");
        std::process::exit(20);
    }
}
