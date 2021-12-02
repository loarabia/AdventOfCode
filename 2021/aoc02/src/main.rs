#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::fs::{read_to_string};
use std::str::FromStr;

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Instruction {

    type Err = regex::Error;  

    fn from_str(inst_line:&str) -> Result<Self, Self::Err> {

        lazy_static!{
            static ref RE:Regex = Regex::new(r"(forward|down|up) ([0-9]+)").unwrap();
        }
        
        let captures = RE.captures(inst_line).unwrap();
        
        let instruction;
        let num = i32::from_str(captures.get(2).unwrap().as_str()).unwrap();

        instruction = match captures.get(1).unwrap().as_str(){
            "forward" => Instruction::Forward(num),
            "down" => Instruction::Down(num),
            "up" => Instruction::Up(num),
            _ => panic!("BAD INPUT FROM AOC2021?"),
        };

        Ok(instruction)
    }
}

struct Sub
{
    x:i32,
    depth:i32,
    aim:i32,
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut sub = Sub{x:0,depth:0,aim:0};
    for inst in instructions {
        match inst {
            Instruction::Forward(dist) => sub.x = sub.x + dist,
            Instruction::Up(dist) => sub.depth = sub.depth - dist,
            Instruction::Down(dist) => sub.depth = sub.depth + dist,
        }
    }
    sub.x*sub.depth
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    let mut sub = Sub{x:0,depth:0,aim:0};
    for inst in instructions {
        match inst {
            Instruction::Forward(dist) => { 
                sub.x = sub.x + dist;
                sub.depth = sub.depth + (dist * sub.aim);
            },
            Instruction::Up(dist) => sub.aim = sub.aim - dist,
            Instruction::Down(dist) => sub.aim = sub.aim + dist,
        }
    }
    sub.x*sub.depth
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let instructions: Vec<Instruction> = 
    input
        .lines()
        .map(|line| Instruction::from_str(line).expect("Bad line item"))
        .collect();
    println!("{}",part1(&instructions));
    println!("{}",part2(&instructions));
}