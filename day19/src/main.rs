use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let patterns: Vec<String> = sections[0]
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs: Vec<String> = sections[1].lines().map(|s| s.trim().to_string()).collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut part1 = 0;
    let mut cache = HashMap::new();

    for design in &designs {
        if is_possible(design, &patterns, &mut cache) {
            part1 += 1;
        }
    }

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut part2 = 0;

    for design in &designs {
        part2 += count_combinations(design, &patterns);
    }

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn is_possible(design: &str, patterns: &[String], cache: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&cached_result) = cache.get(design) {
        return cached_result;
    }

    for pattern in patterns {
        let n = pattern.len();
        if design.len() >= n
            && design[..n] == *pattern
            && is_possible(&design[n..], patterns, cache)
        {
            cache.insert(design.to_string(), true);
            return true;
        }
    }

    cache.insert(design.to_string(), false);
    false
}

fn count_combinations(design: &str, patterns: &[String]) -> usize {
    let n = design.len();
    let mut dp = vec![0_usize; n + 1]; //dp[i] stores the number of ways to construct design[0..i]
    dp[0] = 1;

    for i in 1..=n {
        for pattern in patterns {
            let pattern_len = pattern.len();
            if i >= pattern_len && &design[i - pattern_len..i] == pattern {
                dp[i] += dp[i - pattern_len];
            }
        }
    }

    dp[n] //the result is the number of ways to construct the entire design
}
