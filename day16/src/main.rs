use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
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
    let (sx, sy) = find_starting_position(&grid);
    let dir = (0, 1); //starting direction = right;

    let part1 = dijkstra(&grid, (sx, sy), dir);
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();

    //println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn find_starting_position(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn in_grid((x, y): (i32, i32), r: usize, c: usize) -> bool {
    x >= 0 && x < r as i32 && y >= 0 && y < c as i32
}

fn rotate_90_degrees(dir: (i32, i32)) -> Vec<(i32, i32)> {
    match dir {
        (-1, 0) | (1, 0) => vec![(0, -1), (0, 1)], //up/down -> left, right
        (0, -1) | (0, 1) => vec![(-1, 0), (1, 0)], //left/right -> up, down
        _ => vec![],
    }
}

fn dijkstra(grid: &[Vec<char>], start: (usize, usize), start_dir: (i32, i32)) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut pq = BinaryHeap::new(); //priority queue: (Reverse(score), x, y, direction)
    pq.push((Reverse(0), start.0, start.1, start_dir));

    let mut visited = HashSet::new();

    while let Some((Reverse(score), x, y, dir)) = pq.pop() {
        if grid[x][y] == 'E' {
            return score; //found the end
        }

        //avoid revisiting the same position with the same direction
        if !visited.insert((x, y, dir)) {
            continue;
        }

        //move forward
        let new_x = x as i32 + dir.0;
        let new_y = y as i32 + dir.1;

        if in_grid((new_x, new_y), rows, cols) && grid[new_x as usize][new_y as usize] != '#' {
            pq.push((Reverse(score + 1), new_x as usize, new_y as usize, dir));
        }

        //rotate 90 degrees
        for new_dir in rotate_90_degrees(dir) {
            pq.push((Reverse(score + 1000), x, y, new_dir));
        }
    }

    i32::MAX
}
