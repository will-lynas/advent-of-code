type Grid = Vec<Vec<char>>;

pub fn parse(input: &str) -> Grid {
    input.lines().map(|row| row.chars().collect()).collect()
}

pub fn part1(grid: &Grid) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let target = "XMAS";
    let target_len = target.len();

    let directions = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != target.chars().next().unwrap() {
                continue;
            }

            for &(dx, dy) in &directions {
                let mut matched = true;

                for i in 1..target_len {
                    let new_row = row as isize + dx * i as isize;
                    let new_col = col as isize + dy * i as isize;

                    if new_row < 0
                        || new_row >= rows as isize
                        || new_col < 0
                        || new_col >= cols as isize
                    {
                        matched = false;
                        break;
                    }

                    if grid[new_row as usize][new_col as usize] != target.chars().nth(i).unwrap() {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part2(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] != 'A' {
                continue;
            }

            if ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
                || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
                && ((grid[row + 1][col - 1] == 'M' && grid[row - 1][col + 1] == 'S')
                    || (grid[row + 1][col - 1] == 'S' && grid[row - 1][col + 1] == 'M'))
            {
                count += 1
            }
        }
    }
    count
}
