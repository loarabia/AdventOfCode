use std::fs;
use std::str::FromStr;

mod computer;
use computer::{Computer, State};

struct ThrustResult {
    phase:String,
    value:i32,
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("File access error");

    let tape:Vec<i32> = contents.split(',')
        .map(|num| i32::from_str(num).expect("Bad line item") )
        .collect();

    run_part1(&tape);
    //run_part2(&tape);
}

fn run_part1(tape:&Vec<i32>) {

    let mut result = ThrustResult {phase:String::new(), value:-1};
    let mut thrust = -1;
    let min = 0;
    let max = 4;

    for phase1 in min..=max {
        for phase2 in min..=max {
            if phase2 == phase1 {continue};

            for phase3 in min..=max {
                if phase3 == phase2 || phase3 == phase1 { continue; }

                for phase4 in min..=max {
                    if phase4 == phase3 || phase4 == phase2 || phase4 == phase1 {continue; }

                    for phase5 in min..=max {
                        if phase5 == phase4 || phase5 == phase3 || phase5 == phase2 || phase5 == phase1 {continue;}
                        // println!("{}{}{}{}{}", phase1, phase2, phase3, phase4, phase5);
                        thrust = run_combination(phase1, phase2, phase3, phase4, phase5, tape);
                        
                        if thrust > result.value {
                            result = ThrustResult{
                                phase: format!("{}{}{}{}{}", phase1,phase2,phase3,phase4,phase5),
                                value: thrust,
                            }
                        }//if
                    }//5
                }//4
            }//3
        }//2
    }//1
    println!("{}-{}", result.phase, result.value);
}

// fn run_part2(tape:&Vec<i32>){
//     let mut result = ThrustResult {phase:String::new(), value:-1};
//     let mut thrust = -1;
//     let min = 5;
//     let max = 9;

//     for phase1 in min..=max {
//         for phase2 in min..=max {
//             if phase2 == phase1 {continue};

//             for phase3 in min..=max {
//                 if phase3 == phase2 || phase3 == phase1 { continue; }

//                 for phase4 in min..=max {
//                     if phase4 == phase3 || phase4 == phase2 || phase4 == phase1 {continue; }

//                     for phase5 in min..=max {
//                         if phase5 == phase4 || phase5 == phase3 || phase5 == phase2 || phase5 == phase1 {continue;}
//                         // println!("{}{}{}{}{}", phase1, phase2, phase3, phase4, phase5);
//                         thrust = run_combination(phase1, phase2, phase3, phase4, phase5, tape);
                        
//                         if thrust > result.value {
//                             result = ThrustResult{
//                                 phase: format!("{}{}{}{}{}", phase1,phase2,phase3,phase4,phase5),
//                                 value: thrust,
//                             }
//                         }//if
//                     }//5
//                 }//4
//             }//3
//         }//2
//     }//1
//     println!("{}-{}", result.phase, result.value);
//     println!("{}-{}", result.phase, result.value);
// }

