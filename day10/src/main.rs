use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let grid: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let dd: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut part1 = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            part1 += score(i as i32, j as i32, &grid, &dd);
        }
    }
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();

    let mut part2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 0 {
                part2 += rating(i as i32, j as i32, &grid, &dd);
            }
        }
    }
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn in_grid(i: i32, j: i32, n: i32) -> bool {
    i >= 0 && i < n && j >= 0 && j < n
}

fn score(i: i32, j: i32, grid: &[Vec<u32>], dd: &[(i32, i32); 4]) -> i32 {
    if grid[i as usize][j as usize] != 0 {
        return 0;
    }

    let mut ans = 0;
    let mut stack = vec![(i, j)];
    let mut visited = HashSet::new();
    let n = grid.len() as i32;

    while let Some((cur_i, cur_j)) = stack.pop() {
        // Skip if we've already visited this cell in this path
        if !visited.insert((cur_i, cur_j)) {
            continue;
        }

        let cur = grid[cur_i as usize][cur_j as usize];

        if cur == 9 {
            ans += 1;
            continue;
        }

        for (di, dj) in dd.iter() {
            let ii = cur_i + di;
            let jj = cur_j + dj;

            if !in_grid(ii, jj, n) {
                continue;
            }

            let nbr = grid[ii as usize][jj as usize];
            if nbr != cur + 1 {
                continue;
            }

            // Only add if we haven't visited
            if !visited.contains(&(ii, jj)) {
                stack.push((ii, jj));
            }
        }
    }
    ans
}

fn rating(i: i32, j: i32, grid: &[Vec<u32>], dd: &[(i32, i32); 4]) -> i32 {
    if grid[i as usize][j as usize] == 9 {
        return 1;
    }

    let n = grid.len() as i32;
    let mut ans = 0;
    for (di, dj) in dd.iter() {
        let ii = i + di;
        let jj = j + dj;

        if !in_grid(ii, jj, n) {
            continue;
        }

        if grid[ii as usize][jj as usize] == grid[i as usize][j as usize] + 1 {
            ans += rating(ii, jj, grid, dd);
        }
    }

    ans
}
