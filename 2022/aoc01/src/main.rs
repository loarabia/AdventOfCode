use std::fs::{read_to_string};
use std::str::FromStr;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    println!("Hello, world!");
}

fn part1(input: &String) -> i32 {
    input
        .split("\r\n\r\n")
        .map( |elf_pack| 
            elf_pack
                .lines()
                .map( |cals| 
                    cals.parse::<i32>().unwrap())
                .sum())
        .max().unwrap()
} 
fn part2(input: &String) -> i32 {
    let mut elf_data:Vec<i32> = 
    input
    .split("\r\n\r\n")
    .map( |elf_pack| 
        elf_pack
            .lines()
            .map( |cals| 
                cals.parse::<i32>().unwrap())
            .sum::<i32>()).collect();
    elf_data.sort();
    let slice = &elf_data[elf_data.len()-3..elf_data.len()];
    assert!(slice.len() == 3);
    slice.iter().sum()
    
}