fn apply_rules_one(seats: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();
    for j in 0..seats.len() {
        let row = seats.get(j).unwrap();
        for i in 0..row.len() {
            let seat = row.get(i).unwrap();
            let mut occupied_seats: i8 = 0;
            if i != 0 && *row.get(i-1).unwrap() == '#' {
                occupied_seats += 1;
            }
            if i + 1 != row.len()  && *row.get(i+1).unwrap() == '#' {
                occupied_seats += 1;
            }
            if j != 0 && *seats.get(j-1).unwrap().get(i).unwrap() == '#' {
                occupied_seats += 1;
            }
            if j + 1 != seats.len() && *seats.get(j+1).unwrap().get(i).unwrap() == '#' {
                occupied_seats += 1;
            }
            if j != 0 && i != 0 && *seats.get(j-1).unwrap().get(i-1).unwrap() == '#' {
                occupied_seats += 1;
            }
            if j != 0 && i + 1 != row.len() && *seats.get(j-1).unwrap().get(i+1).unwrap() == '#' {
                occupied_seats += 1;
            }
            if j + 1 != seats.len() && i != 0 && *seats.get(j+1).unwrap().get(i-1).unwrap() == '#' {
                occupied_seats += 1;
            }
            if j + 1 != seats.len() && i + 1 != row.len() && *seats.get(j+1).unwrap().get(i+1).unwrap() == '#' {
                occupied_seats += 1;
            }

            if *seat == 'L' {
                if occupied_seats == 0 {
                    let mut new_row = new_seats.get(j).unwrap().clone();
                    std::mem::replace(&mut new_row[i], '#');
                    std::mem::replace(&mut new_seats[j], new_row);
                }
            } else if *seat == '#' {
                if occupied_seats >= 4 {
                    let mut new_row = new_seats.get(j).unwrap().clone();
                    std::mem::replace(&mut new_row[i], 'L');
                    std::mem::replace(&mut new_seats[j], new_row);
                }
            }
        }
    }
    return new_seats
}

fn equal(seats1: Vec<Vec<char>>, seats2: Vec<Vec<char>>) -> bool {
    for j in 0..seats1.len() {
        let row1 = seats1.get(j).unwrap();
        let row2 = seats2.get(j).unwrap();
        for i in 0..row1.len() {
            let seat1 = row1.get(i).unwrap();
            let seat2 = row2.get(i).unwrap();
            if *seat1 != *seat2 {
                return false;
            }
        }
    }
    return true;
}

fn count_occupied(seats: Vec<Vec<char>>) -> i16 {
    let mut count: i16 = 0;
    for row in seats {
        for seat in row {
            if seat == '#' {
                count += 1;
            }
        }
    }
    return count;
}

pub fn day_eleven_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut seats: Vec<Vec<char>> = Vec::new();
    for line in lines {
        seats.push(line.chars().collect());
    }

    seats = apply_rules_one(seats.clone());

    while true {
        let old_seats = seats.clone();
        seats = apply_rules_one(seats.clone());
        if equal(seats.clone(), old_seats) {
            println!("{}", count_occupied(seats.clone()));
            break;
        }
    }
}

fn column_above_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if row == 0 {
        return false
    }
    let mut i = row - 1;
    while i >= 0 {
        if *seats.get(i).unwrap().get(col).unwrap() == '#' {
            return true;
        }
        if *seats.get(i).unwrap().get(col).unwrap() == 'L' {
            return false
        }
        if i == 0 {
            return false
        }
        i -= 1;
    }
    return false
}

fn column_below_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    for (i, r) in seats.into_iter().enumerate() {
        if i <= row {
            continue;
        }
        if *r.get(col).unwrap() == '#' {
            return true
        } else if *r.get(col).unwrap() == 'L' {
            return false
        }
    }
    return false
}

