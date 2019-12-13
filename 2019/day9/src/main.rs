use std::fs;
use std::str::FromStr;

mod computer;
use computer::{Computer, State};

fn main() {
    let filename:&str = "input.txt";
    let contents:String = fs::read_to_string(filename).unwrap(); 

    let tape:Vec<i128> = contents.split(',')
    .map(|num| i128::from_str(num).expect("Bad line item") )
    .collect();

    run_pt1(&tape);
    run_pt2(&tape);
}

fn run_pt1(tape:&Vec<i128>){
    let comp_name = "Hal9000";
    let mut comp = Computer::init(tape, String::from_str(comp_name).unwrap());
    
    let mut state = comp.run();
    
    while state != State::Halted {
        match state {
            State::WaitingForInput => comp.in_reg = Some(1),
            State::WroteOutput =>  println!("{:?}", comp.out_reg),
            State::Halted => break,
            _ => println!("{:?}", state),
        }
        state = comp.run();
    }
}

fn run_pt2(tape:&Vec<i128>){
    let comp_name = "Hal9000";
    let mut comp = Computer::init(tape, String::from_str(comp_name).unwrap());
    
    let mut state = comp.run();
    
    while state != State::Halted {
        match state {
            State::WaitingForInput => comp.in_reg = Some(2),
            State::WroteOutput =>  println!("{:?}", comp.out_reg),
            State::Halted => break,
            _ => println!("{:?}", state),
        }
        state = comp.run();
    }
}