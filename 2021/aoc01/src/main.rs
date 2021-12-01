use std::fs::{read_to_string};
use std::str::FromStr;

fn part1(depths: &Vec<i32>) -> i32 {
    depths.windows(2).fold(0, |acc, pair| { if pair[1] > pair[0] { return acc + 1} else { return acc}})
}

fn part2(depths: &Vec<i32>) -> i32 {
    depths
        .windows(3)
        .map(|tri| tri[0]+tri[1]+tri[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |acc, pair| { if pair[1] > pair[0] {return acc + 1 } else { return acc}})
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let depths: Vec<i32> = 
    input
        .lines()
        .map(|line| i32::from_str(line).expect("Bad line item"))
        .collect();

    println!("{}",part1(&depths));
    println!("{}",part2(&depths));
}

// 397 too low