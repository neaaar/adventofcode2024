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

    let visited: &mut HashSet<(usize, usize)> = &mut HashSet::with_capacity(20_000);

    let mut part1 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let point = (x, y);
            let (current_perimeter, current_area) = count_slot(point, visited, &grid);
            part1 += current_perimeter * current_area;
        }
    }

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();

    let visited: &mut HashSet<(usize, usize)> = &mut HashSet::with_capacity(20_000);

    let mut part2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let point = (x, y);
            let (current_perimeter, current_area) = count_slot_2(point, visited, &grid);
            part2 += current_perimeter * current_area;
        }
    }
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn count_slot(
    coordinates: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    grid: &[Vec<char>],
) -> (u64, u64) {
    if visited.contains(&coordinates) {
        return (0, 0);
    }

    visited.insert(coordinates);
    let mut current_perimeter = 4;
    let mut current_area = 1;
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    for (dx, dy) in directions {
        let next_x = coordinates.0 as i32 + dx;
        let next_y = coordinates.1 as i32 + dy;

        if next_x >= 0
            && next_y >= 0
            && (next_x as usize) < grid.len()
            && (next_y as usize) < grid[0].len()
        {
            let next_x = next_x as usize;
            let next_y = next_y as usize;

            if grid[next_x][next_y] == grid[coordinates.0][coordinates.1] {
                current_perimeter -= 1;
                let (others_perimeter, others_area) = count_slot((next_x, next_y), visited, grid);
                current_perimeter += others_perimeter;
                current_area += others_area;
            }
        }
    }

    (current_perimeter, current_area)
}

fn count_slot_2(
    coordinates: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    grid: &[Vec<char>],
) -> (u64, u64) {
    if visited.contains(&coordinates) {
        return (0, 0);
    }

    visited.insert(coordinates);
    let mut current_edges = 0;
    let mut edges: HashSet<(i32, i32)> = HashSet::from_iter([(1, 1), (1, -1), (-1, 1), (-1, -1)]);
    let mut current_area = 1;

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for (dx, dy) in directions {
        let next_x = coordinates.0 as i32 + dx;
        let next_y = coordinates.1 as i32 + dy;

        if next_x >= 0
            && next_y >= 0
            && (next_x as usize) < grid.len()
            && (next_y as usize) < grid[0].len()
        {
            let next = (next_x as usize, next_y as usize);

            if grid[next.0][next.1] == grid[coordinates.0][coordinates.1] {
                let (others_edges, others_area) = count_slot_2(next, visited, grid);
                current_edges += others_edges;
                current_area += others_area;

                match (dx, dy) {
                    (0, -1) => {
                        // UP
                        edges.remove(&(1, -1)); // remove UP_RIGHT
                        edges.remove(&(-1, -1)); // remove UP_LEFT
                    }
                    (0, 1) => {
                        // DOWN
                        edges.remove(&(1, 1)); // remove DOWN_RIGHT
                        edges.remove(&(-1, 1)); // remove DOWN_LEFT
                    }
                    (1, 0) => {
                        // RIGHT
                        edges.remove(&(1, -1)); // remove UP_RIGHT
                        edges.remove(&(1, 1)); // remove DOWN_RIGHT
                    }
                    (-1, 0) => {
                        // LEFT
                        edges.remove(&(-1, -1)); // remove UP_LEFT
                        edges.remove(&(-1, 1)); // remove DOWN_LEFT
                    }
                    _ => {}
                }
            }
        }
    }

    // Add edges for concave corners
    let diagonal = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (dx, dy) in diagonal {
        let next_x = coordinates.0 as i32 + dx;
        let next_y = coordinates.1 as i32 + dy;

        if next_x >= 0
            && next_y >= 0
            && (next_x as usize) < grid.len()
            && (next_y as usize) < grid[0].len()
        {
            let next = (next_x as usize, next_y as usize);

            if grid[next.0][next.1] != grid[coordinates.0][coordinates.1] {
                let mut points_to_check = [(0, 0), (0, 0)];

                match (dx, dy) {
                    (1, -1) => {
                        // UP_RIGHT
                        points_to_check = [
                            (coordinates.0, coordinates.1.wrapping_sub(1)), // UP
                            (coordinates.0 + 1, coordinates.1),             // RIGHT
                        ];
                    }
                    (-1, -1) => {
                        // UP_LEFT
                        points_to_check = [
                            (coordinates.0, coordinates.1.wrapping_sub(1)), // UP
                            (coordinates.0.wrapping_sub(1), coordinates.1), // LEFT
                        ];
                    }
                    (1, 1) => {
                        // DOWN_RIGHT
                        points_to_check = [
                            (coordinates.0, coordinates.1 + 1), // DOWN
                            (coordinates.0 + 1, coordinates.1), // RIGHT
                        ];
                    }
                    (-1, 1) => {
                        // DOWN_LEFT
                        points_to_check = [
                            (coordinates.0, coordinates.1 + 1),             // DOWN
                            (coordinates.0.wrapping_sub(1), coordinates.1), // LEFT
                        ];
                    }
                    _ => {}
                }

                if points_to_check.iter().all(|&(x, y)| {
                    x < grid.len()
                        && y < grid[0].len()
                        && grid[x][y] == grid[coordinates.0][coordinates.1]
                }) {
                    edges.insert((dx, dy));
                }
            }
        }
    }

    current_edges += edges.len() as u64;
    (current_edges, current_area)
}
