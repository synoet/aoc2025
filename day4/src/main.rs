static TEST_INPUT: &str = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.#";

type Grid = Vec<Vec<bool>>;
type Pos<T> = (T, T);
type Check = (i32, i32);

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| x.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect::<Grid>()
}

const MAX: u16 = 4;

fn maybe_translate(grid: &Grid, pos: &Pos<i32>, check: &Check) -> Option<Pos<usize>> {
    let max_row = grid.len() as i32;
    let max_col = grid[0].len() as i32;

    let new_pos = (pos.0 + check.0, pos.1 + check.1);
    let row_valid = new_pos.0 >= 0 && new_pos.0 < max_row;
    let col_valid = new_pos.1 >= 0 && new_pos.1 < max_col;

    (row_valid && col_valid).then_some((new_pos.0 as usize, new_pos.1 as usize))
}

const POS_CHECKS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn check(grid: &Grid, pos: &Pos<usize>) -> bool {
    POS_CHECKS
        .iter()
        .filter_map(|check| maybe_translate(&grid, &(pos.0 as i32, pos.1 as i32), check))
        .filter_map(|pos| grid[pos.0][pos.1].then_some(pos))
        .collect::<Vec<_>>()
        .len()
        < MAX.into()
}

fn part_two() -> i32 {
    let mut grid = parse_input(include_str!("./input.txt"));

    let mut next_grid = grid.clone();
    let mut total = 0;

    loop {
        let mut count = 0;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] && check(&grid, &(row, col)) {
                    count += 1;
                    next_grid[row][col] = false;
                }
            }
        }

        if count == 0 {
            break;
        } else {
            total += count;
            grid = next_grid.clone();
        }
    }

    total
}

fn part_one() -> i32 {
    let grid = parse_input(include_str!("./input.txt"));
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] && check(&grid, &(row, col)) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    // part_one();
    part_two();
}
