# advent-of-code

My solutions to [Advent of Code](https://adventofcode.com/) written in Rust

<img src="https://github.com/user-attachments/assets/c5209052-c2b5-40aa-a60b-1b427ef35178" width=400>

## Running the code

### Inputs

Inputs should be placed in the path `input/yearXXXX/dayYY.txt`

### Solutions

- Run everything: `cargo run`
- Specific year: `cargo run year2024`
- Specific day: `cargo run year2024 day04`
- To run with the `release` profile (much faster): `cargo run --release` or `cargo run --release -- year2024 day04`

### Examples

The examples given in the text of each problem are used in tests.

- Run everything: `cargo test`
- Specific year: `cargo test year2024`
- Specific day: `cargo test year2024::day04`
- Specific part: `cargo test year2024::day04::part2`

### Benchmarks

- Everything: `cargo bench`
- Specific year: `cargo bench year2024`
- Specific day: `cargo bench year2024::day04`
- Specific part: `cargo bench year2024::day04::part2`

## Acknowledgments

Thank you to ...

- [maneatingape/advent-of-code-rust](https://github.com/maneatingape/advent-of-code-rust) for the structure of this repo, and for large parts of the solutions
- [Neil Thistlethwaite](https://www.youtube.com/@nthistlethwaite) for his amazing live Advent of Code solutions on YouTube
