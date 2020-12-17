use std::collections::HashMap;

fn cycle(c: HashMap<(i16, i16, i16), bool>) -> HashMap<(i16, i16, i16), bool> {
    let mut cubes = c.clone();
    for key in cubes.clone().keys() {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x == 1 && y == 1 && z == 1 {
                        continue;
                    }
                    let neighbour_key = (key.0 + x - 1, key.1 + y - 1, key.2 + z - 1);
                    match cubes.get(&neighbour_key) {
                        Some(active) => {},
                        None => {
                            cubes.insert(neighbour_key, false);
                        }
                    };
                }
            }
        }
    }

    let mut new_cubes = cubes.clone();
    for key in cubes.clone().keys() {
        let mut active_neighbours = 0;
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x == 1 && y == 1 && z == 1 {
                        continue;
                    }
                    let is_neighbour_active = match cubes.get(&(key.0 + x - 1, key.1 + y - 1, key.2 + z - 1)) {
                        Some(active) => {
                            *active
                        },
                        None => {
                            false
                        }
                    };
                    if is_neighbour_active {
                        active_neighbours += 1;
                    }
                }
            }
        }
        let is_cube_active = match cubes.get(key) {
            Some(active) => {
                *active
            },
            None => {
                panic!("maps are broken");
            }
        };

        if is_cube_active && !(active_neighbours == 2 || active_neighbours == 3) {
            new_cubes.insert(*key, false);
        } else if !is_cube_active && active_neighbours == 3 {
            new_cubes.insert(*key, true);
        }
    }
    return new_cubes;
}

pub fn day_seventeen_problem_one(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut cubes: HashMap<(i16, i16, i16), bool> = HashMap::new();

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    cubes.insert((x, y, 0), true);
                },
                '.' => {
                    cubes.insert((x, y, 0), false);
                },
                _ => {},
            };
            x += 1;
        }
        y += 1;
    }
    
    for _ in 0..6 {
        cubes = cycle(cubes.clone());
    }

    let mut total_active_cubes = 0;
    for active in cubes.values() {
        if *active {
            total_active_cubes += 1;
        }
    }
    println!("{}", total_active_cubes);
}

fn hyper_cycle(h: HashMap<(i16, i16, i16, i16), bool>) -> HashMap<(i16, i16, i16, i16), bool> {
    let mut hyper_cubes = h.clone();
    for key in hyper_cubes.clone().keys() {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    for a in 0..3 {
                        if x == 1 && y == 1 && z == 1 && a == 1 {
                            continue;
                        }
                        let neighbour_key = (key.0 + x - 1, key.1 + y - 1, key.2 + z - 1, key.3 + a - 1);
                        match hyper_cubes.get(&neighbour_key) {
                            Some(active) => {},
                            None => {
                                hyper_cubes.insert(neighbour_key, false);
                            }
                        };
                    }
                }
            }
        }
    }

    let mut new_hyper_cubes = hyper_cubes.clone();
    for key in hyper_cubes.clone().keys() {
        let mut active_neighbours = 0;
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    for a in 0..3 {
                        if x == 1 && y == 1 && z == 1 && a == 1 {
                            continue;
                        }
                        let is_neighbour_active = match hyper_cubes.get(&(key.0 + x - 1, key.1 + y - 1, key.2 + z - 1, key.3 + a - 1)) {
                            Some(active) => {
                                *active
                            },
                            None => {
                                false
                            }
                        };
                        if is_neighbour_active {
                            active_neighbours += 1;
                        }
                    }
                }
            }
        }
        let is_hyper_cube_active = match hyper_cubes.get(key) {
            Some(active) => {
                *active
            },
            None => {
                panic!("maps are broken");
            }
        };

        if is_hyper_cube_active && !(active_neighbours == 2 || active_neighbours == 3) {
            new_hyper_cubes.insert(*key, false);
        } else if !is_hyper_cube_active && active_neighbours == 3 {
            new_hyper_cubes.insert(*key, true);
        }
    }
    return new_hyper_cubes;
}

pub fn day_seventeen_problem_two(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut hyper_cubes: HashMap<(i16, i16, i16, i16), bool> = HashMap::new();

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    hyper_cubes.insert((x, y, 0, 0), true);
                },
                '.' => {
                    hyper_cubes.insert((x, y, 0, 0), false);
                },
                _ => {},
            };
            x += 1;
        }
        y += 1;
    }
    
    for _ in 0..6 {
        hyper_cubes = hyper_cycle(hyper_cubes.clone());
    }

    let mut total_active_hyper_cubes = 0;
    for active in hyper_cubes.values() {
        if *active {
            total_active_hyper_cubes += 1;
        }
    }
    println!("{}", total_active_hyper_cubes);
}
