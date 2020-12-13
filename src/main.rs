mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;
mod day_ten;
mod day_eleven;
mod day_twelve;
mod day_thirteen;

use std::env;
use std::fs;
use std::time::SystemTime;

use day_one::day_one_problem_one;
use day_one::day_one_problem_two;

use day_two::day_two_problem_one;
use day_two::day_two_problem_two;

use day_three::day_three_problem_one;
use day_three::day_three_problem_two;

use day_four::day_four_problem_one;
use day_four::day_four_problem_two;

use day_five::day_five_problem_one;
use day_five::day_five_problem_two;

use day_six::day_six_problem_one;
use day_six::day_six_problem_two;

use day_seven::day_seven_problem_one;
use day_seven::day_seven_problem_two;

use day_eight::day_eight_problem_one;
use day_eight::day_eight_problem_two;

use day_nine::day_nine_problem_one;
use day_nine::day_nine_problem_two;

use day_ten::day_ten_problem_one;
use day_ten::day_ten_problem_two;

use day_eleven::day_eleven_problem_one;
use day_eleven::day_eleven_problem_two;

use day_twelve::day_twelve_problem_one;
use day_twelve::day_twelve_problem_two;

use day_thirteen::day_thirteen_problem_one;
use day_thirteen::day_thirteen_problem_two;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args[1].as_str();
    let problem_input_file_name = format!("inputs/{}.txt", problem);
    let problem_input = fs::read_to_string(problem_input_file_name).expect("Input not found");
    
    let start = SystemTime::now();

    match args[1].as_str() {
        "1-1" => day_one_problem_one(problem_input),
        "1-2" => day_one_problem_two(problem_input),
        "2-1" => day_two_problem_one(problem_input),
        "2-2" => day_two_problem_two(problem_input),
        "3-1" => day_three_problem_one(problem_input),
        "3-2" => day_three_problem_two(problem_input),
        "4-1" => day_four_problem_one(problem_input),
        "4-2" => day_four_problem_two(problem_input),
        "5-1" => day_five_problem_one(problem_input),
        "5-2" => day_five_problem_two(problem_input),
        "6-1" => day_six_problem_one(problem_input),
        "6-2" => day_six_problem_two(problem_input),
        "7-1" => day_seven_problem_one(problem_input),
        "7-2" => day_seven_problem_two(problem_input),
        "8-1" => day_eight_problem_one(problem_input),
        "8-2" => day_eight_problem_two(problem_input),
        "9-1" => day_nine_problem_one(problem_input),
        "9-2" => day_nine_problem_two(problem_input),
        "10-1" => day_ten_problem_one(problem_input),
        "10-2" => day_ten_problem_two(problem_input),
        "11-1" => day_eleven_problem_one(problem_input),
        "11-2" => day_eleven_problem_two(problem_input),
        "12-1" => day_twelve_problem_one(problem_input),
        "12-2" => day_twelve_problem_two(problem_input),
        "13-1" => day_thirteen_problem_one(problem_input),
        "13-2" => day_thirteen_problem_two(problem_input),
        _ => println!("Solution not found"),
    }

    match start.elapsed() {
        Ok(elapsed) => {
            println!("Solution for {} ran in {:?}", problem, elapsed);
        },
        Err(_) => println!("Timing broken, hell is frozen"),
    }
}
