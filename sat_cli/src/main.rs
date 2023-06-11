use clap::Parser;
use sat_lib::cnf::CNF;
use sat_lib::solver::Solver;
use std::path::PathBuf;

#[derive(Parser)]
#[group(required = true)]
struct Args {
    /// A dimacs cnf file
    file: PathBuf,

    /// Disable DLIS decision heuristic
    #[arg(long)]
    no_dlis: bool,
}

fn main() {
    let args = Args::parse();
    let cnf = CNF::from_file(args.file);
    let mut solver = Solver::from_cnf(cnf);

    if args.no_dlis {
        solver = solver.without_dlis();
    }

    if solver.solve() {
        println!("SATISFIABLE");
        println!("assignment: {:?}", solver.assignment());
        std::process::exit(10);
    } else {
        println!("UNSATISFIABLE");
        std::process::exit(20);
    }
}
