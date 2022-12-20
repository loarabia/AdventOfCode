
use core::str::Lines;
use std::fs::{read_to_string};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{static ref RE_MONO:Regex = Regex::new(r"Monkey (\d+):").unwrap();}
lazy_static!{static ref RE_M_ITEMS:Regex = Regex::new(r"Starting items: ([ ,0-9]+)").unwrap();}
//lazy_static!{static ref RE_M_OP:Regex = Regex::new(r"Operation: (.+)").unwrap();}
lazy_static!{static ref RE_M_OP:Regex = Regex::new(r"Operation: new = old ([\+\*]) (old|\d+)").unwrap();}
lazy_static!{static ref RE_M_TEST:Regex = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();}
lazy_static!{static ref RE_M_TEST_T:Regex = Regex::new(r"If true: throw to monkey ([0-9]+)").unwrap();}
lazy_static!{static ref RE_M_TEST_F:Regex = Regex::new(r"If false: throw to monkey ([0-9]+)").unwrap();}

fn read_monkey_id(line:&str) -> u128 {
    let caps = RE_MONO.captures(line).unwrap();
    caps[1].parse::<u128>().unwrap()
}

fn read_items(line:&str) -> Vec<u128> {
    let mut items = Vec::new();
    let caps = RE_M_ITEMS.captures(line).unwrap();
    //println!("{}", &caps[1]);
    for item in caps[1].split(',') {
        items.push(item.trim().parse::<u128>().unwrap());
    }
    items
}

fn read_operation(line:&str) -> Box<dyn Fn(u128) -> u128> {
    let caps = RE_M_OP.captures(line).unwrap();
    println!("{:?} -- {:?}",&caps[1], &caps[2]);

    let is_add = &caps[1] == "+";
    let op_value = caps[2].parse::<u128>();

    let fnc:Box<dyn Fn(u128)-> u128> = match (is_add, op_value) {
        (true, Ok(val)) =>  Box::new(move |x:u128| x + val),
        (false, Ok(val)) => Box::new(move |x:u128| x * val),
        (false, Err(_e)) => Box::new(|x:u128| x.pow(2)),
        (_,_) => {panic!("HOW?!?!")}, 
    };

    fnc
}

fn read_test2(line:&mut Lines) -> (Box<dyn Fn(u128)->u128>, u128) {
    let test = read_test(line.next().unwrap());
    let t_v = read_test_true(line.next().unwrap());
    let f_v = read_test_false(line.next().unwrap());
    (Box::new(move |val| if val % test == 0 { return t_v; } else { return f_v; }),test)
}

fn read_test(line:&str) -> u128 {
    let caps = RE_M_TEST.captures(line).unwrap();
    caps[1].parse::<u128>().unwrap()
}

fn read_test_true(line:&str)->u128 {
    let caps = RE_M_TEST_T.captures(line).unwrap();
    caps[1].parse::<u128>().unwrap()
}

fn read_test_false(line:&str)->u128 {
    let caps = RE_M_TEST_F.captures(line).unwrap();
    caps[1].parse::<u128>().unwrap()
}

struct Monkey {
    id:u128,
    op:Box<dyn Fn(u128)->u128>,
    test:Box<dyn Fn(u128)->u128>,
    test_value:u128,
    items:Vec<u128>,
    items_handled:u128,
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<(usize,u128)>{
        let mut tosses = Vec::new();
        while self.items.len() > 0 {
            let item = self.items.remove(0);
            let worry = (self.op)(item);
            let chill = worry/3;
            let target = (self.test)(chill);
            tosses.push( (target as usize,chill) );
        }//While
        self.items_handled += tosses.len() as u128;
        tosses
    }

    fn take_worryless_turn(&mut self, lcm:u128) -> Vec<(usize,u128)>{
        let mut tosses = Vec::new();
        while self.items.len() > 0 {
            let item = self.items.remove(0);
            // println!("{}",item);
            let worry = (self.op)(item);
            let chill = worry % lcm;
            let target = (self.test)(chill);
            tosses.push( (target as usize,chill) );
        }//While
        self.items_handled += tosses.len() as u128;
        tosses
    } 
}

fn print_monkey_items(monkeys:&Vec<Monkey>) {
    for monkey in monkeys {
        println!("{:?}", monkey.items);
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input:&String) -> u128 {
    let mut lines = input.lines();
    let mut monkeys:Vec<Monkey> = Vec::new();
    
    // READ MONKEY DESCRIPTIONS
    loop {
        let id =  read_monkey_id(lines.next().unwrap());
        let items = read_items(lines.next().unwrap());
        let op = read_operation(lines.next().unwrap());
        let test_parts = read_test2(&mut lines);
        let test_op = test_parts.0;
        let text_val = test_parts.1;
        //println!();
        monkeys.push(Monkey { 
            id:id,
            op:op,
            test:test_op,
            test_value:text_val,
            items:items,
            items_handled:0,
        });
        if lines.next() == None { break; }
    }

    for _rnd in 0..20 {
        for i in 0..monkeys.len() {
            let mono = &mut monkeys[i];
            let tosses = mono.take_turn();
            for toss in tosses {
                monkeys[toss.0].items.push(toss.1);
            }
        }
    }

    monkeys.sort_by_key(|m| m.items_handled);
    monkeys.reverse();
    // for mono in monkeys {
        // println!("{},{}", mono.id, mono.items_handled);
    // }

    monkeys[0].items_handled * monkeys[1].items_handled

}

fn part2(input: &String) -> u128 {
    let mut lines = input.lines();
    let mut monkeys:Vec<Monkey> = Vec::new();
    
    // READ MONKEY DESCRIPTIONS
    loop {
        let id =  read_monkey_id(lines.next().unwrap());
        let items = read_items(lines.next().unwrap());
        let op = read_operation(lines.next().unwrap());
        let test_parts = read_test2(&mut lines);
        let test_op = test_parts.0;
        let text_val = test_parts.1;
        //println!();
        monkeys.push(Monkey { 
            id:id,
            op:op,
            test:test_op,
            test_value:text_val, 
            items:items,
            items_handled:0,
        });
        if lines.next() == None { break; }
    }

    let mut lcm = 1;
    for monkey in &monkeys {
        lcm *= monkey.test_value;
    }
    println!("LCM {}", lcm);

    for _rnd in 0..10000 {
        for i in 0..monkeys.len() {
            let mono = &mut monkeys[i];
            let tosses = mono.take_worryless_turn(lcm);
            for toss in tosses {
                monkeys[toss.0].items.push(toss.1);
            }
        }
    }

    monkeys.sort_by_key(|m| m.items_handled);
    monkeys.reverse();
    // for mono in monkeys {
        // println!("{},{}", mono.id, mono.items_handled);
    // }

    monkeys[0].items_handled * monkeys[1].items_handled
}