fn row_before_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if col == 0 {
        return false
    }
    let mut i = col - 1;
    while i >= 0 {
        if *seats.get(row).unwrap().get(i).unwrap() == '#' {
            return true;
        }
        if *seats.get(row).unwrap().get(i).unwrap() == 'L' {
            return false
        }
        if i == 0 {
            return false
        }
        i -= 1;
    }
    return false
}

fn row_after_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let row = seats.get(row).unwrap();
    for (i, seat) in row.into_iter().enumerate() {
        if i <= col {
            continue;
        }
        if *seat == '#' {
            return true
        } else if *seat == 'L' {
            return false
        }
    }
    return false
}

fn positive_positive_diagonal_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut x = col + 1;
    let mut y = row + 1;
    while y < seats.len() && x < seats.get(0).unwrap().len() {
        if *seats.get(y).unwrap().get(x).unwrap() == '#' {
            return true;
        }
        if *seats.get(y).unwrap().get(x).unwrap() == 'L' {
            return false
        }
        x += 1;
        y += 1;
    }
    return false
}

fn negative_negative_diagonal_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if col == 0 || row == 0 {
        return false;
    }

    let mut x = col - 1;
    let mut y = row - 1;
    while y >= 0 && x >= 0 {
        if *seats.get(y).unwrap().get(x).unwrap() == '#' {
            return true
        }
        if *seats.get(y).unwrap().get(x).unwrap() == 'L' {
            return false
        }
        if y == 0 || x == 0 {
            break;
        }
        x -= 1;
        y -= 1;
    }
    return false
}

fn negative_positive_diagonal_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if col == 0 {
        return false;
    }

    let mut x = col - 1;
    let mut y = row + 1;
    while y < seats.len() && x >= 0 {
        if *seats.get(y).unwrap().get(x).unwrap() == '#' {
            return true;
        }
        if *seats.get(y).unwrap().get(x).unwrap() == 'L' {
            return false;
        }
        if x == 0 {
            break;
        }
        x -= 1;
        y += 1;
    }
    return false
}

fn positive_negative_diagonal_occupied(seats: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if row == 0 {
        return false;
    }

    let mut x = col + 1;
    let mut y = row - 1;
    while y >= 0 && x < seats.get(0).unwrap().len() {
        if *seats.get(y).unwrap().get(x).unwrap() == '#' {
            return true;
        }
        if *seats.get(y).unwrap().get(x).unwrap() == 'L' {
            return false;
        }

        if y == 0 {
            break;
        }
        x += 1;
        y -= 1;
    }
    return false
}

fn apply_rules_two(seats: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();
    for j in 0..seats.len() {
        let row = seats.get(j).unwrap();
        for i in 0..row.len() {
            let seat = row.get(i).unwrap();
            let mut occupied_seats: i8 = 0;
            if column_above_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if column_below_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if row_before_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if row_after_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if positive_positive_diagonal_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if positive_negative_diagonal_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if negative_positive_diagonal_occupied(&seats, j, i) {
                occupied_seats += 1;
            }
            if negative_negative_diagonal_occupied(&seats, j, i) {
                occupied_seats += 1;
            }

            if *seat == 'L' {
                if occupied_seats == 0 {
                    let mut new_row = new_seats.get(j).unwrap().clone();
                    std::mem::replace(&mut new_row[i], '#');
                    std::mem::replace(&mut new_seats[j], new_row);
                }
            } else if *seat == '#' {
                if occupied_seats >= 5 {
                    let mut new_row = new_seats.get(j).unwrap().clone();
                    std::mem::replace(&mut new_row[i], 'L');
                    std::mem::replace(&mut new_seats[j], new_row);
                }
            }
        }
    }
    return new_seats
}

pub fn day_eleven_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut seats: Vec<Vec<char>> = Vec::new();
    for line in lines {
        seats.push(line.chars().collect());
    }

    while true {
        let old_seats = seats.clone();
        seats = apply_rules_two(seats.clone());
        println!("{:?}", count_occupied(seats.clone()));
        if equal(seats.clone(), old_seats) {
            println!("{}", count_occupied(seats.clone()));
            break;
        }
    }
}