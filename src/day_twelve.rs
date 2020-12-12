pub fn day_twelve_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut direction: i16 = 0;
    for line in lines {
        let instruction = line.get(0..1).unwrap();
        let value = line.get(1..).unwrap().parse::<i16>().unwrap() as f64;
        match instruction {
            "N" => {
                y += value;
            },
            "S" => {
                y -= value;
            },
            "E" => {
                x += value;
            },
            "W" => {
                x -= value;
            },
            "L" => {
                direction -= value as i16;
            },
            "R" => {
                direction += value as i16;
            },
            "F" => {
                let y_delta = (direction as f64).to_radians().sin() * value;
                let x_delta = (direction as f64).to_radians().cos() * value;

                y -= y_delta;
                x += x_delta;
            },
            _ => {
                println!("BAD INPUT");
            }
        }
    }
    println!("{}", x.abs() + y.abs());
}

pub fn day_twelve_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut x_ship: f64 = 0.0;
    let mut y_ship: f64 = 0.0;
    let mut x_way: f64 = 10.0;
    let mut y_way: f64 = 1.0;
    for line in lines {
        let instruction = line.get(0..1).unwrap();
        let value = line.get(1..).unwrap().parse::<i16>().unwrap() as f64;
        match instruction {
            "N" => {
                y_way += value;
            },
            "S" => {
                y_way -= value;
            },
            "E" => {
                x_way += value;
            },
            "W" => {
                x_way -= value;
            },
            "L" => {
                let hyp = (x_way.powf(2.0) + y_way.powf(2.0)).sqrt();
                let theta = (y_way as f64).atan2(x_way as f64).to_degrees();
                let new_theta = theta + value;
                x_way = new_theta.to_radians().cos() * hyp;
                y_way = new_theta.to_radians().sin() * hyp;
            },
            "R" => {
                let hyp = (x_way.powf(2.0) + y_way.powf(2.0)).sqrt();
                let theta = (y_way as f64).atan2(x_way as f64).to_degrees();
                let new_theta = theta - value;
                x_way = new_theta.to_radians().cos() * hyp;
                y_way = new_theta.to_radians().sin() * hyp;
            },
            "F" => {
                x_ship += x_way * value;
                y_ship += y_way * value;
            },
            _ => {
                println!("BAD INPUT");
            }
        }
    }
    println!("{}, {}: {}, {}", x_ship, y_ship, x_way, y_way);
    println!("{}", x_ship.abs() + y_ship.abs());
}