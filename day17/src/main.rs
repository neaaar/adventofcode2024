use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error while reading file");
    let now = Instant::now();

    //start parsing
    let sections: Vec<&str> = contents.split("\n\n").collect();
    let registers: Vec<i32> = sections[0]
        .lines()
        .map(|line| line[12..].parse::<i32>().unwrap())
        .collect();

    let program: Vec<i32> = sections[1][9..]
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut device = Machine::new(registers[0], registers[1], registers[2], program);
    let elapsed_parsing = now.elapsed();

    //done with parsing, can start solving part 1
    let now = Instant::now();
    let part1: Vec<String> = device
        .run(device.reg_a, device.reg_b, device.reg_c)
        .iter()
        .map(|num| num.to_string())
        .collect();

    println!("{}", part1.join(","));
    let elapsed_part1 = now.elapsed();

    //done with part 1, can start solving part 2
    let now = Instant::now();
    let part2 = device.find(&device.program, 0).unwrap();

    println!("{}", part2);
    let elapsed_part2 = now.elapsed();

    println!("Time for parsing: {:.2?}", elapsed_parsing);
    println!("Time for part 1: {:.2?}", elapsed_part1);
    println!("Time for part 2: {:.2?}", elapsed_part2);
}

#[derive(Clone)]
struct Machine {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    program: Vec<i32>,
}

impl Machine {
    fn new(reg_a: i32, reg_b: i32, reg_c: i32, program: Vec<i32>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            program,
        }
    }

    fn combo(&self, operand: i32) -> i32 {
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => operand,
        }
    }

    fn run(&mut self, reg_a: i32, reg_b: i32, reg_c: i32) -> Vec<i32> {
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;

        let mut pc = 0;
        let mut output = Vec::new();

        while pc < self.program.len() {
            let instr = self.program[pc];
            let operand = self.program[pc + 1];
            pc += 2;

            match instr {
                0 => self.reg_a >>= self.combo(operand),
                1 => self.reg_b ^= operand,
                2 => self.reg_b = self.combo(operand) % 8,
                3 => {
                    if self.reg_a != 0 {
                        pc = operand as usize;
                    }
                }
                4 => self.reg_b ^= self.reg_c,
                5 => output.push(self.combo(operand) % 8),
                6 => self.reg_b = self.reg_a >> self.combo(operand),
                7 => self.reg_c = self.reg_a >> self.combo(operand),
                _ => (),
            }
        }

        output
    }

    fn find(&self, program: &[i32], ans: u64) -> Option<u64> {
        if program.is_empty() {
            return Some(ans);
        }

        if let Some(&last_value) = program.last() {
            for t in 0..8 {
                let a = (ans << 3) + t;
                let mut b = t ^ 3;
                let c = a >> b;
                b ^= c;
                b ^= 5;

                if b % 8 == last_value as u64 {
                    let sub = self.find(&program[0..program.len() - 1], a);
                    if sub.is_none() {
                        continue;
                    }
                    return Some(sub)?;
                }
            }
        }

        Some(ans)
    }
}
