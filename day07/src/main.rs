use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let lines: Vec<Vec<i64>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| {
                    num.split(":")
                        .next()
                        .unwrap_or(num)
                        .parse::<i64>()
                        .expect("Error while parsing")
                })
                .collect()
        })
        .collect();

    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut part1 = 0;

    for line in &lines {
        if helper_part1(line[0], line[1], &line[2..]) {
            part1 += line[0];
        }
    }

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut part2 = 0;

    for line in &lines {
        if helper_part2(line[0], line[1], &line[2..]) {
            part2 += line[0];
        }
    }

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn helper_part1(target: i64, res: i64, nums: &[i64]) -> bool {
    if res > target {
        return false;
    }

    if nums.is_empty() {
        return res == target;
    }

    let current_num = nums[0];
    let add_result = helper_part1(target, res + current_num, &nums[1..]);
    let mul_result = helper_part1(target, res * current_num, &nums[1..]);

    add_result || mul_result
}

fn helper_part2(target: i64, res: i64, nums: &[i64]) -> bool {
    if res > target {
        return false;
    }

    if nums.is_empty() {
        return res == target;
    }

    let add_result = helper_part2(target, res + nums[0], &nums[1..]);
    let mul_result = helper_part2(target, res * nums[0], &nums[1..]);
    let concat_num = concatenate(res, nums[0]);
    let concat_result = if concat_num <= target {
        helper_part2(target, concat_num, &nums[1..])
    } else {
        false
    };

    add_result || mul_result || concat_result
}

fn concatenate(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    let result_str = format!("{}{}", a, b_str);
    result_str.parse().unwrap_or(i64::MAX)
}
