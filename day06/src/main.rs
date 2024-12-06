use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();

    let mut og_seen = HashSet::new();
    let (mut row, mut col) = find_guard(&grid);
    let (start_row, start_col) = (row, col);
    let mut direction = (-1, 0); //start going

    while let Some((next_row, next_col)) =
        next_position(row, col, direction, grid.len(), grid[0].len())
    {
        og_seen.insert((row, col));
        if grid[next_row][next_col] == '#' {
            direction = turn_right(direction);
        } else {
            row = next_row;
            col = next_col;
        }
    }
    og_seen.insert((row, col));

    let part1 = og_seen.len();

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut grid = grid.clone();
    let mut part2 = 0;

    for &(og_row, og_col) in &og_seen {
        if will_loop(&mut grid, start_row, start_col, og_row, og_col) {
            part2 += 1;
        }
    }

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn find_guard(grid: &[Vec<char>]) -> (usize, usize) {
    for (r, line) in grid.iter().enumerate() {
        if let Some(c) = line.iter().position(|&ch| ch == '^') {
            return (r, c);
        }
    }
    (0, 0)
}

fn next_position(
    row: usize,
    col: usize,
    direction: (i32, i32),
    max_rows: usize,
    max_cols: usize,
) -> Option<(usize, usize)> {
    let new_row = row as i32 + direction.0;
    let new_col = col as i32 + direction.1;
    if new_row >= 0 && new_row < max_rows as i32 && new_col >= 0 && new_col < max_cols as i32 {
        Some((new_row as usize, new_col as usize))
    } else {
        None
    }
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (-1, 0) => (0, 1),  // up -> right
        (0, 1) => (1, 0),   // right -> down
        (1, 0) => (0, -1),  // donw -> left
        (0, -1) => (-1, 0), // left -> up
        _ => (0, 0),
    }
}

fn will_loop(
    grid: &mut [Vec<char>],
    start_row: usize,
    start_col: usize,
    og_row: usize,
    og_col: usize,
) -> bool {
    if grid[og_row][og_col] == '#' {
        return false;
    }

    grid[og_row][og_col] = '#';

    let mut row = start_row;
    let mut col = start_col;
    let mut direction = (-1, 0);
    let mut seen = std::collections::HashSet::new();

    while let Some((next_row, next_col)) =
        next_position(row, col, direction, grid.len(), grid[0].len())
    {
        if !seen.insert((row, col, direction)) {
            grid[og_row][og_col] = '.';
            return true;
        }

        if grid[next_row][next_col] == '#' {
            direction = turn_right(direction);
        } else {
            row = next_row;
            col = next_col;
        }
    }

    grid[og_row][og_col] = '.';
    false
}
