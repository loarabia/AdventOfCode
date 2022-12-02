use std::fs::{read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn pack_to_cals(pack:&str) -> i32 {
    pack
        .lines()
        .map( |cals| cals.parse::<i32>().unwrap())
        .sum()
}

fn part1(input:&String) -> i32 {
    input
        .split("\r\n\r\n")
        .map(|elf_pack| pack_to_cals(elf_pack))
        .max().unwrap()
}

fn part2(input: &String) -> i32 {
    let mut elf_data:Vec<i32> =
        input
            .split("\r\n\r\n")
            .map(|elf_pack| pack_to_cals(elf_pack))
            .collect();
    elf_data.sort();
    elf_data[elf_data.len()-3..elf_data.len()].iter().sum()
}