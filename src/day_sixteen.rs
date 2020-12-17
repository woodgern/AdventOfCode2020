use std::collections::HashMap;


fn compute_error_rate_for_ticket(ticket: Vec<i16>, fields: &Vec<(&str, i16, i16, i16, i16)>) -> i16 {
    let mut total_error = 0;
    for ticket_value in ticket {
        let mut bad_value = true;
        for field in fields {
            if (ticket_value >= field.1 && ticket_value <= field.2 || ticket_value >= field.3 && ticket_value <= field.4) {
                bad_value = false;
            }
        }
        if bad_value {
            total_error += ticket_value;
        }
    }
    return total_error;
}

pub fn day_sixteen_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut nearby_tickets: Vec<Vec<i16>> = Vec::new();
    let mut fields: Vec<(&str, i16, i16, i16, i16)> = Vec::new();
    let mut scanning_nearby_tickets = false;
    let mut scanning_fields = true;
    for line in lines {
        if line.len() == 0 {
            scanning_fields = false;
        }
        if scanning_nearby_tickets {
            nearby_tickets.push(line.split(",").map(|x| x.parse::<i16>().unwrap()).collect());
        } else if scanning_fields {
            let split_line: Vec<&str> = line.split(":").collect();
            let field_name = split_line[0];
            let split_ranges: Vec<&str> = split_line[1].split(" ").collect();
            let first_range: Vec<i16> = split_ranges[1].split("-").map(|x| x.parse::<i16>().unwrap()).collect();
            let second_range: Vec<i16> = split_ranges[3].split("-").map(|x| x.parse::<i16>().unwrap()).collect();
            fields.push((field_name, first_range[0], first_range[1], second_range[0], second_range[1]));
        }
        if line == "nearby tickets:" {
            scanning_nearby_tickets = true;
        }
    }

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        error_rate += compute_error_rate_for_ticket(ticket, &fields) as i64;
    }
    println!("{}", error_rate);
}

fn is_valid(ticket: Vec<i16>, fields: &Vec<(&str, i16, i16, i16, i16)>) -> bool {
    for ticket_value in ticket {
        let mut bad_value = true;
        for field in fields {
            if (ticket_value >= field.1 && ticket_value <= field.2 || ticket_value >= field.3 && ticket_value <= field.4) {
                bad_value = false;
            }
        }
        if bad_value {
            return false
        }
    }
    return true
}

fn is_valid_value(value: i16, field: (&str, i16, i16, i16, i16))-> bool {
    return value >= field.1 && value <= field.2 || value >= field.3 && value <= field.4;
}

fn is_fields_solved(field_map: &HashMap<&str, (Vec<usize>, bool)>) -> bool {
    for (x, _) in field_map.values() {
        if x.len() != 1 {
            return false
        }
    }
    return true
}

fn reduce_field_options(f: HashMap<&str, (Vec<usize>, bool)>) -> HashMap<&str, (Vec<usize>, bool)> {
    let mut field_map = f.clone();
    let mut reducing_index = 0;
    for (indexes, complete) in field_map.values() {
        if indexes.len() == 1 && !complete {
            reducing_index = indexes[0];
        }
    }

    for key in field_map.clone().keys() {
        let mut new_indexes = match field_map.get(key) {
            Some((indexes, _)) => {
                indexes.clone()
            },
            None => {
                panic!("key not found???");
            },
        };
        if new_indexes.len() == 1 {
            field_map.insert(key, (new_indexes, true));
            continue;
        }
        let mut index_to_remove = -1;
        for (i, val) in new_indexes.clone().into_iter().enumerate() {
            if val == reducing_index {
                index_to_remove = i as isize;
                break;
            }
        }
        if index_to_remove == -1 {
            continue;
        }
        new_indexes.remove(index_to_remove as usize);
        field_map.insert(key, (new_indexes, false));
    }
    return field_map;
}

pub fn day_sixteen_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut nearby_tickets: Vec<Vec<i16>> = Vec::new();
    let mut fields: Vec<(&str, i16, i16, i16, i16)> = Vec::new();
    let mut scanning_nearby_tickets = false;
    let mut scanning_fields = true;
    for line in lines {
        if line.len() == 0 {
            scanning_fields = false;
        }
        if scanning_nearby_tickets {
            nearby_tickets.push(line.split(",").map(|x| x.parse::<i16>().unwrap()).collect());
        } else if scanning_fields {
            let split_line: Vec<&str> = line.split(":").collect();
            let field_name = split_line[0];
            let split_ranges: Vec<&str> = split_line[1].split(" ").collect();
            let first_range: Vec<i16> = split_ranges[1].split("-").map(|x| x.parse::<i16>().unwrap()).collect();
            let second_range: Vec<i16> = split_ranges[3].split("-").map(|x| x.parse::<i16>().unwrap()).collect();
            fields.push((field_name, first_range[0], first_range[1], second_range[0], second_range[1]));
        }
        if line == "nearby tickets:" {
            scanning_nearby_tickets = true;
        }
    }
    nearby_tickets = nearby_tickets.into_iter().filter(|x| is_valid(x.to_vec(), &fields)).collect();

    let mut field_map: HashMap<&str, (Vec<usize>, bool)> = HashMap::new();
    for field in fields {
        for index in 0..nearby_tickets[0].len() {
            let mut valid_for_index = true;
            for ticket in &nearby_tickets {
                if !is_valid_value(ticket[index], field) {
                    valid_for_index = false;
                    break;
                }
            }
            if valid_for_index {
                let mut valid_indexes = match field_map.get(&field.0) {
                    Some((indexes, complete)) => {
                        indexes.clone()
                    },
                    None => {
                        Vec::new()
                    },
                };
                valid_indexes.push(index);
                field_map.insert(field.0, (valid_indexes, false));
            }
        }
    }

    while !is_fields_solved(&field_map) {
        field_map = reduce_field_options(field_map.clone());
    }
    for key in field_map.keys() {
        let index = field_map.get(key).unwrap().0[0];
        println!("{} is for ticket value at index {}", key, index);
    }
}
