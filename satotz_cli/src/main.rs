use clap::Parser;
use satotz_lib::cnf::CNF;
use satotz_lib::solver::Solver;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// A dimacs cnf file
    #[clap(required = true)]
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
        println!("s SATISFIABLE");
        println!("v {:?}", solver.assignment());
        std::process::exit(10);
    } else {
        println!("s UNSATISFIABLE");
        std::process::exit(20);
    }
}
