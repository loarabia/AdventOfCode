use std::fs;
use std::str::FromStr;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("File access error");

    let tape:Vec<usize> = contents.split(',')
        .map(|num| usize::from_str(num).expect("Bad line item") )
        .collect();

    run_part1(&tape);
    run_part2(&tape);
}

fn run_part1(tape:&Vec<usize>) {
    let noun = 12;
    let verb = 2;
    let mut computer = Computer::init(noun, verb, tape);
    computer.run();
    println!("{}", computer.result());
}

fn run_part2(tape:&Vec<usize>) {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::init(noun, verb, tape);
            computer.run();

            if computer.result() == 19690720 {
                // Echo Base this is Rogue 2. I've found them, repeat I've found them!
                println!("{}", noun * 100 + verb);
            }
        }
    }
}

struct Computer {
    inst_ptr:usize,
    running:bool,
    memory:Vec<usize>,
}

impl Computer {
    fn init(noun:usize, verb:usize, tape:&Vec<usize>) -> Computer {
        let mut c = Computer {
            inst_ptr: 0,
            running: false,
            memory: tape.clone(),
        };
        c.memory[1] = noun;
        c.memory[2] = verb;
        c
    }

    fn step(&mut self) {
        self.inst_ptr = self.inst_ptr + 4;
    }

    fn halt(&mut self) {
        self.running = false;
    }

    fn result (& self) -> usize {
        self.memory[0]
    }

    fn run(&mut self) {
        self.running = true;
        while self.running {
            let op = self.memory[self.inst_ptr];
            match op {
                // ADD
                1 => {
                    let addr1 = self.memory[self.inst_ptr + 1];
                    let addr2 = self.memory[self.inst_ptr + 2];
                    let addr3 = self.memory[self.inst_ptr + 3];

                    let i1 = self.memory[addr1];
                    let i2 = self.memory[addr2];

                    self.memory[addr3] = i1 + i2;
                    self.step();
                },
                // MUL
                2 => {
                    let addr1 = self.memory[self.inst_ptr + 1];
                    let addr2 = self.memory[self.inst_ptr + 2];
                    let addr3 = self.memory[self.inst_ptr + 3];

                    let i1 = self.memory[addr1];
                    let i2 = self.memory[addr2];
                    
                    self.memory[addr3] = i1 * i2;
                    self.step();
                },
                // HALT
                99 => { self.halt(); },
                // OTHER OPCODES
                _ => { },
            }
        }

    }
}