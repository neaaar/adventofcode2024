use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let px = caps[1].parse().unwrap();
            let py = caps[2].parse().unwrap();
            let vx = caps[3].parse().unwrap();
            let vy = caps[4].parse().unwrap();

            Robot::new((px, py), (vx, vy))
        })
        .collect();
    let mut robots2 = robots.clone();
    let width = 101;
    let height = 103;

    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();

    for robot in robots.iter_mut() {
        for _ in 0..100 {
            robot.update(width, height);
        }
    }

    let part1 = calculate_safety_factor(&robots, width, height);
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let output_dir = "grid_states";

    for step in 0..10000 {
        for robot in robots2.iter_mut() {
            robot.update(width, height);
        }

        if has_cluster(&robots2, width, height, 8) {
            save_grid_as_svg(&robots2, width, height, step, output_dir);
        }
    }

    //println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

type Point = (i32, i32);
type Position = Point;
type Velocity = Point;

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Robot { position, velocity }
    }

    fn update(&mut self, width: i32, height: i32) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(width);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(height);
    }
}

fn calculate_safety_factor(robots: &[Robot], width: i32, height: i32) -> i32 {
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        //skip robots on the middle lines
        if robot.position.0 == width / 2 || robot.position.1 == height / 2 {
            continue;
        }

        match (robot.position.0 < width / 2, robot.position.1 < height / 2) {
            (true, true) => q1 += 1,   //top-left
            (false, true) => q2 += 1,  //top-right
            (true, false) => q3 += 1,  //bottom-left
            (false, false) => q4 += 1, //bottom-right
        };
    }

    q1 * q2 * q3 * q4
}

fn save_grid_as_svg(robots: &[Robot], width: i32, height: i32, step: usize, output_dir: &str) {
    // Create output directory if it doesn't exist
    let _ = fs::create_dir_all(output_dir);

    // Calculate cell size (you can adjust these values)
    let cell_size = 20;
    let svg_width = width * cell_size;
    let svg_height = height * cell_size;

    // Create position counts map
    let mut position_counts: HashMap<(i32, i32), usize> = HashMap::new();
    for robot in robots {
        *position_counts.entry(robot.position).or_insert(0) += 1;
    }

    // Start building SVG content
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        svg_width, svg_height, svg_width, svg_height
    );

    // Add background grid
    for y in 0..height {
        for x in 0..width {
            let px = x * cell_size;
            let py = y * cell_size;

            // Add cell background
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="white" stroke="lightgray" stroke-width="1"/>"#,
                px, py, cell_size, cell_size
            ));

            // Add robot count if any
            if let Some(&count) = position_counts.get(&(x, y)) {
                if count > 0 {
                    let text_x = px + cell_size / 2;
                    let text_y = py + cell_size / 2;
                    let display_char = if count <= 9 {
                        count.to_string()
                    } else {
                        "#".to_string()
                    };

                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" fill="green" text-anchor="middle" dominant-baseline="middle" font-family="Arial" font-size="{}">{}</text>"#,
                        text_x, text_y, cell_size * 2 / 3, display_char
                    ));
                }
            }
        }
    }

    // Add step number
    svg.push_str(&format!(
        r#"<text x="10" y="20" fill="black" font-family="Arial" font-size="16">Step: {}</text>"#,
        step
    ));

    // Close SVG tag
    svg.push_str("</svg>");

    // Save to file
    let filename = format!("{}/grid_{:05}.svg", output_dir, step + 1);
    let _ = fs::write(&filename, svg);
}

fn has_cluster(robots: &[Robot], width: i32, height: i32, cluster_size: i32) -> bool {
    let mut grid = vec![vec![0; width as usize]; height as usize];

    for robot in robots {
        grid[robot.position.1 as usize][robot.position.0 as usize] += 1;
    }

    for y in 0..=(height - cluster_size) {
        for x in 0..=(width - cluster_size) {
            let mut cluster_sum = 0;

            for dy in 0..cluster_size {
                for dx in 0..cluster_size {
                    cluster_sum += grid[(y + dy) as usize][(x + dx) as usize];
                }
            }

            if cluster_sum >= cluster_size * cluster_size {
                return true;
            }
        }
    }

    false
}
