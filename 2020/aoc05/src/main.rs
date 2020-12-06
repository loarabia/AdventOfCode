use std::fs::{read_to_string};
use std::vec::Vec;


fn read_bin(input: &str, beg:usize, end:usize, zero:&str, one:&str) -> u32 {
    let bin_str = input[beg..end].replace(zero, "0").replace(one,"1");
    u32::from_str_radix(&bin_str, 2).unwrap()
}

fn read_seat_id(seat_desc: &str) -> u32 {
    let row_id = read_bin(seat_desc,0,7,"F","B" );
    let col_id = read_bin(seat_desc,7,10,"L","R" );
    
    row_id * 8 + col_id
}

fn part1(input: &str) -> u32 {
    input.lines().map(|seat_desc| read_seat_id(seat_desc) ).max().unwrap()
}

fn part2(input: &str) -> Option<u32> {
    // Find Seat IDs
    let mut ids:Vec<u32> = input.lines().map(|seat_desc| read_seat_id(seat_desc)).collect();
    // Sort them
    ids.sort();
    
    let mut prev_id = ids[0];

    for curr_id in &ids[1..ids.len()-1] {
        if curr_id - prev_id > 1 {
            return Some(curr_id - 1)
        }
        prev_id = *curr_id;
    }
    None
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input).unwrap());
}
