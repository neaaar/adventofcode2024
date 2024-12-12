use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let stones: Vec<String> = contents.split_whitespace().map(String::from).collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();
    let mut part1 = 0;

    for stone in &stones {
        part1 += blink(stone, 0, 25, &mut cache);
    }
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();
    let mut part2 = 0;

    for stone in &stones {
        part2 += blink(stone, 0, 75, &mut cache);
    }
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn blink(
    stone: &str,
    blinks: usize,
    max_blinks: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if blinks == max_blinks {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone.to_string(), blinks)) {
        return result;
    }

    let result = if stone == "0" {
        blink("1", blinks + 1, max_blinks, cache)
    } else if stone.len() % 2 == 0 {
        let mid = stone.len() / 2;
        let left = &stone[..mid];
        let right = &stone[mid..];

        let right = right.parse::<u64>().unwrap_or(0).to_string();
        blink(left, blinks + 1, max_blinks, cache) + blink(&right, blinks + 1, max_blinks, cache)
    } else {
        let num = stone.parse::<u64>().unwrap_or(0) * 2024;
        blink(&num.to_string(), blinks + 1, max_blinks, cache)
    };

    cache.insert((stone.to_string(), blinks), result);
    result
}