fn run_combination(phase1:i32, phase2:i32, phase3:i32, phase4:i32, phase5:i32, tape:&Vec<i32>) -> i32 { 

    let mut computer1 = Computer::init(tape);
    let mut computer2 = Computer::init(tape);
    let mut computer3 = Computer::init(tape);
    let mut computer4 = Computer::init(tape);
    let mut computer5 = Computer::init(tape);
    
    let mut input1:Vec<i32> = Vec::new();
    let mut input2:Vec<i32> = Vec::new();
    let mut input3:Vec<i32> = Vec::new();
    let mut input4:Vec<i32> = Vec::new();
    let mut input5:Vec<i32> = Vec::new();
    
    input1.push(phase1);
    input2.push(phase2);
    input3.push(phase3);
    input4.push(phase4);
    input5.push(phase5);

    input1.push(0);

    computer1.in_reg = Some(input1.remove(0));
    let mut state1 = computer1.run();
    while state1 != State::Halted {
        if state1 == State::WaitingInput {
            computer1.in_reg = Some(input1.remove(0));
            state1 = computer1.run();
        }
    }
    input2.push(computer1.out_reg.unwrap());

    computer2.in_reg = Some(input2.remove(0));
    let mut state2 = computer2.run();
    while state2 != State::Halted {
        if state2 == State::WaitingInput {
            computer2.in_reg = Some(input2.remove(0));
            state2 = computer2.run();
        }
    }
    input3.push(computer2.out_reg.unwrap());

    computer3.in_reg = Some(input3.remove(0));
    let mut state3 = computer3.run();
    while state3 != State::Halted {
        if state3 == State::WaitingInput {
            computer3.in_reg = Some(input3.remove(0));
            state3 = computer3.run();
        }
    }
    input4.push(computer3.out_reg.unwrap());

    computer4.in_reg = Some(input4.remove(0));
    let mut state4 = computer4.run();
    while state4 != State::Halted {
        if state4 == State::WaitingInput {
            computer4.in_reg = Some(input4.remove(0));
            state4 = computer4.run();
        }
    }
    input5.push(computer4.out_reg.unwrap());

    computer5.in_reg = Some(input5.remove(0));
    let mut state5 = computer5.run();
    while state5 != State::Halted {
        if state5 == State::WaitingInput {
            computer5.in_reg = Some(input5.remove(0));
            state5 = computer5.run();
        }
    }

    computer5.out_reg.unwrap()
}

// fn run_combination_with_feedback(phase1:i32, phase2:i32, phase3:i32, phase4:i32, phase5:i32, tape:&Vec<i32>) -> i32 { 

//     let mut computers:Vec<Computer> = Vec::new();
//     for i in 0..5 {} 
//     let mut computer1 = Computer::init(tape);
//     let mut computer2 = Computer::init(tape);
//     let mut computer3 = Computer::init(tape);
//     let mut computer4 = Computer::init(tape);
//     let mut computer5 = Computer::init(tape);
    
//     let mut input1:Vec<i32> = Vec::new();
//     let mut input2:Vec<i32> = Vec::new();
//     let mut input3:Vec<i32> = Vec::new();
//     let mut input4:Vec<i32> = Vec::new();
//     let mut input5:Vec<i32> = Vec::new();
    
//     input1.push(phase1);
//     input2.push(phase2);
//     input3.push(phase3);
//     input4.push(phase4);
//     input5.push(phase5);

//     input1.push(0);

//     computer1.in_reg = Some(input1.remove(0));
//     let mut state1 = computer1.run();
//     while state1 != State::Halted {
//         if state1 == State::WaitingInput {
//             computer1.in_reg = Some(input1.remove(0));
//             state1 = computer1.run();
//         }
//     }
//     input2.push(computer1.out_reg.unwrap());

//     computer2.in_reg = Some(input2.remove(0));
//     let mut state2 = computer2.run();
//     while state2 != State::Halted {
//         if state2 == State::WaitingInput {
//             computer2.in_reg = Some(input2.remove(0));
//             state2 = computer2.run();
//         }
//     }
//     input3.push(computer2.out_reg.unwrap());

//     computer3.in_reg = Some(input3.remove(0));
//     let mut state3 = computer3.run();
//     while state3 != State::Halted {
//         if state3 == State::WaitingInput {
//             computer3.in_reg = Some(input3.remove(0));
//             state3 = computer3.run();
//         }
//     }
//     input4.push(computer3.out_reg.unwrap());

//     computer4.in_reg = Some(input4.remove(0));
//     let mut state4 = computer4.run();
//     while state4 != State::Halted {
//         if state4 == State::WaitingInput {
//             computer4.in_reg = Some(input4.remove(0));
//             state4 = computer4.run();
//         }
//     }
//     input5.push(computer4.out_reg.unwrap());

//     computer5.in_reg = Some(input5.remove(0));
//     let mut state5 = computer5.run();
//     while state5 != State::Halted {
//         if state5 == State::WaitingInput {
//             computer5.in_reg = Some(input5.remove(0));
//             state5 = computer5.run();
//         }
//     }

//     computer5.out_reg.unwrap()
// }
