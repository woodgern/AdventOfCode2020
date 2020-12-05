pub fn day_five_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut largest_seat_id = 0;
    for line in lines {
        let mut row_string = String::new();
        let mut col_string = String::new();
        for c in line.chars() {
            if c == 'F' {
                row_string.push('0');
            } else if c == 'B' {
                row_string.push('1');
            } else if c == 'L' {
                col_string.push('0');
            } else {
                col_string.push('1');
            }
        }
        let row = i64::from_str_radix(&row_string, 2).unwrap();
        let col = i64::from_str_radix(&col_string, 2).unwrap();
        let seat_id = (row * 8) + col;

        if seat_id > largest_seat_id {
            largest_seat_id = seat_id;
        }
    }
    println!("{}", largest_seat_id);
}

pub fn day_five_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut seats: Vec<i64> = Vec::new();
    for line in lines {
        let mut row_string = String::new();
        let mut col_string = String::new();
        for c in line.chars() {
            if c == 'F' {
                row_string.push('0');
            } else if c == 'B' {
                row_string.push('1');
            } else if c == 'L' {
                col_string.push('0');
            } else {
                col_string.push('1');
            }
        }
        let row = i64::from_str_radix(&row_string, 2).unwrap();
        let col = i64::from_str_radix(&col_string, 2).unwrap();
        let seat_id = (row * 8) + col;
        seats.push(seat_id);
    }
    seats.sort();
    let mut last_seat = 0;
    for seat in seats {
        if last_seat == 0 {
            last_seat = seat;
            continue;
        }

        if seat - 1 != last_seat {
            println!("{}", seat - 1);
            break;
        }
        last_seat = seat;
    }
}