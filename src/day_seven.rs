use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Bag {
    contents: Vec<(i8, String)>,
}

fn can_contain_shiny_gold(colour: String, bag_map: HashMap<String, Bag>) -> bool {
    match bag_map.get(&colour) {
        Some(contents) => {
            let mut return_val = false;
            for (quantity, contained_colour) in contents.contents.clone() {
                if contained_colour == "shiny gold" {
                    return true;
                } else {
                    return_val = return_val || can_contain_shiny_gold(contained_colour, bag_map.clone());
                }
            }
            return return_val
        },
        None => return false,
    };
}

pub fn day_seven_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut bag_map: HashMap<String, Bag> = HashMap::new();
    let mut colour_list = Vec::new();
    for line in lines {
        let first_split = line.split(" contain ").collect::<Vec<&str>>();
        let mut subject_colour = first_split[0].replace("bags", "").replace("bag", "");
        subject_colour = subject_colour.trim().to_string();
        let contained_colours_string = first_split[1];
        let contained_colors = contained_colours_string.split(", ").collect::<Vec<&str>>();
        
        let mut contents = Vec::new();
        for colour in contained_colors {
            let bag_vec = colour.split(" ").collect::<Vec<&str>>();
            if bag_vec[0] == "no" {
                continue;
            }
            let quantity = bag_vec[0].parse::<i8>().unwrap();
            let colour_name = bag_vec[1..].join(" ").replace(".", "").replace("bags", "").replace("bag", "");
            contents.push((quantity, colour_name.trim().to_owned()));
        }
        bag_map.insert(subject_colour.to_owned(), Bag{contents:contents});
        colour_list.push(subject_colour);
    }
    let mut count: i32 = 0;
    for colour in colour_list {
        if can_contain_shiny_gold(colour.to_owned(), bag_map.clone()) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn count_contained_bags(colour: String, bag_map: HashMap<String, Bag>) -> i64 {
    match bag_map.get(&colour) {
        Some(contents) => {
            let mut total_contained_bags: i64 = 0;
            for (quantity, contained_colour) in contents.contents.clone() {
                total_contained_bags += quantity as i64 * count_contained_bags(contained_colour, bag_map.clone()) + quantity as i64;
            }
            return total_contained_bags
        },
        None => return 0,
    };
}

pub fn day_seven_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut bag_map: HashMap<String, Bag> = HashMap::new();
    let mut colour_list = Vec::new();
    for line in lines {
        let first_split = line.split(" contain ").collect::<Vec<&str>>();
        let mut subject_colour = first_split[0].replace("bags", "").replace("bag", "");
        subject_colour = subject_colour.trim().to_string();
        let contained_colours_string = first_split[1];
        let contained_colors = contained_colours_string.split(", ").collect::<Vec<&str>>();
        
        let mut contents = Vec::new();
        for colour in contained_colors {
            let bag_vec = colour.split(" ").collect::<Vec<&str>>();
            if bag_vec[0] == "no" {
                continue;
            }
            let quantity = bag_vec[0].parse::<i8>().unwrap();
            let colour_name = bag_vec[1..].join(" ").replace(".", "").replace("bags", "").replace("bag", "");
            contents.push((quantity, colour_name.trim().to_owned()));
        }
        bag_map.insert(subject_colour.to_owned(), Bag{contents:contents});
        colour_list.push(subject_colour);
    }
    println!("{}", count_contained_bags(String::from("shiny gold"), bag_map));
}