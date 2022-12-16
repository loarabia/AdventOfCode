use std::fs::{read_to_string};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{static ref RE_NOOP:Regex = Regex::new(r"noop").unwrap();}
lazy_static!{static ref RE_ADDX:Regex = Regex::new(r"addx (-?\d+)").unwrap();}

#[derive(Debug)]
enum INST {
    NOOP(u32 /* duration */),
    ADDX(u32 /* duration */, i32 /* addend */),
}

fn read_inst(line:&str) -> INST {
    let mut inst = INST::NOOP(1u32);
    if RE_NOOP.is_match(line){
        inst = INST::NOOP(0u32);
    } else if RE_ADDX.is_match(line) {
        let caps = RE_ADDX.captures(line).unwrap();
        let val = caps[1].parse::<i32>().unwrap();
        inst = INST::ADDX(1u32,val);
    }
    inst
}

#[derive(Debug)]
struct INST_CONTEXT {
    inst:INST,
    start_cycle:u32,
    end_cycle:u32,
    val:Option<i32>,
}

impl INST_CONTEXT {
    pub fn new(inst:INST, val:i32, start_cycle:u32) -> INST_CONTEXT {
        match inst {
            INST::NOOP(dur) => {
                INST_CONTEXT { 
                    inst:inst,
                    start_cycle:start_cycle,
                    end_cycle: start_cycle+dur,
                    val:None,
                }
            },
            INST::ADDX(dur, addend) => {
                INST_CONTEXT {
                    inst:inst,
                    start_cycle:start_cycle,
                    end_cycle: start_cycle+dur,
                    val:Some(val+addend),
                }
            },
        }
    }
}

struct CPU {
    inst_queue:Vec<INST_CONTEXT>,
    reg_x:i32,
    cycle:u32,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
            inst_queue:Vec::new(),
            reg_x:1,
            cycle:1,
        }
    }

    pub fn is_processing(&self) -> bool {
        self.inst_queue.len() != 0
    }

    pub fn cycle_num(&self) -> u32 {
        self.cycle
    }

    pub fn step(&mut self) -> u32 {
        let mut to_remove:Vec<usize> = Vec::new();
        for (i,inst) in self.inst_queue.iter().enumerate() {
            if inst.end_cycle == self.cycle {
                if let Some(value) = inst.val {
                    self.reg_x = value;
                }
                to_remove.push(i);
            }
        }

        for index in to_remove {
            self.inst_queue.remove(index);
        }

        self.cycle += 1;

        self.cycle
    }

    fn queue_inst(&mut self, inst:INST) {
        self.inst_queue.push(INST_CONTEXT::new(inst,self.reg_x,self.cycle));
    }

}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    part2(&input);
}

fn part1(input:&String) -> i32 {
    let mut cpu = CPU::new();
    let mut cycle_samples = Vec::new();
    for inst in input.lines().map(|line| read_inst(line)){
        //println!("{:?}",inst);
        cpu.queue_inst(inst);
        while cpu.is_processing() {
            let cycle = cpu.step();
            //println!("cyc: {}, reg: {}",cpu.cycle,cpu.reg_x);
            if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
                cycle_samples.push(cycle as i32 * cpu.reg_x);
            }
        }
    }
    //println!("{:?}", cycle_samples);

    cycle_samples.iter().sum()
}

fn is_in_window(loc:u32, cpu:&CPU) -> bool {
    cpu.reg_x-1 <= loc as i32 && cpu.reg_x+1 >= loc as i32
}

fn part2(input: &String) {
    let mut cpu = CPU::new();
    
    for inst in input.lines().map(|line| read_inst(line)){
        cpu.queue_inst(inst);
        while cpu.is_processing() {
            
            let cycle = cpu.step();
            
            if is_in_window((cycle-1)%40, &cpu) {
                print!("#");       
            } else {
                print!(".");
            }

            if (cycle-1) % 40 == 0 {
                println!("\t {}",cycle);
            }
            
        }
    }

}