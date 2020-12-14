use std::collections::HashMap;

fn pad(string: String, l: usize) -> String {
    let mut s = string;
    while s.len() < l {
        s = String::from("0") + &s;
    }
    return s;
}

fn apply_mask(mask: &str, value: i64) -> i64 {
    let bin_value = pad(format!("{:b}", value), 36);
    let mut masked_value = String::from("");
    for (index, digit) in bin_value.chars().enumerate() {
        let bit = mask.get((index as usize)..(index+1 as usize)).unwrap();
        if bit == "X" {
            masked_value.push(digit);
        } else {
            masked_value.push_str(bit);
        }
    }
    return i64::from_str_radix(&masked_value.to_owned(), 2).unwrap();
}

pub fn day_fourteen_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = "";
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        if split_line[0] == "mask" {
            mask = split_line[2];
        } else {
            let mem_addr = split_line[0].split("]").collect::<Vec<&str>>()[0].split("[").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
            let value = split_line[2].parse::<i64>().unwrap();
            let masked_value = apply_mask(mask, value);
            memory.insert(mem_addr, masked_value);
        }
    }
    let mut sum = 0;
    for value in memory.values() {
        sum += value;
    }
    println!("{}", sum);
}

fn apply_mem_mask(mask: &str, mem_addr: i64) -> Vec<i64> {
    let bin_addr = pad(format!("{:b}", mem_addr), 36);
    let mut masked_addr = String::from("");
    let mut floating_count = 0;
    for (index, digit) in bin_addr.chars().enumerate() {
        let bit = mask.get((index as usize)..(index+1 as usize)).unwrap();
        match bit {
            "X" => {
                masked_addr.push('X');
                floating_count += 1;
            },
            "1" => {
                masked_addr.push('1');
            },
            "0" => {
                masked_addr.push(digit);
            },
            _ => {
                panic!("BAD INPUT");
            },
        }
    }
    let mut mem_addrs = Vec::new();
    for i in 0..(2 as i64).pow(floating_count) {
        let mut digits = pad(format!("{:b}", i), floating_count as usize);
        let mut mem_addr = String::new();
        for bit in masked_addr.chars() {
            if bit == 'X' {
                mem_addr.push(digits.pop().unwrap());
            } else {
                mem_addr.push(bit);
            }
        }
        mem_addrs.push(i64::from_str_radix(&mem_addr.to_owned(), 2).unwrap());
    }
    return mem_addrs
}

pub fn day_fourteen_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = "";
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        if split_line[0] == "mask" {
            mask = split_line[2];
        } else {
            let mem_addr = split_line[0].split("]").collect::<Vec<&str>>()[0].split("[").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
            let value = split_line[2].parse::<i64>().unwrap();
            let masked_addrs = apply_mem_mask(mask, mem_addr);
            for masked_addr in masked_addrs {
                memory.insert(masked_addr, value);
            }
        }
    }
    let mut sum = 0;
    for value in memory.values() {
        sum += value;
    }
    println!("{}", sum);
}
