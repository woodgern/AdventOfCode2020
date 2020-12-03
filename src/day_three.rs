pub fn day_three_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let width: i16 = lines[0].len() as i16;
    let mut latitude: i16 = 0;
    let mut count: i16 = 0;
    for line in lines {
        let altitudenal_slice: Vec<char> = line.chars().collect();
        if altitudenal_slice[latitude as usize] == '#' {
            count += 1;
        }
        latitude = (latitude + 3) % width;
    }
    println!("{}", count);
}

pub fn day_three_problem_two(input: String) {
    let mut product: i64 = 1;
    for (right, down) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let lines: Vec<&str> = input.split("\n").collect();
        let width: i16 = lines[0].len() as i16;
        let mut latitude: i16 = 0;
        let mut count: i16 = 0;
        let mut altitude: i16 = 0;
        while altitude < lines.len() as i16 {
            let line = lines[altitude as usize];
            let altitudenal_slice: Vec<char> = line.chars().collect();
            if altitudenal_slice[latitude as usize] == '#' {
                count += 1;
            }
            latitude = (latitude + right) % width;
            altitude += down;
        }
        product *= count as i64;
    }
    println!("{}", product);
}