use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let re = Regex::new(r"[+=](\d+)").unwrap();
    let machines: Vec<Vec<i64>> = contents
        .split("\n\n")
        .map(|group| {
            re.captures_iter(group)
                .filter_map(|m| m.get(1))
                .filter_map(|m| m.as_str().parse().ok())
                .collect()
        })
        .collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut part1 = 0;

    for group in &machines {
        let (a, b, c, d, e, f) = (group[0], group[2], group[1], group[3], group[4], group[5]);
        let det = a * d - b * c;

        if det == 0 {
            continue; //skip systems with no solution
        }

        if (d * e - b * f) % det != 0 || (a * f - c * e) % det != 0 {
            continue; //skip systems with no integer solutions
        }

        let x = (d * e - b * f) / det;
        let y = (a * f - c * e) / det;

        if x > 100 || y > 100 {
            continue; //skip when x and y are out of bounds
        }

        part1 += x * 3 + y; //add to part1 if all constraints are satisfied
    }
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut part2 = 0;

    for group in &machines {
        let (a, b, c, d, e, f) = (
            group[0],
            group[2],
            group[1],
            group[3],
            group[4] + 10000000000000,
            group[5] + 10000000000000,
        );
        let det = a * d - b * c;

        if det == 0 {
            continue; //skip systems with no solution
        }

        if (d * e - b * f) % det != 0 || (a * f - c * e) % det != 0 {
            continue; //skip systems with no integer solutions
        }

        let x = (d * e - b * f) / det;
        let y = (a * f - c * e) / det;

        part2 += x * 3 + y; //add to part1 if all constraints are satisfied
    }

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}
