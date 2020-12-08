use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Eq)]
enum State {
    Error,
    Normal,
}

struct Computer<'a> {
    inst_ptr:usize,
    reg_acc:i32,
    memory:Vec<&'a str>,
    decoder:Regex,
    state:State,
    flip_op_addr:Option<usize>,
}

impl <'a> Computer<'a> {

    fn new(program:&str) -> Computer {
        Computer {
            inst_ptr:0,
            reg_acc:0,
            memory:program.lines().collect(),
            decoder:Regex::new(r"(?P<inst>\w{3}) (?P<num>[+-]\d+)").unwrap(),
            state:State::Normal,
            flip_op_addr:None,
        }
    }

    fn reset(&mut self) {
        self.inst_ptr = 0;
        self.reg_acc = 0;
        self.state = State::Normal;
        self.flip_op_addr=None;
    }

    // 113196 TOO HIGH! <- forgot to call reset !!!
    fn run_fix_halting(&mut self) -> Option<u32> {
        let (_val, set) = match self.run_halting() {
            Ok(val) => { (val, HashSet::new()) },
            Err((val, set)) => (val,set),
        };

        for addr in set.iter() {
            self.flip_op_addr = Some(*addr);
            match self.run_halting() {
                Ok(val) => { return Some(val); },
                Err(_) => {},
            } //match
            self.reset();
        } //for

        None
    } //fn

    fn run_halting(&mut self ) -> Result<u32, (u32, HashSet<usize>)> {
        let mut visited_addr = HashSet::new();
        //println!("RUNNING =================================");
        loop {
            // Is the instruction we're about to fetch one we've seen? If so, stop the madness HALT!!
            // If you couldn't insert the value into the set, you've seen it.
            if  !visited_addr.insert(self.inst_ptr) {
                self.state = State::Error;
                break;
            }

            // Are you about to execute off the end?
            if self.inst_ptr >= self.memory.len() {
                self.state = State::Normal;
                break;
            }

            // Fetch Instruction
            let inst:&str = self.memory[self.inst_ptr];

            // Decode it
            let captures = self.decoder.captures(inst).unwrap();
            let mut name = captures.name("inst").unwrap().as_str();
            let num = i32::from_str(captures.name("num").unwrap().as_str()).unwrap();
            //println!("Op: {}, val: {}, ip:{}, acc:{} ", name, num, self.inst_ptr, self.reg_acc);

            // Fix it Up
            if self.flip_op_addr == Some(self.inst_ptr) {
                name = match name {
                    "jmp" => "nop", //flip
                    "nop" => "jmp", //flip
                    "acc" => "acc", 
                    _ => { panic!("Unknown op:{} with value: {}", name, num)}, 

                }
            }

            // Execute it
            match name {
                "nop" => {self.execute_nop()},
                "jmp" => {self.execute_jmp(num)},
                "acc" => {self.execute_acc(num)},
                _ => { panic!("Unknown op:{} with value: {}", name, num)}, 
            }

        }

        if self.state == State::Error {
            Err(( self.reg_acc as u32, visited_addr))
        } else {
            Ok( self.reg_acc as u32 )
        }
    }

    fn execute_nop(&mut self) {
        self.inst_ptr += 1;
    }
    
    fn execute_acc(&mut self, num:i32) {
        self.reg_acc += num;
        self.inst_ptr += 1;
    }

    fn execute_jmp(&mut self, num:i32) {
        self.inst_ptr = (self.inst_ptr as i32 + num ) as usize;
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    println!("{}",part1(&input).unwrap());
    println!("{}",part2(&input));

}

fn part1(input:&str) -> Option<u32> {
    let mut computer = Computer::new(input);
    match computer.run_halting() {
        Ok(val) => Some(val),
        Err( (val, _set)) => Some(val) ,
    }
}

fn part2(input:&str) -> u32 {
    let mut computer = Computer::new(input);
    computer.run_fix_halting().unwrap()
}