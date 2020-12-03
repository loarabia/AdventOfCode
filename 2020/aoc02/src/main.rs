use std::fs::{read_to_string};
use std::str::FromStr;
use std::option::Option;

extern crate regex;
use regex::Regex;

fn part1(input:& str) -> Option<i32> {
    
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<let>[[:alpha:]]): (?P<pw>[[:alpha:]]+)").unwrap();
    let mut valid_pw_count = 0;

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let min:i32 = cap.name("min").map_or(-1, |m| i32::from_str(m.as_str()).expect("Number Expected"));
        let max:i32 = cap.name("max").map_or(-1, |m| i32::from_str(m.as_str()).expect("Number Expected"));

        let ltr = cap.name("let").map_or(0, |m| m.as_str().as_bytes()[0] );
        let pw = cap.name("pw").map_or("", |m| m.as_str());
        
        let mut key_ltr_cnt = 0;

        for &letter in pw.as_bytes() {
            if letter == ltr {
                key_ltr_cnt += 1;
            }
        }

        if key_ltr_cnt >= min && key_ltr_cnt <= max {
            valid_pw_count += 1;
        }

    }
    Some(valid_pw_count)
}

fn part2(input:&str) -> Option<i32> {
    let re = Regex::new(r"(?P<idx1>\d+)-(?P<idx2>\d+) (?P<let>[[:alpha:]]): (?P<pw>[[:alpha:]]+)").unwrap();
    let mut valid_pw_count = 0;

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let idx1 = cap.name("idx1").map_or(0, |m| usize::from_str(m.as_str()).expect("Number Expected"));
        let idx2 = cap.name("idx2").map_or(0, |m| usize::from_str(m.as_str()).expect("Number Expected"));

        let ltr = cap.name("let").map_or(0, |m| m.as_str().as_bytes()[0] );
        let pw = cap.name("pw").map_or("", |m| m.as_str());
        
        //println!("{},{}", idx1, idx2);
        let p1 = pw.as_bytes()[idx1-1] == ltr;
        let p2 = pw.as_bytes()[idx2-1] == ltr;

        if p1 ^ p2  {
            valid_pw_count += 1;
        }

    }
    Some(valid_pw_count)

}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    println!("{}",part1(&input).unwrap());   
    println!("{}",part2(&input).unwrap());
}
