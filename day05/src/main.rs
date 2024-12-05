use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let rules: Vec<(i32, i32)> = parts[0]
        .lines()
        .map(|line| {
            let mut nums = line.split("|").filter_map(|num| num.parse::<i32>().ok());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let mut part1 = 0;
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        if follows_rules(&update, &rules) {
            part1 += update[update.len() / 2];
        } else {
            invalid_updates.push(update);
        }
    }

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let mut part2 = 0;

    for mut update in invalid_updates {
        bubble_sort(&mut update, &rules);
        part2 += update[update.len() / 2]
    }

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn follows_rules(update: &[i32], rules: &[(i32, i32)]) -> bool {
    let mut idx = std::collections::HashMap::new();

    for (i, &num) in update.iter().enumerate() {
        idx.insert(num, i);
    }

    for &(a, b) in rules {
        if let (Some(&ia), Some(&ib)) = (idx.get(&a), idx.get(&b)) {
            if ia >= ib {
                return false;
            }
        }
    }

    true
}

fn bubble_sort(update: &mut [i32], rules: &[(i32, i32)]) {
    loop {
        let mut is_sorted = true;

        for i in 0..update.len() - 1 {
            if rules.contains(&(update[i + 1], update[i])) {
                update.swap(i, i + 1);
                is_sorted = false;
            }
        }

        if is_sorted {
            break;
        }
    }
}
