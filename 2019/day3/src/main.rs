use std::fmt;
use std::fs;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use std::collections::HashSet;

use regex::{Regex, Captures};

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("File access error");

    let wires:Vec<&str> = contents.split('\n').collect();

    let wire1_def:Vec<&str> = wires[0].split(',').collect();
    let wire2_def:Vec<&str> = wires[1].split(',').collect();

    let set1:HashSet<Point> = read_wire(wire1_def);
    let set2:HashSet<Point> = read_wire(wire2_def);

    run_part1(&set1, &set2);
    run_part2(&set1, &set2);
}

fn run_part1(wire1:&HashSet<Point>, wire2:&HashSet<Point>) {
    let part1 = wire1.intersection(wire2)
        .min_by_key(|pt| pt.city_distance())
        .unwrap();
    println!("pt1: {}", part1.city_distance());
}

fn run_part2(wire1:&HashSet<Point>, wire2:&HashSet<Point>){
    let point = wire1.intersection(wire2)
        .min_by_key(|pt| {
            let w1pt = wire1.get(pt).unwrap();
            let w2pt = wire2.get(pt).unwrap();
            w1pt.dist + w2pt.dist
        })
        .unwrap();
    println!("pt2: {}", wire1.get(point).unwrap().dist + wire2.get(point).unwrap().dist);
}

fn read_wire(wire_def:Vec<&str>) -> HashSet<Point> {
    let mut result = HashSet::new();
    let regex_set:Vec<Regex> = vec![
        Regex::new(r"(U)([0-9]+)").unwrap(),
        Regex::new(r"(D)([0-9]+)").unwrap(),
        Regex::new(r"(R)([0-9]+)").unwrap(),
        Regex::new(r"(L)([0-9]+)").unwrap(),
    ];

    let mut current:Point = Point{x:0,y:0, dist:0};
    for def in wire_def {

        // Assumption: should only match one given the leading character.
        // Capture Group 0 is the whole text.
        // Capture Group 1 is the letter. 
        // Capture Group 2 is an integer.
        if let Some(captures) = regex_set[0].captures(def) {
            // Up
            let length = unpack_length(captures);
            build_list(&mut result, &mut current, length, |pt| pt.y+=1);

        } else if let Some(captures) = regex_set[1].captures(def) {
            // Down
            let length = unpack_length(captures);
            build_list(&mut result, &mut current, length, |pt| pt.y-=1);

        } else if let Some(captures) = regex_set[2].captures(def) {
            // Right
            let length = unpack_length(captures);
            build_list(&mut result, &mut current, length, |pt| pt.x+=1);

        } else if let Some(captures) = regex_set[3].captures(def) {
            // Left
            let length = unpack_length(captures);
            build_list(&mut result, &mut current, length, |pt| pt.x-=1);
        } else {
            panic!("HOW?!?");
        }
    }
    result
}

fn unpack_length(cap:Captures<'_>) -> i32 {
    //let length = cap.get(2).map_or(-1, |m| i32::from_str(m.as_str()).unwrap() );
    // Same as below but better error handling. Not really more readable in thie case.
    i32::from_str(cap.get(2).unwrap().as_str()).unwrap()
}

fn build_list<F> (
    set:&mut HashSet<Point>, 
    start:&mut Point,
    length:i32,
    func:F ) where F: Fn(&mut Point) 
{
        for _ in 0..length {
            func(start);
            start.dist += 1;
            set.insert(start.clone());
        }
}

#[derive(Clone)]
struct Point {
    x:i32,
    y:i32,
    dist:i32,
}

impl Point {
    fn city_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point{
    fn hash<H:Hasher>(&self, state: &mut H){
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {},  {})", self.x, self.y, self.dist)
    }
}