use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut prev = levels[0];
    let (mut increasing, mut decreasing) = (false, false);

    for &num in &levels[1..] {
        let diff = num - prev;
        if !increasing && !decreasing {
            match prev.cmp(&num) {
                Ordering::Less => increasing = true,
                Ordering::Greater => decreasing = true,
                Ordering::Equal => return false,
            }
        }

        if (increasing && !(1..=3).contains(&diff)) || (decreasing && !(-3..=-1).contains(&diff)) {
            return false;
        }
        prev = num;
    }
    true
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    //no need for particular parsing in this problem
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let mut part1 = 0;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Error while parsing"))
            .collect();
        if is_safe(&levels) {
            part1 += 1;
        }
    }
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let mut part2 = 0;
    for line in contents.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Error while parsing"))
            .collect();
        if is_safe(&levels) {
            part2 += 1;
        } else {
            for i in 0..levels.len() {
                let mut modified_levels = levels.clone();
                modified_levels.remove(i);

                if is_safe(&modified_levels) {
                    part2 += 1;
                    break;
                }
            }
        }
    }
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}
