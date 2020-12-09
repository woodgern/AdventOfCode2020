pub fn day_nine_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sequence: Vec<i64> = Vec::new();
    for line in lines {
        sequence.push(line.parse::<i64>().unwrap());
    }
    for i in 25..sequence.len() {
        let value = *sequence.get(i).unwrap();
        let preceding = sequence.get(i-25..i).unwrap().to_vec();
        let mut found = false;
        for x in preceding.clone() {
            for y in preceding.clone() {
                if x != y && x + y == value {
                    found = true;
                }
            }
        }
        if !found {
            println!("{}", value);
            break;
        }
    }
}

pub fn day_nine_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sequence: Vec<i64> = Vec::new();
    for line in lines {
        sequence.push(line.parse::<i64>().unwrap());
    }
    for x in 0..sequence.len() {
        for y in x+1..sequence.len() {
            let mut sum: i64 = 0;
            let mut i = x;
            let mut largest: i64 = 0;
            let mut smallest: i64 = std::i64::MAX;
            while i <= y {
                let val = *sequence.get(i).unwrap();
                sum += val;
                if val > largest {
                    largest = val;
                } else if val < smallest {
                    smallest = val;
                }
                if sum > 2089807806 {
                    break;
                }
                i += 1;
            }
            if sum == 2089807806 {
                println!("{}", largest + smallest);
                break;
            }
        }
    }
}