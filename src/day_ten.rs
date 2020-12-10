pub fn day_ten_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut adapters: Vec<i16> = Vec::new();
    for line in lines {
        adapters.push(line.parse::<i16>().unwrap());
    }

    let mut sorted_adapters: Vec<i16> = adapters.clone();
    sorted_adapters.sort();
    let mut one_diffs: i16 = 0;
    let mut three_diffs: i16 = 1;
    let mut last_adapter: i16 = 0;
    for adapter in sorted_adapters.clone() {
        let diff = adapter - last_adapter;
        println!("{}", diff);
        if diff == 1 {
            one_diffs += 1;
        } else if diff == 3 {
            three_diffs += 1;
        }
        last_adapter = adapter;
    }
    println!("{}", one_diffs * three_diffs);
}

pub fn day_ten_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut adapters: Vec<i16> = Vec::new();
    for line in lines {
        adapters.push(line.parse::<i16>().unwrap());
    }

    let mut sorted_adapters: Vec<i16> = adapters.clone();
    sorted_adapters.sort();
    println!("{:?}", sorted_adapters);
    let mut one_diffs: i16 = 0;
    let mut last_adapter: i16 = 0;
    let mut product: i64 = 1;
    for adapter in sorted_adapters.clone() {
        let diff = adapter - last_adapter;
        if diff == 1 {
            one_diffs += 1;
        } else if diff == 3 {
            product *= match one_diffs {
                0 => 1,
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 7,
                _ => panic!("AHHH"),
            };
            one_diffs = 0;
        }
        last_adapter = adapter;
    }
    println!("{}", product);
}