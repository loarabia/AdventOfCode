use core::panic;
use std::collections::HashMap;
use std::fs::read_to_string;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_MOVE: Regex = Regex::new(r"([RDLU]) (\d+)").unwrap();
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn read_direction(line: &str) -> DIRECTION {
    if RE_MOVE.is_match(line) {
        let caps = RE_MOVE.captures(line).unwrap();
        let amount = caps[2].parse::<i32>().unwrap();
        let direction: &str = &caps[1];

        match direction {
            "R" => DIRECTION::RIGHT(amount),
            "L" => DIRECTION::LEFT(amount),
            "D" => DIRECTION::DOWN(amount),
            "U" => DIRECTION::UP(amount),
            _ => panic!("UNKNOWN DIRECTION {}", direction),
        }
    } else {
        panic!("UNKNOWN INSTRUCTIONS {}", line);
    }
}

#[derive(Debug)]
enum DIRECTION {
    RIGHT(i32),
    DOWN(i32),
    LEFT(i32),
    UP(i32),
}

fn get_new_head_and_tail(
    head: &(i32, i32),
    tail: &(i32, i32),
    step: &(i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let new_head = (head.0 + step.0, head.1 + step.1);
    let new_tail = get_new_tail_loc(&new_head, tail);

    (new_head, new_tail)
}

fn get_new_tail_loc(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let mut new_tail = *tail;

    if (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 1 {
        // Large Knight's move
        new_tail.0 += (head.0 - tail.0) / 2;
        new_tail.1 += head.1 - tail.1;
    } else if (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 2 {
        // Large Knight's move
        new_tail.0 += head.0 - tail.0;
        new_tail.1 += (head.1 - tail.1) / 2;
    } else if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
        // Diagonals and vertical or horizontal moves
        new_tail.0 += (head.0 - tail.0) / 2;
        new_tail.1 += (head.1 - tail.1) / 2;
    }
    new_tail
}

fn update_chain(chain: &mut Vec<(i32, i32)>, first_step: &(i32, i32)) {
    for i in 1..chain.len() {
        let head = chain[i - 1];
        let tail = chain[i];

        let old_tail = tail.clone();

        if i == 1 {
            //println!(" Updating Head ");
            let (new_head, new_tail) = get_new_head_and_tail(&head, &tail, first_step);
            chain[i - 1] = new_head;
            chain[i] = new_tail;
            //print_rope(chain);
            //println!();
        } else {
            //println!(" Updating Tail Parts {}-{}",i-1,i );
            let new_tail = get_new_tail_loc(&head, &old_tail);
            if new_tail == old_tail {
                break;
            }
            chain[i] = new_tail;
            //print_rope(chain);
            //println!();
        }
    }
}

fn lerp(d_min: i32, val: i32) -> usize {
    let offset = 0 - d_min;
    assert!(val + offset >= 0);
    (val + offset) as usize
}

fn draw_rope(rope: &Vec<(i32, i32)>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut grid: Vec<Vec<usize>> = Vec::new();
    for (x, y) in rope {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    for _y in min_y..max_y + 1 {
        let mut row: Vec<usize> = Vec::new();
        for _y in min_x..max_x + 1 {
            row.push(0);
        }
        grid.push(row);
    }

    // println!("X -- min {}  max {}", min_x, max_x);
    // println!("Y -- min {}  max {}", min_y, max_y);
    // println!("{}", grid.len());

    for (i, (x, y)) in rope.iter().enumerate() {
        grid[lerp(min_y, *y)][lerp(min_x, *x)] = i + 1;
    }

    for row in grid {
        for cell in row {
            if cell == 0 {
                print!(".");
            } else if cell == 1 {
                print!("H");
            } else if cell == 10 {
                print!("T");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn print_rope(rope: &Vec<(i32, i32)>) {
    for (i, seg) in rope.iter().enumerate() {
        if i == 0 {
            print!("Head {:?}", seg);
        } else if i == rope.len() - 1 {
            print!(" {:?} Tail", seg);
        } else {
            print!(" {:?} ", seg);
        }
    }
}

fn part1(input: &String) -> usize {
    let mut past_tail_locs: HashMap<(i32, i32), i32> = HashMap::new();

    let mut head_loc: (i32, i32) = (0, 0);
    let mut tail_loc: (i32, i32) = (0, 0);
    past_tail_locs.insert(tail_loc, 1);

    for command in input.lines().map(|line| read_direction(line)) {
        //println!("{:?}",command);
        let step = match command {
            DIRECTION::DOWN(dist) => ((0, -1), dist),
            DIRECTION::UP(dist) => ((0, 1), dist),
            DIRECTION::LEFT(dist) => ((-1, 0), dist),
            DIRECTION::RIGHT(dist) => ((1, 0), dist),
        };
        for _i in 0..step.1 {
            let new_head_and_tail = get_new_head_and_tail(&head_loc, &tail_loc, &step.0);
            head_loc = new_head_and_tail.0;
            tail_loc = new_head_and_tail.1;
            match past_tail_locs.get(&tail_loc) {
                Some(cnt) => past_tail_locs.insert(tail_loc, cnt + 1),
                None => past_tail_locs.insert(tail_loc, 1),
            };
        }
    }
    past_tail_locs.len()
}

fn part2(input: &String) -> usize {
    let mut past_tail_locs: HashMap<(i32, i32), i32> = HashMap::new();

    let mut rope = Vec::new();
    for _i in 0..10 {
        rope.push((0, 0));
    }

    // let mut head_loc:(i32,i32) = (0,0);
    let tail_loc: (i32, i32) = (0, 0);
    past_tail_locs.insert(tail_loc, 1);

    for command in input.lines().map(|line| read_direction(line)) {
        // println!("{:?}", command);
        let step = match command {
            DIRECTION::DOWN(dist) => ((0, -1), dist),
            DIRECTION::UP(dist) => ((0, 1), dist),
            DIRECTION::LEFT(dist) => ((-1, 0), dist),
            DIRECTION::RIGHT(dist) => ((1, 0), dist),
        };

        for _i in 0..step.1 {
            update_chain(&mut rope, &step.0);

            // print_rope(&rope);
            // println!("{:?}", rope[0]);

            match past_tail_locs.get(&rope[9]) {
                Some(cnt) => past_tail_locs.insert(rope[9], cnt + 1),
                None => past_tail_locs.insert(rope[9], 1),
            };
        }

        // println!("==========================================");
        // draw_rope(&rope);
        // println!("{}", past_tail_locs.len());
    } // For

    past_tail_locs.len()
}
