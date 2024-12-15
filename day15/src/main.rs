use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let grid_str = parts[0];
    let mut grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let instructions: Vec<char> = parts[1].replace("\n", "").chars().collect();
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let part1 = solve(&mut grid, &instructions);
    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let mut doubled_grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|line| line.chars().flat_map(|ch| double(ch)).collect())
        .collect();

    let part2 = solve(&mut doubled_grid, &instructions);
    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn find_starting_position(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&ch| ch == '@') {
            return (i, j);
        }
    }

    (0, 0)
}

fn in_grid(pos: (i32, i32), grid: &[Vec<char>]) -> bool {
    let (x, y) = pos;
    x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len()
}

fn extract_component(
    grid: &mut Vec<Vec<char>>,
    r: usize,
    c: usize,
    dr: i32,
    dc: i32,
) -> Vec<(usize, usize, char)> {
    if grid[r][c] == '#' || grid[r][c] == '.' {
        return vec![];
    }

    let ch = grid[r][c];
    grid[r][c] = '.'; //mark as visited

    let mut component = vec![(r, c, ch)];
    let (new_r, new_c) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
    if in_grid((new_r as i32, new_c as i32), grid) {
        component.extend(extract_component(grid, new_r, new_c, dr, dc));
    }

    if ch == '[' {
        component.extend(extract_component(grid, r, c + 1, dr, dc));
    } else if ch == ']' {
        component.extend(extract_component(grid, r, c - 1, dr, dc));
    }

    component
}

fn solve(grid: &mut Vec<Vec<char>>, instructions: &[char]) -> i32 {
    let mut dirs: HashMap<char, (i32, i32)> = HashMap::new();
    dirs.insert('^', (-1, 0));
    dirs.insert('v', (1, 0));
    dirs.insert('<', (0, -1));
    dirs.insert('>', (0, 1));

    let (mut robot_r, mut robot_c) = find_starting_position(grid);

    for &instr in instructions {
        if let Some(&(dr, dc)) = dirs.get(&instr) {
            let component = extract_component(grid, robot_r, robot_c, dr, dc);

            if component.iter().all(|&(r, c, _)| {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                in_grid((nr, nc), grid) && grid[nr as usize][nc as usize] == '.'
            }) {
                for &(r, c, ch) in &component {
                    let (nr, nc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
                    grid[nr][nc] = ch;
                }

                robot_r = (robot_r as i32 + dr) as usize;
                robot_c = (robot_c as i32 + dc) as usize;
            } else {
                for &(r, c, ch) in &component {
                    grid[r][c] = ch;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' || grid[i][j] == '[' {
                ans += 100 * i + j
            }
        }
    }

    ans as i32
}

fn double(ch: char) -> Vec<char> {
    match ch {
        '@' => vec!['@', '.'],
        '.' => vec!['.', '.'],
        '#' => vec!['#', '#'],
        'O' => vec!['[', ']'],
        _ => vec![],
    }
}
