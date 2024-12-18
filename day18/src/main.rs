use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    let obstacles: Vec<(usize, usize)> = contents
        .lines()
        .filter_map(|line| {
            line.split_once(',').and_then(|(x, y)| {
                Some((
                    x.trim().parse::<usize>().ok()?,
                    y.trim().parse::<usize>().ok()?,
                ))
            })
        })
        .collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    for (x, y) in obstacles.iter().take(1024) {
        grid[*x][*y] = '#';
    }

    let part1 = bfs(&grid, (0, 0), (70, 70));
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut i = 1024;
    let mut part2 = (0, 0);
    while bfs(&grid, (0, 0), (70, 70)) != -1 {
        let (x, y) = (obstacles[i].0, obstacles[i].1);
        grid[x][y] = '#';
        i += 1;
        part2 = (x, y);
    }
    println!("{:?}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn in_grid(x: i32, y: i32, n: usize) -> bool {
    x >= 0 && x < n as i32 && y >= 0 && y < n as i32
}

fn bfs(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> i32 {
    let n = grid.len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new(); //queue for BFS
    queue.push_back((start.0, start.1, 0)); //(row, col, steps)

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == end {
            return steps;
        }

        for (dx, dy) in directions.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if in_grid(new_x, new_y, n) {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if grid[new_x][new_y] == '.' && !visited.contains(&(new_x, new_y)) {
                    visited.insert((new_x, new_y));
                    queue.push_back((new_x, new_y, steps + 1));
                }
            }
        }
    }

    -1
}
