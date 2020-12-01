mod day_one;

use std::env;
use std::fs;
use std::time::SystemTime;

use day_one::day_one_problem_one;
use day_one::day_one_problem_two;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args[1].as_str();
    let problem_input_file_name = format!("inputs/{}.txt", problem);
    let problem_input = fs::read_to_string(problem_input_file_name).expect("Input not found");
    
    let start = SystemTime::now();

    match args[1].as_str() {
        "1-1" => day_one_problem_one(problem_input),
        "1-2" => day_one_problem_two(problem_input),
        _ => println!("Solution not found"),
    }

    match start.elapsed() {
        Ok(elapsed) => {
            println!("Solution for {} ran in {:?}", problem, elapsed);
        },
        Err(_) => println!("Timing broken, hell is frozen"),
    }
}
