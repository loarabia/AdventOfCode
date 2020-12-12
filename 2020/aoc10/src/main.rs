use std::fs::read_to_string;
use std::str::FromStr;
// use std::iter;
use std::vec::Vec;
use std::collections::HashMap;

// Input Joltage is 1,2,3 jolts lower than rated output Joltage
// Device is exactly 3 jolts higher than my max adapater
fn part1(values:&Vec<u32>) -> Option<u32> {
    // Do a custom sort keeping track of the 1 jolt diffs and 3 jolt diffs.
    let mut counter_1s:u32 = 0;
    let mut counter_3s:u32 = 0;

    for pair in values.windows(2) {
        if pair[1] - pair[0] == 1 { counter_1s += 1; }
        else if pair[1] - pair[0] == 3 { counter_3s += 1; }
    }   

    // Check the start
    // if values[0] == 1 { counter_1s +=1; }
    // else if values[0] == 3 { counter_3s +=1; }

    // // The end is by definition 3 jolts higher than max so add it
    // counter_3s += 1;

    println!("1s: {}, 3s: {}", counter_1s, counter_3s);
    Some(counter_1s * counter_3s)
}

fn part2(values:&Vec<u32>) -> Option<u64> {
    let mut counts:HashMap<u32,u64> = HashMap::new();
    counts.insert(0,1);
    for value in values {
        let curr_cnt = counts.get(value).unwrap().clone();
        for i in 1..=3 {
            match counts.get( &(value+i)) {
                Some(num) => counts.insert(value+i, curr_cnt + num),
                None => counts.insert(value+i, curr_cnt),
            };
        }
        // println!("{:?}", counts);
    }

    // for value in values {
        // let curr_cnt = counts.get(value).unwrap().clone();
        // counts.insert(value+1, curr_cnt + 1);
        // counts.insert(value+2, curr_cnt + 1);
        // counts.insert(value+3, curr_cnt + 1);
        // println!("{:?}",counts)
    // }
    let idx = values.len()-1;
    let final_val = values[idx];
    let result = counts.get(&final_val).unwrap();
    Some(*result)
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let input_iter = input.lines().map(|val| u32::from_str(val).unwrap());

    let mut values:Vec<u32> = Vec::new();
    values.push(0);
    values.append(&mut input_iter.collect::<Vec<u32>>());
    values.sort_unstable();
    values.push( values.last().unwrap() + 3 );

    
    let number = part1(&values).unwrap();
    println!("{}",number);
    
    println!("{}",part2(&values).unwrap());
}
