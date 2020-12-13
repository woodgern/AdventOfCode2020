use modinverse::modinverse;

pub fn day_thirteen_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let timestamp = lines[0].parse::<i32>().unwrap();
    let bus_ids: Vec<i32> = lines[1].split(",").into_iter().filter(|x| *x != "x").map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{:?}", bus_ids);
    let mut minutes_waited = 0;
    let mut done = false;
    while !done {
        for bus_id in bus_ids.clone() {
            println!("{}", minutes_waited);
            if (timestamp + minutes_waited) % bus_id == 0 {
                println!("{}", bus_id * minutes_waited);
                done = true;
                break;
            }
        }
        minutes_waited += 1;
    }
}

fn check_candidate(candidate: i128, bus_ids: Vec<(usize, i32)>) -> bool {
    for (offset, bus_id) in bus_ids {
        if (candidate + (offset as i128)) % bus_id as i128 != 0 {
            return false
        }
    }
    return true
}

pub fn day_thirteen_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let bus_ids: Vec<(usize, i32)> = lines[1].split(",").into_iter().enumerate().filter(|x| x.1 != "x").map(|x| (x.0, x.1.parse::<i32>().unwrap())).collect();

    let first_bus = bus_ids[0];
    let remaining_buses = bus_ids.get(1..).unwrap();
    let mut t: i128 = 0;
    let mut modulus: i128 = first_bus.1 as i128;

    for (offset, bus_id) in remaining_buses {
        println!("{}", t);
        let t1 = t * (*bus_id as i128) * modinverse(*bus_id as i128, modulus).unwrap();
        t = t1 + ((*bus_id - *offset as i32) as i128 * (modulus) * modinverse(modulus, *bus_id as i128).unwrap());
        modulus = *bus_id as i128 * modulus;

        t = t % modulus;
    }
    println!("{} mod {}", t, modulus);
}
