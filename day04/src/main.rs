use itertools::Itertools;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    // start parsing
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let elapsed_parsing = now.elapsed();

    // done with parsing, can start solving part 1
    let part1 = part1(&grid);
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    // done with part 1, can start solving part 2
    let part2 = part2(&grid);
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn part1(grid: &[Vec<char>]) -> usize {
    let word: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;

    for y in 0..grid.len() - 1 {
        for x in 0..grid[y].len() - 1 {
            search(grid, &word, 0, y, x, None, &mut count);
        }
    }

    count
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 'A'
                || y == 0
                || x == 0
                || x == grid[y].len() - 1
                || y == grid.len() - 1
            {
                continue;
            }

            let diag_a = [grid[y - 1][x - 1], grid[y][x], grid[y + 1][x + 1]];
            let diag_b = [grid[y + 1][x - 1], grid[y][x], grid[y - 1][x + 1]];

            if (diag_a.contains(&'S') && diag_a.contains(&'M'))
                && (diag_b.contains(&'S') && diag_b.contains(&'M'))
            {
                count += 1;
            };
        }
    }

    count
}

fn search(
    grid: &[Vec<char>],
    word: &[char],
    word_index: usize,
    y: usize,
    x: usize,
    offset: Option<(i32, i32)>,
    count: &mut usize,
) -> bool {
    if word[word_index] != grid[y][x] {
        return false;
    }

    if word_index == word.len() - 1 && word[word_index] == grid[y][x] {
        return true;
    }

    let offsets = match offset {
        Some(offset) => vec![offset],
        None => (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&offset| offset != (0, 0))
            .collect(),
    };

    for (x_offset, y_offset) in offsets {
        let new_y: i32 = y as i32 + y_offset;
        let new_x: i32 = x as i32 + x_offset;

        if new_x < 0 || new_y < 0 || new_y == grid.len() as i32 || new_x == grid[0].len() as i32 {
            continue;
        }

        if search(
            grid,
            word,
            word_index + 1,
            new_y as usize,
            new_x as usize,
            Some((x_offset, y_offset)),
            count,
        ) {
            *count += 1;
        }
    }

    false
}
