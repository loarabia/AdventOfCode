use std::fs::{read_to_string}; 

fn is_positive(num:i32) -> bool {
    if num > 0 { true } else { false }
}

fn part1(input:&String) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    // The text we'll derive gamma and epsilon from
    let mut vec:Vec<i32> = vec![0; input.lines().peekable().peek().unwrap().len()];
    for line in input.lines(){
        for (i,val) in line.chars().enumerate() {
            match val {
                '0' => vec[i] -= 1,
                '1' => vec[i] += 1,
                _ => panic!("Unexpected input"),
            }
        }
    }
    //println!("{:?}", vec);

    for (i, digit) in vec.iter().rev().enumerate() {
        match is_positive(*digit) {
            true =>  gamma |= 1<<i,
            false =>  epsilon |= 1 <<i,
        }
    }

    gamma * epsilon
}

fn split_on_place(mut readings:Vec<&str>, place:usize) -> (Vec<&str>, Vec<&str>) {
    let mut zeros = Vec::new();
    let mut ones = Vec::new();
    while let Some(reading) = readings.pop() {
        match reading.chars().nth(place).unwrap() {
            '0' => zeros.push(reading),
            '1' => ones.push(reading),
            _ => panic!("Unknown Char")
        }
    }

    (zeros, ones)
}

// 2013264 (TOO LOW)
fn part2(input: &String) -> u32 {
    let o2_rating:u32;
    let co2_rating:u32;

    // Peek at the first line, how long are the numbers (12 bits or 5 bits?)
    let bit_len = input.lines().peekable().peek().unwrap().len();

    // Split into 1st subsets
    let (cand1, cand2) = split_on_place(input.lines().collect(), 0);

    let mut co2_cands;
    let mut o2_cands;
    
    if cand1.len() > cand2.len() { 
        o2_cands = cand1;
        co2_cands = cand2;
    } else {
        o2_cands = cand2;
        co2_cands = cand1;
    };

    // Find the o2 candidate
    for place in 1..bit_len {
        if o2_cands.len() == 1 { break; }
        let (z1s, o1s) = split_on_place(o2_cands,place);
        if o1s.len() > z1s.len() {
            o2_cands = o1s;
        } else if o1s.len() < z1s.len() {
            o2_cands = z1s;
        } else {
            // Equal
            o2_cands = o1s;
        }
    }
    o2_rating = u32::from_str_radix(o2_cands[0],2).unwrap();

    // Find the co2 candidate
    for place in 1..bit_len {
        if co2_cands.len() == 1 {break; };

        let (z1s, o1s) = split_on_place(co2_cands,place);
        if z1s.len() > o1s.len() {
            co2_cands = o1s;
        } else if z1s.len() < o1s.len() {
            co2_cands = z1s;
        } else {
            // Equal
            co2_cands = z1s;
        }
    }
    co2_rating = u32::from_str_radix(co2_cands[0],2).unwrap();

    o2_rating * co2_rating
}
 

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    println!("{}",part1(&input));
    println!("{}",part2(&input));
}