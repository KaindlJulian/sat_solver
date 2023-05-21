import os
from pysat.solvers import Solver
from pysat.formula import CNF

excluded = [".\\test_formulas\\mcs3.in", ".\\test_formulas\\mus0.in"]


def evaluate_cnf_file(file_path):
    if file_path in excluded:
        return

    cnf = CNF(from_file=file_path)

    with Solver(bootstrap_with=cnf) as solver:
        if solver.solve():
            os.rename(file_path, file_path.replace("in", "sat"))
            print(f"{file_path}: SAT")
        else:
            print(f"{file_path}: UNSAT")
            os.rename(file_path, file_path.replace("in", "unsat"))

    solver.delete()


def evaluate_cnf_files(directory):
    for root, dirs, files in os.walk(directory):
        for file_name in files:
            file_path = os.path.join(root, file_name)
            print(file_path)
            evaluate_cnf_file(file_path)


# Example usage
evaluate_cnf_files(".\\test_formulas")
