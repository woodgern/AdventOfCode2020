use std::collections::HashMap;
use std::collections::HashSet;

fn count_unique_answers(answers: Vec<&str>) -> i32 {
    let mut answer_set = HashSet::new();
    for answer in answers {
        for c in answer.chars() {
            answer_set.insert(c);
        }
    }
    return answer_set.len() as i32;
}

pub fn day_six_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut group_answers = Vec::new();
    let mut total_count: i32 = 0;
    for line in lines {
        if line.len() == 0 {
            total_count += count_unique_answers(group_answers);
            group_answers = Vec::new();
        } else {
            group_answers.push(line);
        }
    }
    total_count += count_unique_answers(group_answers);
    println!("{}", total_count);
}

fn count_all_answers(answers: Vec<&str>) -> i32 {
    let mut answer_map = HashMap::new();
    let total_answers = answers.len();
    for answer in answers {
        for c in answer.chars() {
            match answer_map.get(&c) {
                Some(total) => answer_map.insert(c, total + 1),
                None => answer_map.insert(c, 1),
            };
        }
    }
    let mut count: i32 = 0;
    for val in answer_map.values() {
        if *val == total_answers {
            count += 1;
        }
    }
    return count;
}

pub fn day_six_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut group_answers = Vec::new();
    let mut total_count: i32 = 0;
    for line in lines {
        if line.len() == 0 {
            total_count += count_all_answers(group_answers);
            group_answers = Vec::new();
        } else {
            group_answers.push(line);
        }
    }
    total_count += count_all_answers(group_answers);
    println!("{}", total_count);
}