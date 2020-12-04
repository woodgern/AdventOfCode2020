use std::collections::HashSet;

use regex::Regex;

fn is_valid_keys(passport: Vec<&str>) -> bool {
    let mut passport_fields: HashSet<&str> = HashSet::new();
    for field in passport {
        let field_key = field.split(":").collect::<Vec<&str>>()[0];
        if field_key != "cid" {
            passport_fields.insert(field_key);
        }
    }
    return passport_fields.len() == 7;
}

pub fn day_four_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut passport: Vec<&str> = Vec::new();
    let mut count = 0;
    for line in lines {
        if line.len() != 0 {
            passport.extend(line.split(" ").collect::<Vec<&str>>().clone());
        } else {
            if is_valid_keys(passport.clone()) {
                count += 1;
            }
            passport = Vec::new();
        }
    }
    if is_valid_keys(passport.clone()) {
        count += 1;
    }
    println!("{}", count);

}

fn is_valid_keys_and_fields(passport: Vec<&str>) -> bool {
    let mut passport_fields: HashSet<&str> = HashSet::new();
    for field in passport {
        let field_kv = field.split(":").collect::<Vec<&str>>();
        let field_key = field_kv[0];
        let field_value = field_kv[1];
        if field_key == "byr" {
            let val = field_value.parse::<i32>().unwrap();
            if val >= 1920 && val <= 2002 {
                passport_fields.insert(field_key);
            }
        }
        if field_key == "iyr" {
            let val = field_value.parse::<i32>().unwrap();
            if val >= 2010 && val <= 2020 {
                passport_fields.insert(field_key);
            }
        }
        if field_key == "eyr" {
            let val = field_value.parse::<i32>().unwrap();
            if val >= 2020 && val <= 2030 {
                passport_fields.insert(field_key);
            }
        }
        if field_key == "hgt" {
            let b: Vec<char> = field_value.chars().rev().collect();
            if b[0] == 'n' && b[1] == 'i' {
                let x: Vec<char> = field_value.chars().rev().skip(2).collect::<Vec<char>>();
                let str_val: String = x.into_iter().rev().collect();
                let val = str_val.parse::<i32>().unwrap();
                if val >= 59 && val <= 76 {
                    passport_fields.insert(field_key);
                }

            } else if b[0] == 'm' && b[1] == 'c' {
                let x: Vec<char> = field_value.chars().rev().skip(2).collect::<Vec<char>>();
                let str_val: String = x.into_iter().rev().collect();
                let val = str_val.parse::<i32>().unwrap();
                if val >= 150 && val <= 193 {
                    passport_fields.insert(field_key);
                }
            }
        }
        if field_key == "hcl" {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            if re.is_match(field_value) {
                passport_fields.insert(field_key);
            }
        }
        if field_key == "ecl" {
            if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field_value) {
                passport_fields.insert(field_key);
            }
        }
        if field_key == "pid" {
            let re = Regex::new(r"^[0-9]{9}$").unwrap();
            if re.is_match(field_value) {
                passport_fields.insert(field_key);
            }
        }
    }
    return passport_fields.len() == 7;
}

pub fn day_four_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut passport: Vec<&str> = Vec::new();
    let mut count = 0;
    for line in lines {
        if line.len() != 0 {
            passport.extend(line.split(" ").collect::<Vec<&str>>().clone());
        } else {
            if is_valid_keys_and_fields(passport.clone()) {
                count += 1;
            }
            passport = Vec::new();
        }
    }
    if is_valid_keys_and_fields(passport.clone()) {
        count += 1;
    }
    println!("{}", count);
}