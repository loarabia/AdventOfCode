
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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

pub struct Computer {
    inst_ptr:usize,
    state:State,
    memory:Vec<i32>,
    pub in_reg:Option<i32>,
    pub out_reg:Option<i32>,
    name:String,
}

impl Computer {
    pub fn init( tape:&Vec<i32>, name:String) -> Computer {
        let c = Computer {
            inst_ptr: 0,
            state: State::Ready,
            memory: tape.clone(),
            in_reg: Option::None,
            out_reg: Option::None,
            name:name,
        };
        c
    }

    fn fetch(& self) -> i32 {
        self.memory[self.inst_ptr]
    }

    fn decode(& self, opcode:i32) -> Instruction {
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
        println!("{} - Requesting Input", self.name);
        if self.in_reg == Option::None {
            println!("{} - Requesting Input: None ready", self.name);
            self.state = State::WaitingForInput;
        }

        if let Some(value) = self.in_reg {
            println!("{} - Requesting Input: Recieved", self.name);
            self.write_memory(value, 1, mode);
            self.in_reg = Option::None;
            self.inst_ptr += 2;
        }
    }

    fn execute_write_output(&mut self, mode:Mode) {
        // match mode {
        //     Mode::Immediate => println!("->{}", self.memory[self.inst_ptr +1]),
        //     Mode::Position  =>{
        //         let addr = self.memory[self.inst_ptr+1]as usize;
        //         println!("->{}", self.memory[addr])
        //     },
        // }
        println!("{} - Writing Output", self.name);
        self.state = State::WroteOutput;
        self.out_reg = Some(self.read_memory(1, mode));
        self.inst_ptr += 2;
    }

    fn execute_jmp_true(&mut self,mode1:Mode, mode2:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);

        if in_operand1 > 0 { self.inst_ptr = in_operand2 as usize }
        else { self.inst_ptr += 3};
    }
    
    fn execute_jmp_false(&mut self,mode1:Mode, mode2:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);

        if in_operand1 == 0 { self.inst_ptr = in_operand2 as usize}
        else { self.inst_ptr += 3 }
    }
    
    fn execute_lt(&mut self,mode1:Mode, mode2:Mode, mode3:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);
        
        self.write_memory((in_operand1 < in_operand2) as i32, 3, mode3);

        self.inst_ptr += 4;
    }
    
    fn execute_eq(&mut self,mode1:Mode, mode2:Mode, mode3:Mode){
        let in_operand1 = self.read_memory(1, mode1);
        let in_operand2 = self.read_memory(2, mode2);

        self.write_memory((in_operand1 == in_operand2) as i32, 3, mode3);
        
        self.inst_ptr += 4;
    }
    
    fn execute_halt(&mut self) {
        println!("{} - HALTING!!!!!!!!!!!!!!!!!!!!", self.name);
        self.state = State::Halted;
    }

    fn sample(&self, addr:usize) {
        println!("{}: {}",addr, self.memory[addr]);
    }

    fn sample_range(&self, addr:usize, len:usize) {
        for i in addr..addr+len {
            println!("{}: {}",i, self.memory[i]);
        }
    }

    fn read_memory(&self, offset:usize, mode:Mode) -> i32 {
        let addr;
        match mode {
            Mode::Immediate => addr = self.inst_ptr + offset,
            Mode::Position => addr = self.memory[self.inst_ptr + offset] as usize,
        }
        self.memory[addr]
    }

    fn write_memory(&mut self, value:i32, offset:usize, mode:Mode) {
        let addr;
        match mode {
            Mode::Immediate => addr = self.inst_ptr + offset,
            Mode::Position => addr = self.memory[ self.inst_ptr + offset] as usize,
        };

        self.memory[addr] = value;
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