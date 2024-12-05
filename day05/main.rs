use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("example.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let instruction_vector: Vec<Vec<i32>> = parts[0]
        .lines()
        .map(|line| {
            line.split("|")
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
    let page_order_vector: Vec<Vec<i32>> = parts[1]
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
    let mut invalid_page_values: Vec<Vec<i32>> = Vec::new();
    for page_order in page_order_vector {
        let mut seen = Vec::new();
        let mut valid = true;
        invalid_page_values.push(page_order.clone());
        let middle_value = page_order[(page_order.len() - 1) / 2];

        for num in page_order {
            seen.push(num);
            for instuction in &instruction_vector {
                if num == instuction[0] && seen.contains(&instuction[1]) {
                    valid = false;
                }
            }
        }

        if valid {
            part1 += middle_value;
            invalid_page_values.remove(invalid_page_values.len() - 1);
        }
        println!("{:?}", invalid_page_values);
    }

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let mut part2 = 0;
    for invalid_page_value in invalid_page_values {
        let sorted_values = topological_sort(&invalid_page_value, &instruction_vector);
        println!("{}", sorted_values.len());
        if sorted_values.is_empty() {
            continue;
        }
        let middle_value = sorted_values[(sorted_values.len() - 1) / 2];
        part2 += middle_value;
    }

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn topological_sort(values: &Vec<i32>, instructions: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new(); //grado entrante

    for &value in values {
        graph.entry(value).or_default();
        in_degree.entry(value).or_insert(0);
    }

    for instruction in instructions {
        graph
            .entry(instruction[0])
            .or_default()
            .push(instruction[1]);

        *in_degree.entry(instruction[1]).or_insert(0) += 1;
    }

    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&value, _)| value)
        .collect();

    let mut sorted: Vec<i32> = Vec::new();

    while let Some(value) = queue.pop_front() {
        sorted.push(value);

        if let Some(neighbors) = graph.get(&value) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    println!("{:?}", sorted);

    if sorted.len() != values.len() {
        println!("Error: The graph contains a cycle or missing nodes.");
    }
    sorted
}
