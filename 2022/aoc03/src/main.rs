use std::fs::{read_to_string};
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}


fn split_sack(input:&str) -> (&[u8], &[u8]) {
    let data: &[u8] = input.as_bytes();
    let sack1:&[u8] = &data[0..data.len()/2];
    let sack2:&[u8] = &data[data.len()/2..data.len()];
    (sack1,sack2)
}

fn find_overlap(comp1:&[u8], comp2:&[u8]) -> Option<u8> {
    let mut map = HashMap::new();
    for item in comp1 {
        map.insert(item, 1);
    }

    for item in comp2 {
        if map.contains_key(item) {
            return Some(*item);
        }
    }
    None
}

fn find_overlap3(comp1:&[u8], comp2:&[u8], comp3:&[u8]) -> Option<u8> {
    let mut map = HashMap::new();
    for item in comp1 {
        map.insert(item, 1);
    }

    for item in comp2 {
        if map.contains_key(item) {
            map.insert(item, 2);
        }
    }
    
    for item in comp3 {
        if map.contains_key(item) && *map.get(item).unwrap() == 2 {
            map.insert(item, 3);
        }
    }

    let mut result = None;
    for (k,v) in map {
        if v >= 3{
            result = Some(*k);
        }

    }
    result

}

fn convert_to_priority(item:u8) -> i32 {
    if (b'a'..b'z'+1).contains(&item) {
        return item as i32 - 97 + 1; // -97 to 0 base the range +1 because problem definition
    } else if (b'A'..b'Z'+1).contains(&item) {
        return item as i32 - 65 + 27; // - 65 to 0 base the range +27 because problem said
    } else {
        println!("{}", item);
        panic!("Got something outside of Ascii 'a-zA-Z'");
    }
}

fn part1(input:&String)->i32{

    input
        .lines()
        .map(|line| split_sack(line))
        .map(|(comp1,comp2)| find_overlap(comp1, comp2))
        .map(|item| convert_to_priority(item.unwrap()))
        .sum()
}

// 2686 TOO HIGH okay I have a comparison bug somewhere.
fn part2(input:&String)->i32{
    input
        .lines()
        .tuples()
        .map(| (a,b,c)| find_overlap3(a.as_bytes(),b.as_bytes(),c.as_bytes()))
        .map(|item| convert_to_priority(item.unwrap()))
        .sum()
}
