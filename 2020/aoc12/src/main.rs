use std::fs::read_to_string;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Heading {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
enum NavInst {
    North(u32),
    South(u32),
    West(u32),
    East(u32),
    Forward(u32),
    Backward(u32),
    Left(u32),
    Right(u32),
}

fn manhattan_distance(ship_loc:&(i32,i32)) -> u32 {
    (ship_loc.0.abs() +ship_loc.1.abs()) as u32
}

fn read_inst(input:&str) -> NavInst {
    let re = Regex::new(r"(?P<op>[NSWEFBLR])(?P<num>\d+)").unwrap();
    let caps = re.captures(input).unwrap();

    let num = u32::from_str(caps.name("num").unwrap().as_str()).unwrap();
    
    let inst = match caps.name("op").unwrap().as_str() {
        "N" => NavInst::North(num),
        "S" => NavInst::South(num),
        "W" => NavInst::West(num),
        "E" => NavInst::East(num),
        "F" => NavInst::Forward(num),
        "B" => NavInst::Backward(num),        
        "L" => NavInst::Left(num),
        "R" => NavInst::Right(num),
        _ => panic!("Unknown op {}", input ),
    };

    inst
}

fn update_heading(curr_heading:&Heading, nav_inst:&NavInst) -> Heading {

    let mut rotations:Vec<Heading> = Vec::new();
    rotations.push(Heading::North); // 0
    rotations.push(Heading::East); // 1
    rotations.push(Heading::South); // 2
    rotations.push(Heading::West); // 3
    rotations.push(Heading::North); // 4
    rotations.push(Heading::East); // 5
    rotations.push(Heading::South); // 6
    rotations.push(Heading::West); // 7
    rotations.push(Heading::North); // 8
    rotations.push(Heading::East); // 9
    rotations.push(Heading::South); // 10
    rotations.push(Heading::West); // 11
    
    let rot_start = match curr_heading {
        Heading::North => 4,
        Heading::South => 6,
        Heading::West => 7,
        Heading::East => 5,
    };

    let rot_diff = match nav_inst {
        NavInst::Left(val) | NavInst::Right(val) => {
            match val {
                0 => 0,
                90 => 1,
                180 => 2,
                270 => 3,
                360 => 0,
                _ => panic!("Don't know how to rotate by {}", val),
            }
        },
        _ => panic!("Cannot rotate by {:?}", nav_inst),
    };

    // RETURNS result
    match nav_inst {
        NavInst::Left(_) => rotations[rot_start-rot_diff],
        NavInst::Right(_) => rotations[rot_start+rot_diff],
        _ => panic!("Cannot rotate by {:?}", nav_inst),
    }
}

fn move_along_heading( dist:i32, loc:&mut (i32,i32, Heading)) {
    match loc.2 {
        Heading::North => loc.1 += dist as i32,
        Heading::South => loc.1 -= dist as i32,
        Heading::West => loc.0 -= dist as i32,
        Heading::East => loc.0 += dist as i32,
    };
}

fn execute_inst( ship_loc:&mut (i32, i32, Heading), inst:NavInst) {
    // println!("=========================");
    // println!("{:?}",ship_loc);
    // println!("{:?}", inst);
    match inst {
        NavInst::North(num) => ship_loc.1 += num as i32,
        NavInst::South(num) => ship_loc.1 -= num as i32,
        NavInst::West(num) => ship_loc.0 -= num as i32,
        NavInst::East(num) => ship_loc.0 += num as i32,
        NavInst::Forward(num) => move_along_heading(num as i32, ship_loc),
        NavInst::Backward(num) => move_along_heading(-(num as i32), ship_loc),
        NavInst::Left(_num) => ship_loc.2 = update_heading(&ship_loc.2, &inst),
        NavInst::Right(_num) => ship_loc.2 = update_heading(&ship_loc.2, &inst),
    };
    // println!("{:?}",ship_loc);
}

fn execute_waypoint_inst(
    ship:&mut (i32, i32),
    wypt:&mut (i32,i32),
    inst:&NavInst,
) {
    // println!("=========================");
    // println!("ship:{:?}, wypt:{:?}",ship,wypt);
    // println!("{:?}", inst);
    match inst {
        NavInst::North(_num) => wypt_move(wypt, inst),
        NavInst::South(_num) => wypt_move(wypt, inst),
        NavInst::West(_num) => wypt_move(wypt, inst),
        NavInst::East(_num) => wypt_move(wypt, inst),

        NavInst::Forward(num) => ship_move(*num as i32, wypt, ship),
        NavInst::Backward(_num) => {/* NOP */} ,
        
        NavInst::Left(num) => wypt_rot(wypt,ship,true,*num as i32),
        NavInst::Right(num) => wypt_rot(wypt,ship,false,*num as i32),
    };
}

fn wypt_move(wypt:&mut(i32,i32), inst:&NavInst) {

    match inst {
        NavInst::North(val) => wypt.1 += *val as i32,
        NavInst::South(val) => wypt.1 -= *val as i32,
        NavInst::West(val) => wypt.0 -= *val as i32,
        NavInst::East(val) => wypt.0 += *val as i32,
        _ => panic!("Expected N,s,w,e, got {:?}", inst),
    };
}

fn wypt_rot(wypt:&mut(i32,i32), ship:&mut (i32,i32), rot_left:bool, angle:i32) {

    // let translate_wypt = (wypt.0-ship.0, wypt.1-ship.1);
    let translate_wypt = (wypt.0, wypt.1);

    let new_wypt = match (rot_left, angle) {
        (_,0)|(_,360) => translate_wypt,
        (_,180) => (-translate_wypt.0, -translate_wypt.1),
        (true,90)|(false,270)=> (-translate_wypt.1, translate_wypt.0),
        (false,90)|(true,270)=> (translate_wypt.1, -translate_wypt.0),
        _ => panic!("Not sure what to do"),
    };

    wypt.0 = new_wypt.0;
    wypt.1 = new_wypt.1;
}

fn ship_move(times:i32, wypt:&mut(i32,i32), ship:&mut (i32,i32)) {
    ship.0 += wypt.0*times;
    ship.1 += wypt.1*times;
}

// 479 TOO LOW
// 2243 TOO HIGH
fn part1(input:&str) -> u32 {
    let mut ship_loc = (0,0, Heading::East);
    for inst in input.lines().map(|inst_str| read_inst(inst_str)) {
        execute_inst(&mut ship_loc, inst);
    }
    manhattan_distance(&(ship_loc.0,ship_loc.1))
}

fn part2(input:&str) -> u32 {
    let mut ship = (0,0);
    let mut wypt = (10, 1);

    for inst in input
        .lines()
        .map(|inst_str| read_inst(inst_str))
        .filter(|inst| match inst {
            NavInst::Backward(_val) => false,
            _ => true, 
        }) {
        execute_waypoint_inst(&mut ship, &mut wypt, &inst);
    }
    manhattan_distance(&ship)

}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input file");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
