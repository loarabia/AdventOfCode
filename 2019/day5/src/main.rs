use std::io;
use std::fs;
use std::str::FromStr;


fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("File access error");

    let tape:Vec<i32> = contents.split(',')
        .map(|num| i32::from_str(num).expect("Bad line item") )
        .collect();

    run_part1(&tape);
}

fn run_part1(tape:&Vec<i32>) {
    let mut computer = Computer::init(tape);
    computer.run();
}

enum Mode {
    Immediate,
    Position,
}

enum Instruction {
    HALT, //99
    Add(Mode, Mode, Mode), //1
    Mul(Mode, Mode, Mode), //2
    ReadInput(Mode), //3
    WriteOutput(Mode), //4
    JumpIfTrue(Mode, Mode), //5
    JumpIfFalse(Mode, Mode), //6
    LT(Mode, Mode, Mode), //7
    EQ(Mode, Mode, Mode), //8
}

struct Computer {
    inst_ptr:usize,
    running:bool,
    memory:Vec<i32>,
}

impl Computer {
    fn init( tape:&Vec<i32>) -> Computer {
        let c = Computer {
            inst_ptr: 0,
            running: false,
            memory: tape.clone(),
        };
        c
    }

    fn fetch(& self) -> i32 {
        self.memory[self.inst_ptr]
    }

