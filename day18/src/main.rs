use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
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
    let part1 = bfs(&obstacles, 70, 1024);
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut left = 0;
    let mut right = obstacles.len() - 1;
    let mut middle = (left + right) / 2;

    while left < right {
        //binary search (be wary of 1-off errors)
        middle = (left + right) / 2;
        if bfs(&obstacles, 70, middle + 1) != -1 {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    let part2 = obstacles[middle];
    println!("{:?}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn in_grid(x: i32, y: i32, n: usize) -> bool {
    x >= 0 && x < n as i32 && y >= 0 && y < n as i32
}

fn bfs(obstacles: &[(usize, usize)], end: usize, n_obstacles: usize) -> i32 {
    let mut grid = vec![vec!['.'; end + 1]; end + 1];

    for (x, y) in obstacles.iter().take(n_obstacles) {
        grid[*x][*y] = '#';
    }

    let n = grid.len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new(); //queue for BFS
    queue.push_back((0, 0, 0)); //(row, col, steps)

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == (end, end) {
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
