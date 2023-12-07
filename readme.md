# Advent of Code 2023: Carcinized

Solving the Advent of Code puzzles using Rust as a newbie (ideomatic code is
ostensibly a mistake).

This project uses cargo workspaces to make it easy to run and test the
solutions for each day. There is a root cargo config that references the
individual workspaces, and each of them--in turn--have their own cargo config.

# Running

To run tests for a certain day (e.g. `one`):

```
cargo test -p one
```

To run all the tests, just leave off the `-p` parameter.

To run the solution for a certain day (e.g. `one`):

```
cargo run -p one
```

NB: When running tests (`cargo test`), the cwd is the workspace dir, but when
running the solution it is the dir where `cargo run` is executed (usually the
root); this mostly impacts relative pathnames for loading input files. Maybe
there's a way to solve this, if so it is currently beyond my rust or cargo
knowledge.

# Adding a Day

To add a new a new solution, add its folder name to the `members` list in the
root `Cargo.toml`, then run `cargo new foldername` from the root to create the
workspace dir and config.
