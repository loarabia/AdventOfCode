use std::fs::{read_to_string};
//use std::str::FromStr;
//use std::vec::Vec;
use std::option::Option;
use std::collections::HashSet;
use std::collections::HashMap;

// 6768 is too low -- have bug
fn part1(input: &str) -> Option<usize> {
    let mut count:usize = 0;
    let mut set = HashSet::<u8>::new();
    for line in input.lines(){
        if line.is_empty() {
            count += set.len();
            set.clear();
        } else {
            for c in line.as_bytes() {
                set.insert(*c);
            }
        }
    }
    count += set.len();
    Some(count)
}

fn part2(input: &str) -> Option<usize> {
    let mut result:usize = 0;
    let mut map = HashMap::<u8,usize>::new();
    let mut num_mmbrs:usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            // Iterate over map and count how many items are equal to number of group members
            result += map.values().filter(|&&val| val == num_mmbrs).count();
            // println!("{}", map.values().filter(|&&val| val == num_mmbrs).count());
            // Clear everything
            map.clear();
            num_mmbrs = 0;
        } else {
            // Each row is a passenger
            for c in line.as_bytes() {
                // Try to get the key if it is there increment otherwise initialize
                let key = match map.get(c) {
                    Some(val) => val+1,
                    None => 1,
                };
                map.insert(*c, key);
            }
            num_mmbrs += 1;
        }
    }
    result += map.values().filter(|&&val| val == num_mmbrs).count();
    Some(result)
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input1.txt");
    println!("{}",part1(&input).unwrap());
    println!("{}",part2(&input).unwrap());
}
