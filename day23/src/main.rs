use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let pairs: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.trim().split('-').collect())
        .collect();
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for pair in pairs {
        let (x, y) = (pair.first().unwrap(), pair.get(1).unwrap());
        connections.entry(x).or_default().insert(y);
        connections.entry(y).or_default().insert(x);
    }
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let mut sets = HashSet::new();

    for x in connections.keys() {
        if let Some(neighbors_x) = connections.get(x) {
            for y in neighbors_x {
                if let Some(neighbors_y) = connections.get(y) {
                    for z in neighbors_y {
                        if x != z
                            && connections
                                .get(z)
                                .map_or(false, |neighbors_z| neighbors_z.contains(x))
                        {
                            //create a sorted tuple of (x, y, z) to ensure uniqueness
                            let mut triplet = [*x, *y, *z];
                            triplet.sort();
                            sets.insert((
                                triplet[0].to_string(),
                                triplet[1].to_string(),
                                triplet[2].to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
    let part1 = sets
        .iter()
        .filter(|(x, y, z)| x.starts_with('t') || y.starts_with('t') || z.starts_with('t'))
        .count();

    println!("{}", part1);
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    for x in connections.keys() {
        let mut sets: HashSet<Vec<&str>> = HashSet::new();
        let mut req: HashSet<&str> = HashSet::new();

        req.insert(*x);
        search(x, &mut req, &mut sets, &connections);
    }

    //println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

fn search<'a>(
    node: &'a str,
    req: &mut HashSet<&'a str>,
    sets: &mut HashSet<Vec<&'a str>>,
    conns: &HashMap<&'a str, HashSet<&'a str>>,
) {
    //create a sorted key for the current set of nodes in `req`
    let mut key: Vec<&str> = req.iter().cloned().collect();
    key.sort();
    if sets.contains(&key) {
        return;
    }

    //add the key to `sets`
    sets.insert(key);

    //explore neighbors of the current node
    if let Some(neighbors) = conns.get(node) {
        for &neighbor in neighbors {
            //skip if `neighbor` is already in `req`
            if req.contains(neighbor) {
                continue;
            }

            //ensure all nodes in `req` are connected to `neighbor`
            if !req
                .iter()
                .all(|&query| conns.get(query).map_or(false, |qs| qs.contains(neighbor)))
            {
                continue;
            }

            //add the neighbor to `req` and recurse
            req.insert(neighbor);
            search(neighbor, req, sets, conns);
            req.remove(neighbor);
        }
    }
}
