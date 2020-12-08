use std::collections::HashSet;

fn execute(instructions: Vec<(String, i32)>) {
    let mut executed_instructions: HashSet<i32> = HashSet::new();
    let mut pos: i32 = 0;
    let mut acc: i32 = 0;
    while !(pos < 0 || pos > instructions.len() as i32) {
        if executed_instructions.contains(&pos) {
            println!("{}", acc);
            break;
        }
        executed_instructions.insert(pos);
        let (instruction, value) = instructions.get(pos as usize).unwrap();
        match instruction.as_str() {
            "acc" => {
                acc += value;
                pos += 1;
            },
            "jmp" => {
                pos += value;
            },
            _ => {
                pos += 1;
            }
        }
    };
}

pub fn day_eight_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut instructions: Vec<(String, i32)> = Vec::new();
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        let value = split_line[1].parse::<i32>().unwrap();
        instructions.push((split_line[0].to_owned(), value));
    }
    execute(instructions);
}

fn execute_two(instructions: Vec<(String, i32)>) -> i32 {
    let mut executed_instructions: HashSet<i32> = HashSet::new();
    let mut pos: i32 = 0;
    let mut acc: i32 = 0;
    while !(pos < 0 || pos >= instructions.len() as i32) {
        if executed_instructions.contains(&pos) {
            return -1;
        }
        executed_instructions.insert(pos);
        let (instruction, value) = instructions.get(pos as usize).unwrap();
        match instruction.as_str() {
            "acc" => {
                acc += value;
                pos += 1;
            },
            "jmp" => {
                pos += value;
            },
            _ => {
                pos += 1;
            }
        }
    };
    return acc;
}

pub fn day_eight_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut instructions: Vec<(String, i32)> = Vec::new();
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        let value = split_line[1].parse::<i32>().unwrap();
        instructions.push((split_line[0].to_owned(), value));
    }
    for (index, instruction) in instructions.clone().iter().enumerate() {
        let (mut i, value) = instruction.clone();
        match i.as_str() {
            "nop" => {
                i = String::from("jmp");
            },
            "jmp" => {
                i = String::from("nop");
            },
            _ => continue,
        };
        let mut instruction_copy = instructions.clone();
        std::mem::replace(&mut instruction_copy[index], (i, value));
        let acc = execute_two(instruction_copy);
        if acc != -1 {
            println!("{}", acc);
            break;
        }
    }
}