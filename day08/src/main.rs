use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let lines: Vec<&str> = contents.lines().collect();
    let max_len = lines.len();
    let mut positions = HashMap::new();

    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            if ch != '.' {
                positions
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push((row_idx as i32, col_idx as i32));
            }
        }
    }

    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();

    let mut antinodes = HashSet::new();
    for key in positions.keys() {
        if let Some(points) = positions.get(key) {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let p1 = points[i];
                    let p2 = points[j];

                    for antinode in get_antinodes_p1(p1, p2, max_len as i32) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    let part1 = antinodes.len();
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();

    antinodes.clear();
    for key in positions.keys() {
        if let Some(points) = positions.get(key) {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let p1 = points[i];
                    let p2 = points[j];

                    for antinode in get_antinodes_p2(p1, p2, max_len as i32) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    let part2 = antinodes.len();
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn get_antinodes_p1(
    a: (i32, i32),
    b: (i32, i32),
    max_len: i32,
) -> impl Iterator<Item = (i32, i32)> {
    let (ax, ay) = a;
    let (bx, by) = b;

    let cx = ax - (bx - ax);
    let cy = ay - (by - ay);
    let dx = bx + (bx - ax);
    let dy = by + (by - ay);

    let mut result = Vec::new();

    if in_bounds(cx, cy, max_len) {
        result.push((cx, cy));
    }
    if in_bounds(dx, dy, max_len) {
        result.push((dx, dy));
    }

    result.into_iter()
}

fn get_antinodes_p2(
    a: (i32, i32),
    b: (i32, i32),
    max_len: i32,
) -> impl Iterator<Item = (i32, i32)> {
    let (ax, ay) = a;
    let (bx, by) = b;

    let dx = bx - ax;
    let dy = by - ay;

    let mut result = Vec::new();

    let mut i = 0;
    loop {
        let x = ax - dx * i;
        let y = ay - dy * i;
        if !in_bounds(x, y, max_len) {
            break;
        }
        result.push((x, y));
        i += 1;
    }

    let mut i = 0;
    loop {
        let x = bx + dx * i;
        let y = by + dy * i;
        if !in_bounds(x, y, max_len) {
            break;
        }
        result.push((x, y));
        i += 1;
    }

    result.into_iter()
}

fn in_bounds(x: i32, y: i32, max_len: i32) -> bool {
    x >= 0 && x < max_len && y >= 0 && y < max_len
}
