pub fn day_one_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let numbers: Vec<i32> = lines.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for n1 in numbers.clone() {
        for n2 in numbers.clone() {
            if n1 + n2 == 2020 {
                println!("{}", n1 * n2);
                return;
            }
        } 
    }
}

pub fn day_one_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let numbers: Vec<i32> = lines.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for n1 in numbers.clone() {
        for n2 in numbers.clone() {
            for n3 in numbers.clone() {
                if n1 + n2 + n3 == 2020 {
                    println!("{}", n1 * n2 * n3);
                    return;
                }
            }
        } 
    }
}