use std::collections::HashMap;

pub fn day_fifteen_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let sequence: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut sequence_map: HashMap<i32, usize> = HashMap::new();

    for (index, num) in sequence.clone().into_iter().enumerate() {
        if index == sequence.len() - 1 {
            break;
        }
        sequence_map.insert(num, index);
    }

    let mut last_spoken = *sequence.last().unwrap();
    for i in sequence.len()..(2020 as usize) {
        let mut next_spoken = 0;
        match sequence_map.get(&last_spoken) {
            Some(index) => {
                next_spoken = (i - index - 1) as i32;
            },
            None => {},
        }
        sequence_map.insert(last_spoken, i - 1);
        last_spoken = next_spoken;
    }
    println!("{}", last_spoken);
}

pub fn day_fifteen_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let sequence: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut sequence_map: HashMap<i32, usize> = HashMap::new();

    for (index, num) in sequence.clone().into_iter().enumerate() {
        if index == sequence.len() - 1 {
            break;
        }
        sequence_map.insert(num, index);
    }

    let mut last_spoken = *sequence.last().unwrap();
    for i in sequence.len()..(30000000 as usize) {
        let mut next_spoken = 0;
        match sequence_map.get(&last_spoken) {
            Some(index) => {
                next_spoken = (i - index - 1) as i32;
            },
            None => {},
        }
        sequence_map.insert(last_spoken, i - 1);
        last_spoken = next_spoken;
    }
    println!("{}", last_spoken);
}
