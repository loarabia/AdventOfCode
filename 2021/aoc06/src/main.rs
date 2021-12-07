use std::fs::{read_to_string};
use std::str::FromStr;

fn part(groups:&mut Vec<u64>, iters:u64) -> u64 {
    for _i in 0..iters {
        let mut temp = 0;
        // println!("{:?}", groups);
        for age in (0..groups.len()).rev() {
            if age == 0 {
                groups[8] = temp;
                groups[6] += temp;
            } else if age == 8 {
                temp = groups[age-1];
                groups[age-1] = groups[age];
            } else {
                let temp2 = groups[age-1];
                groups[age-1] = temp;
                temp = temp2;
            }
        }
    }
    groups.iter().sum()
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");

    let mut groups = vec![0;9];
    for num in input.split(',' ) {
        let age = usize::from_str(num).unwrap();
        groups[age] += 1;
    }
    println!("{}", part(&mut groups.clone(),80));
    println!("{}", part(&mut groups,256));
}
