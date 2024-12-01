use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt");
    let now = Instant::now();

    //start parsing
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut i = 0;
    if let Ok(line) = contents {
        for num in line.split_whitespace() {
            match i % 2 {
                0 => left.push(num.parse().expect("Error while parsing")),
                1 => right.push(num.parse().expect("Error while parsing")),
                _ => continue,
            }
            i += 1;
        }
    }
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    left.sort();
    right.sort();
    i = 0;
    let mut part1 = 0;
    while i < left.len() && i < right.len() {
        part1 += (left[i] - right[i]).abs();
        i += 1;
    }
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut count;
    let mut part2 = 0;
    i = 0;
    for left_num in left {
        count = 0;
        while i < right.len() && right[i] <= left_num {
            if left_num == right[i] {
                count += 1
            };
            i += 1;
        }
        part2 += left_num * count;
    }
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}
