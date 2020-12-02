fn is_valid_password(letter: &str, min: i16, max: i16, password: &str) -> bool {
    let l: char = letter.chars().collect::<Vec<char>>()[0];
    let mut count = 0;
    for c in password.chars() {
        if c == l {
            count += 1;
        }
    }
    return count >= min && count <= max
}

pub fn day_two_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut count: i16 = 0;
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        let min_max: Vec<&str> = split_line[0].split("-").collect();
        let letter = &split_line[1][0..1];
        let password = split_line[2];
        if is_valid_password(
            letter,
            min_max[0].parse::<i16>().unwrap(),
            min_max[1].parse::<i16>().unwrap(),
            password,
        ) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_valid_new_password_new_policy(letter: &str, first: i16, second: i16, password: &str) -> bool {
    let l: char = letter.chars().collect::<Vec<char>>()[0];
    let password_chars = password.chars().collect::<Vec<char>>();
    let mut first_position_has_letter = false;
    let mut second_position_has_letter = false;
    if password_chars[(first as usize) - 1] == l {
        first_position_has_letter = true;
    }
    if password_chars[(second as usize) - 1] == l {
        second_position_has_letter = true;
    }
    return (first_position_has_letter || second_position_has_letter) && !(first_position_has_letter && second_position_has_letter)
}

pub fn day_two_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut count: i16 = 0;
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        let min_max: Vec<&str> = split_line[0].split("-").collect();
        let letter = &split_line[1][0..1];
        let password = split_line[2];
        if is_valid_new_password_new_policy(
            letter,
            min_max[0].parse::<i16>().unwrap(),
            min_max[1].parse::<i16>().unwrap(),
            password,
        ) {
            count += 1;
        }
    }
    println!("{}", count);
}