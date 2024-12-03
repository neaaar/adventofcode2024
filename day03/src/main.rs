use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    //for lines in contents.lines() {}
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let re_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let re_numbers = Regex::new(r"(\d+),(\d+)").unwrap();
    let part1: i32 = re_mul
        .find_iter(&contents)
        .filter_map(|mat| {
            re_numbers.captures(mat.as_str()).and_then(|nums| {
                let num1: i32 = nums.get(1)?.as_str().parse().ok()?;
                let num2: i32 = nums.get(2)?.as_str().parse().ok()?;
                Some(num1 * num2)
            })
        })
        .sum();
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let re_instructions = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let re_numbers = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut enabled = true;
    let mut part2 = 0;

    for instruction in re_instructions.find_iter(&contents) {
        let instr = instruction.as_str();

        match instr {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if instr.starts_with("mul(") && enabled {
                    if let Some(nums) = re_numbers.captures(instr) {
                        let num1: i32 = nums.get(1).unwrap().as_str().parse().ok().unwrap();
                        let num2: i32 = nums.get(2).unwrap().as_str().parse().ok().unwrap();
                        part2 += num1 * num2;
                    }
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
