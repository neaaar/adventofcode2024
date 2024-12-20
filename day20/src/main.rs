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
    let (r, c) = find_start(&grid);
    let part1 = find_cheats(&grid, (r, c));
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let part2 = find_cheats_new(&grid, (r, c));
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn in_grid(x: i32, y: i32, n: usize) -> bool {
    x >= 0 && x < n as i32 && y >= 0 && y < n as i32
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    let (mut r, mut c) = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                (r, c) = (i, j);
            }
        }
    }

    (r, c)
}

fn find_cheats(grid: &[Vec<char>], (r, c): (usize, usize)) -> i32 {
    let (mut r, mut c) = (r, c);
    let mut cheats = 0;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let cheat_directions = [(2, 0), (1, 1), (0, 2), (-1, 1)];
    let mut dists = vec![vec![-1; grid.len()]; grid[0].len()];
    dists[r][c] = 0;

    //no need for bfs since there's only one valid path without cheats
    while grid[r][c] != 'E' {
        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if !in_grid(nr, nc, grid.len()) {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            if dists[nr][nc] != -1 || grid[nr][nc] == '#' {
                continue;
            }

            dists[nr][nc] = dists[r][c] + 1;
            r = nr;
            c = nc;
        }
    }

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' {
                continue;
            }

            for (dr, dc) in cheat_directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if !in_grid(nr, nc, grid.len()) {
                    continue;
                }

                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == '#' {
                    continue;
                }

                let diff: i32 = dists[r][c] - dists[nr][nc];
                if (diff).abs() >= 102 {
                    cheats += 1;
                }
            }
        }
    }

    cheats
}

fn find_cheats_new(grid: &[Vec<char>], (r, c): (usize, usize)) -> usize {
    let (mut r, mut c) = (r, c);
    let mut cheats: usize = 0;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dists = vec![vec![-1; grid.len()]; grid[0].len()];
    dists[r][c] = 0;

    //no need for bfs since there's only one valid path without cheats
    while grid[r][c] != 'E' {
        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if !in_grid(nr, nc, grid.len()) {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            if dists[nr][nc] != -1 || grid[nr][nc] == '#' {
                continue;
            }

            dists[nr][nc] = dists[r][c] + 1;
            r = nr;
            c = nc;
        }
    }

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' {
                continue;
            }

            for radius in 2..=20 {
                for dr in 0..=radius {
                    let dc: i32 = radius - dr;

                    for (nr, nc) in [
                        (r as i32 + dr, c as i32 + dr),
                        (r as i32 + dr, c as i32 - dc),
                        (r as i32 - dr, c as i32 + dc),
                        (r as i32 - dr, c as i32 - dc),
                    ] {
                        if !in_grid(nr, nc, grid.len()) {
                            continue;
                        }

                        let (nr, nc) = (nr as usize, nc as usize);
                        if grid[nr][nc] == '#' {
                            continue;
                        }

                        let diff: i32 = dists[r][c] - dists[nr][nc];
                        if diff >= 100 + radius {
                            cheats += 1;
                        }
                    }
                }
            }
        }
    }

    cheats
}
