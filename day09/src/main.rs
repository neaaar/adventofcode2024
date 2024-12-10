use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let mut blocks: Vec<Option<i32>> = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    let mut loc = Vec::new();
    let mut size = Vec::new();

    for c in contents.chars() {
        let n: usize = c.to_digit(10).unwrap_or(0) as usize;
        if is_file {
            loc.push(blocks.len());
            size.push(n);
            blocks.extend(vec![Some(file_id); n].iter());
            file_id += 1;
            is_file = false;
        } else {
            blocks.extend(vec![None; n].iter());
            is_file = true;
        }
    }
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();

    let mut first_free: usize = 0;
    while blocks[first_free].is_some() {
        first_free += 1;
    }

    let mut i = blocks.len() - 1;
    while blocks[i].is_none() {
        i -= 1;
    }

    while first_free < i {
        blocks[first_free] = blocks[i];
        blocks[i] = None;
        while blocks[first_free].is_some() {
            first_free += 1;
        }

        while blocks[i].is_none() {
            i -= 1;
        }
    }

    let part1: i64 = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|b| i as i64 * b as i64))
        .sum();

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();

    let mut big = 0;
    while big < size.len() && size[big] > 0 {
        big += 1;
    }
    big -= 1;

    for to_move in (0..=big).rev() {
        let mut first_free = 0;

        while first_free + size[to_move] <= loc[to_move] {
            let mut free_space = 0;

            while first_free + free_space < blocks.len()
                && blocks[first_free + free_space].is_none()
            {
                free_space += 1;
                if free_space == size[to_move] {
                    break;
                }
            }

            if free_space >= size[to_move] {
                break;
            }

            first_free += free_space + 1;
        }

        if first_free + size[to_move] > loc[to_move] || first_free + size[to_move] > blocks.len() {
            continue;
        }

        for idx in 0..size[to_move] {
            blocks[first_free + idx] = Some(to_move as i32);
            blocks[loc[to_move] + idx] = None;
        }
    }

    let part2: i64 = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|b| i as i64 * b as i64))
        .sum();

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}