    fn decode(& self, opcode:i32) -> Instruction {
        match read_opcode(&opcode) {
            1 => {
                Instruction::Add(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            2 => {
                Instruction::Mul(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            3 => { Instruction::ReadInput(read_mode(&opcode,&2))},
            4 => { Instruction::WriteOutput(read_mode(&opcode, &2))},
            5 => { Instruction::JumpIfTrue(
                    read_mode(&opcode, &2),
                    read_mode(&opcode, &3))},
            6 => { 
                Instruction::JumpIfFalse(
                    read_mode(&opcode, &2),
                    read_mode(&opcode, &3))},
            7 => { 
                Instruction::LT(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            8 => { Instruction::EQ(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            99 => { Instruction::HALT },
            _ => { 
                println!("opcode:{}", opcode);
                println!("inst_ptr:{}", self.inst_ptr);
                // println!("outputting memory");
                // for i in 0..self.inst_ptr + 5 {
                //     println!("{}", self.memory[i]);
                // }
                panic!("UNKNOWN OPCODE")
            }
        }
    }

    fn run(&mut self) {
        self.running = true;
        while self.running {
            match self.decode(self.fetch()) {
                Instruction::HALT => {self.execute_halt()},
                Instruction::Add(m1,m2,m3) => {self.execute_add(m1,m2,m3)},
                Instruction::Mul(m1,m2,m3) => {self.execute_mul(m1, m2, m3)},
                Instruction::ReadInput(m1) => {self.execute_read_input(m1)},
                Instruction::WriteOutput(m1) => { self.execute_write_output(m1)},
                Instruction::JumpIfTrue(m1,m2) => { self.execute_jmp_true(m1,m2)},
                Instruction::JumpIfFalse(m1,m2) => { self.execute_jmp_false(m1,m2)},
                Instruction::LT(m1,m2,m3) => { self.execute_lt(m1,m2,m3)},
                Instruction::EQ(m1,m2,m3) => { self.execute_eq(m1,m2,m3)},
            }
        }
    }

    fn execute_add(&mut self, mode1:Mode, mode2:Mode, mode3:Mode) 
    {
        let in_addr1;
        match mode1 {
            Mode::Immediate => in_addr1 = self.inst_ptr + 1,
            Mode::Position => in_addr1 = self.memory[ self.inst_ptr +1] as usize,
        };
        let in_operand1 = self.memory[in_addr1];

        let in_addr2;
        match mode2 {
            Mode::Immediate => in_addr2 = self.inst_ptr + 2,
            Mode::Position => in_addr2 = self.memory[ self.inst_ptr +2] as usize,
        };
        let in_operand2 = self.memory[in_addr2];

        let in_addr3;
        match mode3 {
            Mode::Immediate => in_addr3 = self.inst_ptr + 3,
            Mode::Position => in_addr3 = self.memory[ self.inst_ptr +3] as usize,
        };

        self.memory[in_addr3] = in_operand1 + in_operand2;
        self.inst_ptr += 4;
    }
    
    fn execute_mul(&mut self,mode1:Mode, mode2:Mode, mode3:Mode) {
        let in_addr1;
        match mode1 {
            Mode::Immediate => in_addr1 = self.inst_ptr + 1,
            Mode::Position => in_addr1 = self.memory[ self.inst_ptr +1] as usize,
        };
        let in_operand1 = self.memory[in_addr1];

        let in_addr2;
        match mode2 {
            Mode::Immediate => in_addr2 = self.inst_ptr + 2,
            Mode::Position => in_addr2 = self.memory[ self.inst_ptr +2] as usize,
        };
        let in_operand2 = self.memory[in_addr2];

        let in_addr3;
        match mode3 {
            Mode::Immediate => in_addr3 = self.inst_ptr + 3,
            Mode::Position => in_addr3 = self.memory[ self.inst_ptr +3] as usize,
        };
        self.memory[in_addr3] = in_operand1 * in_operand2;
        self.inst_ptr += 4;
    }

    fn execute_read_input(&mut self, mode:Mode) {
        let in_addr;
        match mode {
            Mode::Immediate => in_addr = self.inst_ptr + 1,
            Mode::Position => in_addr = self.memory[ self.inst_ptr +1] as usize,
        };

        println!("Please input a value:\n");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                // println!("Target Address:{}", addr1);
                // println!("Current Value:{}", self.memory[addr1]);
                self.memory[in_addr] = input.trim().parse().unwrap();
                // println!("Updated Value:{}", self.memory[addr1]);
            },
            Err(error) => {
                println!("{}",error);
                panic!("Couldn't read stdin");
            }
        }
        self.inst_ptr += 2;
    }

    fn execute_write_output(&mut self, mode:Mode) {
        match mode {
            Mode::Immediate => println!("->{}", self.memory[self.inst_ptr +1]),
            Mode::Position  =>{
                let addr = self.memory[self.inst_ptr+1]as usize;
                println!("->{}", self.memory[addr])
            },
        }
        self.inst_ptr += 2;
        // self.execute_halt();
    }

    fn execute_jmp_true(&mut self,mode1:Mode, mode2:Mode){
        let in_addr1;
        match mode1 {
            Mode::Immediate => in_addr1 = self.inst_ptr + 1,
            Mode::Position => in_addr1 = self.memory[ self.inst_ptr +1] as usize,
        };
        let in_operand1 = self.memory[in_addr1];

        let in_addr2;
        match mode2 {
            Mode::Immediate => in_addr2 = self.inst_ptr + 2,
            Mode::Position => in_addr2 = self.memory[ self.inst_ptr +2] as usize,
        };
        let in_operand2 = self.memory[in_addr2];

        if in_operand1 > 0 { self.inst_ptr = in_operand2 as usize }
        else { self.inst_ptr += 3};
    }
    
    fn execute_jmp_false(&mut self,mode1:Mode, mode2:Mode){
        let in_addr1;
        match mode1 {
            Mode::Immediate => in_addr1 = self.inst_ptr + 1,
            Mode::Position => in_addr1 = self.memory[ self.inst_ptr +1] as usize,
        };
        let in_operand1 = self.memory[in_addr1];

        let in_addr2;
        match mode2 {
            Mode::Immediate => in_addr2 = self.inst_ptr + 2,
            Mode::Position => in_addr2 = self.memory[ self.inst_ptr +2] as usize,
        };
        let in_operand2 = self.memory[in_addr2];

        if in_operand1 == 0 { self.inst_ptr = in_operand2 as usize}
        else { self.inst_ptr += 3 }
    }
    
    fn execute_lt(&mut self,mode1:Mode, mode2:Mode, mode3:Mode){
        let in_addr1;
        match mode1 {
            Mode::Immediate => in_addr1 = self.inst_ptr + 1,
            Mode::Position => in_addr1 = self.memory[ self.inst_ptr +1] as usize,
        };
        let in_operand1 = self.memory[in_addr1];

        let in_addr2;
        match mode2 {
            Mode::Immediate => in_addr2 = self.inst_ptr + 2,
            Mode::Position => in_addr2 = self.memory[ self.inst_ptr +2] as usize,
        };
        let in_operand2 = self.memory[in_addr2];

        let in_addr3;
        match mode3 {
            Mode::Immediate => in_addr3 = self.inst_ptr + 3,
            Mode::Position => in_addr3 = self.memory[ self.inst_ptr +3] as usize,
        };
        self.memory[in_addr3] = (in_operand1 < in_operand2) as i32;
        self.inst_ptr += 4;
    }
    
    fn execute_eq(&mut self,mode1:Mode, mode2:Mode, mode3:Mode){
        let in_addr1;
        match mode1 {
            Mode::Immediate => in_addr1 = self.inst_ptr + 1,
            Mode::Position => in_addr1 = self.memory[ self.inst_ptr +1] as usize,
        };
        let in_operand1 = self.memory[in_addr1];

        let in_addr2;
        match mode2 {
            Mode::Immediate => in_addr2 = self.inst_ptr + 2,
            Mode::Position => in_addr2 = self.memory[ self.inst_ptr +2] as usize,
        };
        let in_operand2 = self.memory[in_addr2];

        let in_addr3;
        match mode3 {
            Mode::Immediate => in_addr3 = self.inst_ptr + 3,
            Mode::Position => in_addr3 = self.memory[ self.inst_ptr +3] as usize,
        };
        self.memory[in_addr3] = (in_operand1 == in_operand2) as i32;
        self.inst_ptr += 4;
    }
    
    fn execute_halt(&mut self) {
        self.running = false;
    }

    fn sample(&self, addr:usize) {
        println!("{}: {}",addr, self.memory[addr]);
    }

    fn sample_range(&self, addr:usize, len:usize) {
        for i in addr..addr+len {
            println!("{}: {}",i, self.memory[i]);
        }
    }

}

fn read_digit( number:&i32, place:&u32 ) -> i32 {
    read_digits(number, place, &1)
}

fn read_digits(number:&i32, start_place:&u32, num_digits:&u32) -> i32 {
    number / 10i32.pow(*start_place) % 10i32.pow(*num_digits)
}

fn read_opcode( number: &i32) -> i32 {
    read_digits(number, &0, &2)
}

fn read_mode(number: &i32, place:&u32) -> Mode {
    assert!( place >= &2 && place <= &4);

    match read_digit(number, place) {
        0 => Mode::Position,
        1 => Mode::Immediate,
        _ => {panic!("Can't read the mode")}
    }
}

#[test]
fn one_plus_one() {
    let tape = vec![1101,1,1,5,99,0];
    let mut comp = Computer::init(&tape);
    assert_eq!(0,comp.sample(5));
    comp.run();
    assert_eq!(2,comp.sample(5));
}

#[test]
fn negone_plus_negone() {
    let tape = vec![1101,-1,-1,5,99,0];
    let mut comp = Computer::init(&tape);
    assert_eq!(0,comp.sample(5));
    comp.run();
    assert_eq!(-2,comp.sample(5));
}

#[test]
fn one_plus_one_positional() {
    let tape = vec![1,5,5,5,99,1];
    let mut comp = Computer::init(&tape);
    assert_eq!(1,comp.sample(5));
    comp.run();
    assert_eq!(2,comp.sample(5));
}

#[test]
fn negone_plus_negone_positional() {
    let tape = vec![1,5,5,5,99,-1];
    let mut comp = Computer::init(&tape);
    assert_eq!(-1,comp.sample(5));
    comp.run();
    assert_eq!(-2,comp.sample(5));
}

#[test]
fn two_mul_two_positional() {
    let tape = vec![2,5,5,5,99,2];
    let mut comp = Computer::init(&tape);
    assert_eq!(2,comp.sample(5));
    comp.run();
    assert_eq!(4,comp.sample(5));
}
#[test]
fn two_mul_two() {
    let tape = vec![2,2,2,5,99,0];
    let mut comp = Computer::init(&tape);
    assert_eq!(0,comp.sample(5));
    comp.run();
    assert_eq!(4,comp.sample(5));
}

#[test]
fn mixed_mode_mul() {
    let tape = vec![1002,5,3,5,99,33];
    let mut comp = Computer::init(&tape);
    assert_eq!(33,comp.sample(5));
    comp.run();
    assert_eq!(99,comp.sample(5));
}