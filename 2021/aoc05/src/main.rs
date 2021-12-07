extern crate once_cell;
use once_cell::sync::Lazy;

extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::fs::{read_to_string};
use std::mem::{swap};
use std::str::FromStr;


#[derive(Debug,Hash,Eq,PartialEq)]
struct Point {
    x:i32,
    y:i32,
}

#[derive(Debug)]
struct Pair {
    beg:Point,
    end:Point,
}

impl Pair {
    fn read_pair(line:&str) -> Pair {
        static RE:Lazy<Regex> =Lazy::new(|| Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap());
        let caps = RE.captures(line).unwrap();

        let mut x1 = i32::from_str(caps.get(1).unwrap().as_str()).unwrap();
        let mut y1 = i32::from_str(caps.get(2).unwrap().as_str()).unwrap();
        let mut x2 = i32::from_str(caps.get(3).unwrap().as_str()).unwrap();
        let mut y2 = i32::from_str(caps.get(4).unwrap().as_str()).unwrap();

        // Ensure that the left most and top most point is beg.
        if x1 > x2 { 
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }
        if y1 > y2 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }

        Pair { 
            beg:Point{x:x1,y:y1}, 
            end:Point{x:x2,y:y2}
        }
    }

    // Part1
    fn points_hv(&self) -> Vec<Point> {
        let mut pts:Vec<Point> = Vec::new();
        if self.beg.x == self.end.x {
            // Vertical Line
            let x = self.beg.x;
            for y in self.beg.y..=self.end.y {
                pts.push(Point{x,y});
            }
        } else if self.beg.y == self.end.y {
            // Horizontal Lines
            let y = self.beg.y;
            for x in self.beg.x..=self.end.x {
                pts.push(Point{x,y});
            }
        } else {
            // 45 Degree  lines ... 
        }
        pts
    }

    // Part 2
    fn points(&self) -> Vec<Point> {
        let mut pts:Vec<Point> = Vec::new();
        if self.beg.x == self.end.x {
            // Vertical Line
            let x = self.beg.x;
            for y in self.beg.y..=self.end.y {
                pts.push(Point{x,y});
            }
        } else if self.beg.y == self.end.y {
            // Horizontal Lines
            let y = self.beg.y;
            for x in self.beg.x..=self.end.x {
                pts.push(Point{x,y});
            }
        } else {
            // 45 Degree  lines ... 
            let x_step:i32;
            let y_step:i32;
            if self.beg.x < self.end.x { x_step = 1 } else { x_step = -1 }
            if self.beg.y < self.end.y { y_step = 1 } else { y_step = -1 }
            
            let mut i = 0;
            loop {
                let new_x = self.beg.x + i * x_step;
                let new_y = self.beg.y + i * y_step;
                i+=1;
                pts.push(Point{x:new_x,y:new_y});
                if(new_x == self.end.x) && new_y == self.end.y{
                    break;
                }
            }
        }
        pts
    }
}

fn part1(lines:&Vec<Pair>) -> u32 {
    
    let mut grid:HashMap<Point,u32> = HashMap::new();

    // Form all Points
    for point in lines.iter().map(|item| item.points_hv()).flatten() {
        if let Some(val) = grid.get_mut(&point) {
            *val += 1;
        } else {
            grid.insert(point,1);
        }
    }
    grid.into_values().filter(|val| val > &1).count() as u32
}

fn part2(lines:&Vec<Pair>) -> u32 {
    
    let mut grid:HashMap<Point,u32> = HashMap::new();

    // Form all Points
    for point in lines.iter().map(|item| item.points()).flatten() {
        if let Some(val) = grid.get_mut(&point) {
            *val += 1;
        } else {
            grid.insert(point,1);
        }
    }
    grid.into_values().filter(|val| val > &1).count() as u32
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let pairs:Vec<Pair> = input.lines().map(|line| Pair::read_pair(line)).collect();
    println!("{}", part1(&pairs));
    println!("{}", part2(&pairs));

}
