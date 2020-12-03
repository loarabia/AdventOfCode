use std::fs::{read_to_string};
//use std::str::FromStr;
use std::option::Option;

const WIDTH:usize = 31;
const HEIGHT:usize = 323;

type ForestT = [[bool; WIDTH]; HEIGHT];

struct Forest {
    curr_x:usize,
    curr_y:usize,
    x_slope:usize,
    y_slope:usize,
    forest:ForestT,
}

impl Forest {
    fn new(forest:ForestT,x_slope:usize,y_slope:usize) -> Forest {
        Forest{ curr_x:0,
                curr_y:0,
                x_slope:x_slope,
                y_slope:y_slope,
                forest:forest,
            }
    }
}

impl Iterator for Forest {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut next_x = self.curr_x + self.x_slope; 
        let next_y = self.curr_y + self.y_slope;
        
        // At the end.
        if next_y >= HEIGHT {
            return None;
        }

        // Off the Right edge 
        if next_x >= WIDTH {
            next_x = next_x - WIDTH;
        }

        self.curr_x = next_x;
        self.curr_y = next_y;
        let val = self.forest[self.curr_y][self.curr_x];
        if val {
            Some(1) // has a Tree
        } else {
            Some(0) // Empty Space
        }
    }
}

fn part1(forest:Forest) -> Option<u32> {
    Some(forest.sum())
}

fn part2(forest:ForestT) -> Option<u32> {
    // Right 1, down 1.
    let v1:u32 = Forest::new(forest,1,1).sum();
    // Right 3, down 1. (This is the slope you already checked.)
    let v2:u32 = Forest::new(forest,3,1).sum();
    // Right 5, down 1.
    let v3:u32 = Forest::new(forest,5,1).sum();
    // Right 7, down 1.
    let v4:u32 = Forest::new(forest,7,1).sum();
    // Right 1, down 2.
    let v5:u32 = Forest::new(forest,1,2).sum();
    Some(v1*v2*v3*v4*v5)
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    
    let mut forest = [[false; WIDTH]; HEIGHT];
    for (y,line) in input.lines().enumerate() {
        for (x,val) in line.as_bytes().iter().enumerate() {
            forest[y][x] = match *val as char {
                '.' => false,
                '#' => true,
                _ => panic!("What letter {}", *val as char),
            }
        }
    }
    println!("{}",part1(Forest::new(forest,3,1)).unwrap());   
    println!("{}",part2(forest).unwrap());
}
