![](https://github.com/kaindljulian/sat_solver/actions/workflows/build_and_test.yml/badge.svg)

__Compile and Run:__

````
$ cargo build --release
$ ./target/release/sat_cli ./test_formulas/sat1.sat
s SATISFIABLE
v [1, 2]
````
__Help:__
```
Usage: sat_cli <FILE>

Arguments:
  <FILE>  A dimacs cnf file

Options:
      --no-dlis  Disable DLIS decision heuristic
  -h, --help     Print help
```
