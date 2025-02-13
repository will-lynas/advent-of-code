# Adding a New Day

To add a new day's solution, follow these steps:

## 1. Input File

- Create the input file at `input/yearXXXX/dayYY.txt`
- Replace XXXX with the year (e.g., 2024) and YY with the day number (e.g., 01)

## 2. Solution Code

- Create a new file at `src/yearXXXX/dayYY.rs`
- You can copy the template from `src/template_year/template_day.rs`

## 3. Test File

- Create a test file at `tests/yearXXXX/dayYY.rs`
- You can copy the template from `tests/template_year/template_day.rs`

## 4. Update Macros

1. In `src/lib.rs`, add the day to the `lib!` macro:
2. In `tests/tests.rs`, add the day to the `test!` macro:
3. In `benches/benchmarks.rs`, add the day to the `benchmark!` macro:
4. In `src/main.rs`, add the day to the `run!` macro:
