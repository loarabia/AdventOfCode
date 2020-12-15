use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

use regex::{Regex, RegexSet};

fn set_bits(num:&mut u64, mask_str:&str) {
    // println!("#####\t{}\t#####", num);
    // println!("{}", mask_str);
    for (i,c) in mask_str.split("=").nth(1).unwrap().trim().as_bytes().iter().rev().enumerate() {
        // println!("{}{:2}    {:36b}", c, i, num);
        // println!("        {:36b}", mask_bit);
        match *c as char {
            'X' => {   },
            '0' => { *num = set_bit_0(num, i as u64) },
            '1' => { *num = set_bit_1(num, i as u64) },
            _ => panic!("There shouldn't be {} char in this", c),
        }   
    } 
}

fn apply_mask(num:u64, mask:&str) -> u64 {
    let mut masked_num = num;

    for (i,c) in mask.split("=").nth(1).unwrap().trim().bytes().rev().enumerate() {
        match c as char {
            'X' => {  },
            '0' => {  },
            '1' => { masked_num = set_bit_1(&masked_num, i as u64) },
            _ => panic!("There shouldn't be {} char in this", c),
        }
    }
    masked_num
}

fn write_memory(input:&str, memory:&mut HashMap<u32,u64>, mask:&str) {
    let re = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)").unwrap();
    let caps = re.captures(input).unwrap();

    let addr = u32::from_str( caps.name("addr").unwrap().as_str()).unwrap();
    let mut val = u64::from_str( caps.name("val").unwrap().as_str()).unwrap();

    set_bits(&mut val, mask);
    memory.insert(addr, val);
}

fn enum_bits( num:&u64, idx:u64) -> Vec<u64> {
    vec![set_bit_0(num, idx), set_bit_1(num,idx)]
}

fn set_bit_0( num:&u64, idx:u64) -> u64 {
    let mask_bit = 1 << idx;
    ( num | mask_bit ) ^ mask_bit
}

fn set_bit_1( num:&u64, idx:u64) -> u64 {
    let mask_bit = 1 << idx;
    num | mask_bit
}

// TOO LOW  7187896205312
// TOO LOW  9744015374479
// TOO HI  10056808196593
// Ans: 10050490168421
fn part1(input:&str) -> u64 {
    
    let mut mask:&str = "";
    let mut memory:HashMap<u32,u64> = HashMap::new();

    let re_set:regex::RegexSet = RegexSet::new(&[
        r"mask = (?P<mask>)\w+", /* mask line */
        r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)"  /* mem line */
    ]).unwrap();

    for line in input.lines(){
        match re_set.matches(line).into_iter().collect::<Vec<usize>>()[0] {
            0 => mask = line,
            1 => write_memory(line, &mut memory, mask),
            _ => panic!("Unknown regex!")
        }
    }

    memory.values().sum()
}

fn generate_addresses(mask:&str, base_addr:u64) -> Vec<u64> {
    let mut addresses:Vec<u64> = vec![base_addr];

    for (i, _x) in mask.bytes().rev().enumerate().filter(|(_i,x)| *x == 'X' as u8) {
        let mut new_addresses:Vec<u64> = Vec::new();
        while let Some(val) = addresses.pop() {
            
            // Apply mask
            let masked_val = apply_mask(val, mask);            

            // Enumerate options
            new_addresses.append( &mut enum_bits(&masked_val, i as u64));
        }
        addresses.append(&mut new_addresses);
    }

    // Sanity check to catch logic errors early
    let num_x:u32 = mask.bytes().filter(|c| *c as char == 'X').count() as u32;
    assert!(2usize.pow(num_x) == addresses.len());

    addresses
}

fn write_memory_masked(input:&str, memory:&mut HashMap<u64,u64>, mask:&str) {

    let re = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)").unwrap();
    let caps = re.captures(input).unwrap();

    let orig_addr = u64::from_str( caps.name("addr").unwrap().as_str()).unwrap();
    let val = u64::from_str( caps.name("val").unwrap().as_str()).unwrap();

    let addresses = generate_addresses(mask, orig_addr);

    for addr in addresses {
        memory.insert(addr, val);
    }

}

// TOO HI  2871839400490
//         2173858456958
fn part2(input:&str) -> u64 {

    let mut mask:&str = "";
    let mut memory:HashMap<u64,u64> = HashMap::new();

    let re_set:regex::RegexSet = RegexSet::new(&[
        r"mask = (?P<mask>)\w+", /* mask line */
        r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)"  /* mem line */
    ]).unwrap();

    for line in input.lines(){
        match re_set.matches(line).into_iter().collect::<Vec<usize>>()[0] {
            0 => mask = line,
            1 => write_memory_masked(line, &mut memory, mask),
            _ => panic!("Unknown regex!")
        }
    }

    memory.values().sum()
}

fn main() {
    let filename = "input.txt";
    let input = read_to_string(filename).expect(&format!("Couldn't read file: {}", filename));
    println!("{}",part1(&input));
    println!("{}",part2(&input));
}
