use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let seeds: Vec<usize> = contents
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut part1 = 0;
    for seed in &seeds {
        let mut secret_number = *seed;
        for _ in 0..2000 {
            secret_number = ((secret_number * 64) ^ secret_number) % 16777216;
            secret_number = ((secret_number / 32) ^ secret_number) % 16777216;
            secret_number = ((secret_number * 2048) ^ secret_number) % 16777216;
        }
        part1 += secret_number;
    }

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut change_counts = HashMap::new();

    for seed in &seeds {
        let mut secret_number = *seed;
        let mut offers = Vec::new();
        for _ in 0..2000 {
            secret_number = ((secret_number * 64) ^ secret_number) % 16777216;
            secret_number = ((secret_number / 32) ^ secret_number) % 16777216;
            secret_number = ((secret_number * 2048) ^ secret_number) % 16777216;
            offers.push((secret_number % 10) as i8);
        }

        let mut seen = HashSet::new();
        for i in 4..offers.len() {
            let changes = [
                offers[i - 3] - offers[i - 4],
                offers[i - 2] - offers[i - 3],
                offers[i - 1] - offers[i - 2],
                offers[i] - offers[i - 1],
            ];

            if !seen.insert(changes) {
                continue;
            }

            *change_counts.entry(changes).or_insert(0) += offers[i] as usize;
        }
    }

    let part2 = change_counts.values().max().unwrap();
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}
