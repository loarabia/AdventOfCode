use std::fs::{read_to_string};
use std::str;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn read_stacks(input:&str) -> Vec<Vec<u8>>{

    let mut stack_count = 0;
    let mut stacks:Option<Vec<Vec<u8>>> = None;

    for line in input.lines() {
        
        // initialize
        if stack_count == 0 {
            stack_count = (line.len()+1)/4;
            stacks = Some(init_stacks( (line.len()+1)/4));
        }

        // Fetch a line of data
        let line_stack = read_stack_line(line);
        if line_stack == None {
            break;
        }

        // Merge it in.
        for (i,v) in line_stack.unwrap() {
            match stacks {
                None => {},
                Some(ref mut vec) => { vec[i].push(v)},
            }
        }
    }// FOR

    // Make the stacks the right order
    match stacks {
        None => {},
        Some(ref mut vec) => {
            for i in vec {
                i.reverse();
            }
        }
    }
    // In normal coding this is garbage because Stack init could've failed and be None
    stacks.unwrap()
}

fn init_stacks(cnt:usize) -> Vec<Vec<u8>>{
    let mut stacks:Vec<Vec<u8>> = Vec::new();
    for _i in 0..cnt{
        stacks.push(Vec::new());
    }
    stacks
}

fn read_stack_line(line:&str) -> Option<Vec<(usize, u8)>> {
    let mut stacks:Vec<(usize,u8)> = Vec::new();
    for (i,v) in
    line
        .as_bytes()
        .chunks(4)
        .enumerate()
        {
            // Short circuit and end the line with numbers
            if v[1] == b'1' {
                return None;
            }
            // Handle anything that isn't a space
            // entries are '[_] ' each or '    ' if blank.
            // Final column is 3 u8s because the \n is the 4th char which was already stripped
            if v[1] != b' ' {
                stacks.push((i,v[1]));
            }
        }
    Some(stacks)
}

fn read_instruction(input:&str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();

    let amount:usize = caps[1].parse().unwrap();
    let from:usize = caps[2].parse().unwrap();
    let to:usize = caps[3].parse().unwrap();
    (amount,from,to)
}

fn read_instructions(input:&str) -> Vec<(usize,usize,usize)> {
    let mut insts = Vec::new();
    for mv_inst in input.lines() {
        insts.push(read_instruction(mv_inst));
        }
    insts
}

fn handle_inst_9000(inst:(usize,usize,usize), stacks:&mut Vec<Vec<u8>>) {
    for _i in 0..inst.0 {
        let item = stacks[inst.1-1].pop().unwrap();
        stacks[inst.2-1].push(item);
    }
}

fn handle_inst_9001(inst:(usize /* amt */,usize /* from */,usize /* to */), stacks:&mut Vec<Vec<u8>>) {
    let stack = &mut stacks[inst.1-1];
    let items:Vec<u8> = stack.split_off(stack.len()-inst.0);
    for item in items {
        stacks[inst.2-1].push(item);
    }
}


fn part1(input:&String)->String {
    let mut input_sections = input.split("\r\n\r\n");
    
    let mut stacks = read_stacks(input_sections.next().unwrap());
    let insts = read_instructions(input_sections.next().unwrap());

    for inst in insts {
        handle_inst_9000(inst, &mut stacks);
    }
    
    let result: Vec<u8> = stacks
        .iter()
        .map(|v| v[v.len()-1])
        .collect();
    
    
    String::from_utf8(result).unwrap()
}

fn part2(input:&String)->String{
    let mut input_sections = input.split("\r\n\r\n");
    
    let mut stacks = read_stacks(input_sections.next().unwrap());
    let insts = read_instructions(input_sections.next().unwrap());

    for inst in insts {
        handle_inst_9001(inst, &mut stacks);
    }
    
    let result: Vec<u8> = stacks
        .iter()
        .map(|v| v[v.len()-1])
        .collect();
    
    
    String::from_utf8(result).unwrap()
}
