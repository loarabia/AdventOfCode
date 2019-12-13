use std::convert::TryFrom;

use num_traits::pow;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum State {
    Ready, //
    WaitingForInput,
    WroteOutput,
    Running,
    Halted,
}

enum Mode {
    Immediate,
    Position,
    Relative,
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
    AdjRelBase(Mode), //9
}

// IntCode computer
// Supports a 64 bit address space on a 64 bit CPI and 32 bit on a 32 bit CPU.
// It can operate on 128 bit values.
pub struct Computer {
    inst_ptr:usize,
    state:State,
    memory:Vec<i128>,
    pub in_reg:Option<i128>,
    pub out_reg:Option<i128>,
    name:String,
    relative_base:usize,
}

impl Computer {
    pub fn init( tape:&Vec<i128>, name:String) -> Computer {
        Computer {
            inst_ptr: 0,
            state: State::Ready,
            memory: tape.clone(),
            in_reg: Option::None,
            out_reg: Option::None,
            name:name,
            relative_base:0,
        }
    }

    fn fetch(& self) -> i128 {
        self.memory[self.inst_ptr]
    }

    fn decode(& self, opcode:i128) -> Instruction {
        match read_opcode(&opcode) {
            1 => { Instruction::Add(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            2 => { Instruction::Mul(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            3 => { Instruction::ReadInput(read_mode(&opcode,&2))},
            4 => { Instruction::WriteOutput(read_mode(&opcode, &2))},
            5 => { Instruction::JumpIfTrue(
                    read_mode(&opcode, &2),
                    read_mode(&opcode, &3))},
            6 => { Instruction::JumpIfFalse(
                    read_mode(&opcode, &2),
                    read_mode(&opcode, &3))},
            7 => { Instruction::LT(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            8 => { Instruction::EQ(
                    read_mode(&opcode, &2), // param 1 mode
                    read_mode(&opcode, &3), // param 2 mode
                    read_mode(&opcode, &4)) // param 3 mode
                },
            9 => { Instruction::AdjRelBase(
                    read_mode(&opcode, &2))
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

    pub fn run(&mut self) -> State {
        self.state = State::Running;
        while self.state == State::Running {
            match self.decode(self.fetch()) {
                Instruction::HALT => self.execute_halt(),
                Instruction::Add(m1,m2,m3) => self.execute_add(m1,m2,m3),
                Instruction::Mul(m1,m2,m3) => self.execute_mul(m1, m2, m3),
                Instruction::ReadInput(m1) => self.execute_read_input(m1),
                Instruction::WriteOutput(m1) => self.execute_write_output(m1),
                Instruction::JumpIfTrue(m1,m2) => self.execute_jmp_true(m1,m2),
                Instruction::JumpIfFalse(m1,m2) => self.execute_jmp_false(m1,m2),
                Instruction::LT(m1,m2,m3) => self.execute_lt(m1,m2,m3),
                Instruction::EQ(m1,m2,m3) => self.execute_eq(m1,m2,m3),
                Instruction::AdjRelBase(m1) => self.execute_adjust_relative_base(m1),
            }
        }
        self.state
    }

    fn execute_add(&mut self, mode1:Mode, mode2:Mode, mode3:Mode) {
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);

        self.write_memory(in_operand1 + in_operand2, 3, mode3);

        self.inst_ptr += 4;
    }
    
    fn execute_mul(&mut self,mode1:Mode, mode2:Mode, mode3:Mode) {
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);
        
        self.write_memory(in_operand1 * in_operand2, 3, mode3);

        self.inst_ptr += 4;
    }

    fn execute_read_input(&mut self, mode:Mode) {
        // println!("{} - Requesting Input", self.name);
        if self.in_reg == Option::None {
            // println!("{} - Requesting Input: None ready", self.name);
            self.state = State::WaitingForInput;
        }

        if let Some(value) = self.in_reg {
            // println!("{} - Requesting Input: Recieved", self.name);
            self.write_memory(value, 1, mode);
            self.in_reg = Option::None;
            self.inst_ptr += 2;
        }
    }

    fn execute_write_output(&mut self, mode:Mode) {
        self.state = State::WroteOutput;
        self.out_reg = Some(self.read_memory(1, mode));
        self.inst_ptr += 2;
    }

    fn execute_jmp_true(&mut self, mode1:Mode, mode2:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);

        if in_operand1 > 0 { self.inst_ptr = in_operand2 as usize }
        else { self.inst_ptr += 3};
    }
    
    fn execute_jmp_false(&mut self, mode1:Mode, mode2:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);

        if in_operand1 == 0 { self.inst_ptr = in_operand2 as usize}
        else { self.inst_ptr += 3 }
    }
    
    fn execute_lt(&mut self, mode1:Mode, mode2:Mode, mode3:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);
        
        self.write_memory((in_operand1 < in_operand2) as i128, 3, mode3);

        self.inst_ptr += 4;
    }
    
    fn execute_eq(&mut self,mode1:Mode, mode2:Mode, mode3:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);
        self.write_memory((in_operand1 == in_operand2) as i128, 3, mode3);
        
        self.inst_ptr += 4;
    }
    
    fn execute_halt(&mut self) {
        println!("{} - HALTING!!!!!!!!!!!!!!!!!!!!", self.name);
        self.state = State::Halted;
    }

    fn execute_adjust_relative_base(&mut self, mode1:Mode) {
        let in_operand1 = self.read_memory(1, mode1);
        match in_operand1.is_negative() {
            true => self.relative_base -= in_operand1.abs() as usize,
            false => self.relative_base += in_operand1 as usize,
        }
        self.inst_ptr += 2;
    }

    fn sample(&self, addr:usize) {
        println!("{}: {}",addr, self.memory[addr]);
    }

    fn sample_range(&self, addr:usize, len:usize) {
        for i in addr..addr+len {
            println!("{}: {}",i, self.memory[i]);
        }
    }

    fn calc_immediate_addr(&self, offset: isize) -> usize {
        match offset.is_negative() {
            true => self.inst_ptr - offset.abs() as usize,
            false => self.inst_ptr + offset as usize,
        }
    }

    fn calc_positional_addr(&self, offset: isize) -> usize {
        self.memory[self.calc_immediate_addr(offset)] as usize
    }

    fn calc_relative_addr(&self, offset: isize) -> usize {
        let value = self.memory[ self.calc_immediate_addr(offset)];
        
        match value.is_negative() {
            true => self.relative_base - value.abs() as usize,
            false => self.relative_base + value as usize,
        }
    }

    fn calc_addr(&self, offset:isize, mode:Mode) -> usize {
        match mode {
            Mode::Immediate => self.calc_immediate_addr(offset),
            Mode::Position => self.calc_positional_addr(offset),
            Mode::Relative => self.calc_relative_addr(offset), 
        }
    }

    fn read_memory(&mut self, offset:isize, mode:Mode) -> i128 {
        let addr = self.calc_addr(offset, mode);
        if addr > self.memory.len() {
            self.memory.resize_with(addr+1, Default::default);
        }
        self.memory[addr]
    }

    fn write_memory(&mut self, value:i128, offset:isize, mode:Mode) {
        let addr = self.calc_addr(offset, mode);
        if addr >= self.memory.len() {
            self.memory.resize_with(addr+1, Default::default);
        }
        self.memory[addr] = value;
    }
}

fn read_digit( number:&i128, place:&usize ) -> i128 {
    read_digits(number, place, &1)
}

fn read_digits(number:&i128, start_place:&usize, num_digits:&usize) -> i128 {
    *number / pow(10i128, *start_place) % pow(10i128, *num_digits)
}

fn read_opcode(number: &i128) -> i128 {
    read_digits(number, &0, &2)
}

fn read_mode(number: &i128, place:&usize) -> Mode {
    assert!( place >= &2 && place <= &4);

    match read_digit(number, place) {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => {panic!("Can't read the mode")}
    }
}