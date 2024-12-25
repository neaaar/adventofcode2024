use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let schematics: Vec<&str> = contents.split("\n\n").collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics {
        let schematic: Vec<Vec<char>> = schematic
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        if schematic[0][0] == '#' {
            locks.push(calculate_lock_height(&schematic));
        } else {
            keys.push(calculate_key_height(&schematic));
        }
    }

    let mut part1 = 0;
    for lock in &locks {
        for key in &keys {
            if fit_together(lock, key) {
                part1 += 1;
            }
        }
    }
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

fn calculate_lock_height(schematic: &[Vec<char>]) -> (i8, i8, i8, i8, i8) {
    let (mut c0, mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0, 0);
    for c in 0..schematic[0].len() {
        for r in 1..schematic.len() {
            if schematic[r][c] == '#' {
                match c {
                    0 => c0 += 1,
                    1 => c1 += 1,
                    2 => c2 += 1,
                    3 => c3 += 1,
                    4 => c4 += 1,
                    _ => (),
                }
            } else {
                continue;
            }
        }
    }

    (c0, c1, c2, c3, c4)
}

fn calculate_key_height(schematic: &[Vec<char>]) -> (i8, i8, i8, i8, i8) {
    let (mut c0, mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0, 0);
    for c in 0..schematic[0].len() {
        for r in (0..schematic.len() - 1).rev() {
            if schematic[r][c] == '#' {
                match c {
                    0 => c0 += 1,
                    1 => c1 += 1,
                    2 => c2 += 1,
                    3 => c3 += 1,
                    4 => c4 += 1,
                    _ => (),
                }
            } else {
                continue;
            }
        }
    }

    (c0, c1, c2, c3, c4)
}

fn fit_together(lock: &(i8, i8, i8, i8, i8), key: &(i8, i8, i8, i8, i8)) -> bool {
    let available_space = 5;
    if lock.0 + key.0 > available_space {
        return false;
    }
    if lock.1 + key.1 > available_space {
        return false;
    }
    if lock.2 + key.2 > available_space {
        return false;
    }
    if lock.3 + key.3 > available_space {
        return false;
    }
    if lock.4 + key.4 > available_space {
        return false;
    }

    true
}
