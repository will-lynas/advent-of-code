# advent-of-code

My (work in progress) solutions to [Advent of Code](https://adventofcode.com/) written in Rust

<img src="https://github.com/user-attachments/assets/c5209052-c2b5-40aa-a60b-1b427ef35178" width=400>

## Running the code

### Running against the examples

These are the small examples given in the text of the problem

Everything:

```
cargo test
```

Specific year:

```
cargo test year2024
```

Specific day:

```
cargo test year2024::day04
```

### Running against puzzle inputs

First, place the relevant puzzle inputs in the path `input/yearXXXX/dayYY.txt`

To run everything:

```
cargo run
```

Specific year:

```
cargo run year2024
```

Specific day:

```
cargo run year2024 day04
```

The output should look something like this:

```
year2024 day04
    Part 1: 2517
    Part 2: 1960
🕓 7 ms
```

## Acknowledgments

Thank you to ...

- [maneatingape/advent-of-code-rust](https://github.com/maneatingape/advent-of-code-rust) for the structure of this repo, and for large parts of the solutions
- [Neil Thistlethwaite](https://www.youtube.com/@nthistlethwaite) for his amazing live Advent of Code solutions on YouTube
