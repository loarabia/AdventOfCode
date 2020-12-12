use std::cmp::min;
use std::fs::read_to_string;

// const file:&str = "input2.txt";
// const HEIGHT:usize = 10;
// const WIDTH:usize = 10;

const file:&str = "input.txt";
const HEIGHT:usize = 93;
const WIDTH:usize = 94;

type SeatsT = [[Option<bool>; WIDTH]; HEIGHT];

fn print_map(seats:&SeatsT) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match seats[y][x] {
                Some(true) => print!("#"),
                Some(false) => print!("L"),
                None => print!("."),
            }
        }
        print!("\n");
    }
}

fn number_of_occupied_adjacent_seats(x:usize, y:usize, seats:&SeatsT) -> u32 {
    let left = x.saturating_sub(1);
    let top = y.saturating_sub(1);
    let right = min(x+1,WIDTH-1);
    let bot = min(y+1,HEIGHT-1);

    // println!("\n\n=================");
    // println!("{} {} {}", left, x, right);
    // println!("{} {} {}", top, y, bot);

    let mut count = 0;

    for yy in top..=bot {
        for xx in left..=right {
            if x == xx && y == yy {
                continue;
            }
            // println!("{},{}", xx, yy);
            match seats[yy][xx] {
                Some(true) => {count +=1;},
                _ => {},
            }
        }
    }

    count
}

fn run_rules(seats:&SeatsT) -> Vec<(usize,usize, Option<bool>)> {
    let mut changes = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = number_of_occupied_adjacent_seats(x, y, seats);
            match seats[y][x] {
                Some(true) => { // Occupied seat
                    if neighbors >= 4 {
                        changes.push((x,y,Some(false)));
                    }
                },
                Some(false) => { // Empty seat
                    if neighbors == 0 { 
                        changes.push((x,y,Some(true)));
                    }
                },
                None => {},
            }
        }
    }
    changes
}

// 2170 TOO LOW
// 7252 TOO HIGH
fn part1(seats:&mut SeatsT) -> Option<u32> {

    loop {
        // println!("\n ===== BEFORE =====\n\n");
        // print_map(seats);
        // Gather Diffs
        let changes = run_rules(seats);
        
        // done?
        if changes.is_empty() { break; }
        // println!("Num Changes: {}", changes.len());

        // Commit diffs and run again
        for (x,y,val) in changes {
            seats[y][x] = val;
        }

        // println!("\n ===== AFTER ===== \n\n");
        // print_map(seats);
    }

    let mut count = 0;

    for y in 0..HEIGHT {
        for x in 0.. WIDTH {
            if seats[y][x] == Some(true) {
                count += 1;
            }
        }
    }
    Some(count)
}

fn walk(start_x:usize, start_y:usize, dx:isize, dy:isize, seats: &SeatsT) -> Option<u32> {
    let mut x = start_x as isize + dx;
    let mut y = start_y as isize + dy;
    
    let mut seat = None;
    
    loop {
        // Don't go off the edges
        if x < 0 || y < 0 || y >= HEIGHT as isize || x >= WIDTH as isize {
            break;
        }

        match seats[y as usize][x as usize] {
            None => { seat = None },
            Some(true) => { seat = Some(1); break;},
            Some(false) => { seat = Some(0); break; },
        }

        x += dx;
        y += dy;
    }
    seat
}

fn number_of_occupied_visible_seats(x:usize, y:usize, seats:&SeatsT) -> u32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            match walk(x,y,dx,dy,seats) {
                Some(val) => count += val,
                _ => {},
            }
        }
    }
    count
}

fn run_new_rules(seats:&SeatsT) -> Vec<(usize,usize,Option<bool>)> {
    let mut changes = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = number_of_occupied_visible_seats(x, y, seats);
            match seats[y][x] {
                Some(true) => { // Occupied seat
                    if neighbors >= 5 {
                        changes.push((x,y,Some(false)));
                    }
                },
                Some(false) => { // Empty seat
                    if neighbors == 0 { 
                        changes.push((x,y,Some(true)));
                    }
                },
                None => {},
            }
        }
    }
    changes
}

fn part2(seats:&mut SeatsT) -> Option<u32> {

    loop {
        // println!("\n ===== BEFORE =====\n\n");
        // print_map(seats);
        // Gather Diffs
        let changes = run_new_rules(&seats);
        
        // done?
        if changes.is_empty() { break; }
        // println!("Num Changes: {}", changes.len());

        // Commit diffs and run again
        for (x,y,val) in changes {
            seats[y][x] = val;
        }

        // println!("\n ===== AFTER ===== \n\n");
        // print_map(seats);
    }

    let mut count = 0;

    for y in 0..HEIGHT {
        for x in 0.. WIDTH {
            if seats[y][x] == Some(true) {
                count += 1;
            }
        }
    }
    Some(count)
}

fn main() {
    let input = read_to_string(file).expect("Couldn't read input.txt");
    
    let mut seats = [[None; WIDTH]; HEIGHT];

    for (y,line) in input.lines().enumerate() {
        for (x,val) in line.as_bytes().iter().enumerate() {
            seats[y][x] = match *val as char {
                'L' => Some(false),
                '#' => Some(true),
                '.' => None,
                _ => panic!("What letter {}", *val as char),
            }
        }
    }
    println!("{}",part1(&mut seats.clone()).unwrap());   
    println!("{}",part2(&mut seats.clone()).unwrap());
}
