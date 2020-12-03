use std::fs::{read_to_string};
use std::str::FromStr;
use std::vec::Vec;
use std::option::Option;


fn part1(expenses: &Vec<i32>) -> Option<i32> {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return Some(expenses[i] * expenses[j])
            }
        }
    }
    None
}

fn part2(expenses: &Vec<i32>) -> Option<i32> {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            for k in j..expenses.len() {
               if expenses[i] + expenses[j] + expenses[k] == 2020 {
                   println!("{}-{}-{}",i,j,k);
                    return Some(expenses[i] * expenses[j] * expenses[k])
                }
            }
        }
    }
    None
}

fn main() {
    let input1 = read_to_string("input1.txt").expect("Couldn't read input1.txt");

    let expenses: Vec<i32> = 
        input1
            .lines()
            .map(|line| i32::from_str(line).expect("Bad line item"))
            .collect();

    let result1 = part1(&expenses).expect("Couldn't find an error");
    println!("{}",result1);

    let result2 = part2(&expenses).expect("Couldn't find an error");
    println!("{}",result2);

}
