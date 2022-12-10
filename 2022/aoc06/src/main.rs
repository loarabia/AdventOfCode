use std::fs::{read_to_string};
use std::collections::HashMap;


fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input)); 
}

fn part1(input:&String)->usize {
    const WIN_SIZE:usize = 4;
    let mut unique_four_iter = input
        .as_bytes()
        .windows(WIN_SIZE)
        .map(|w| (w[0], w[1], w[2], w[3]))
        .enumerate()
        .filter(| (_index, (a, b, c, d)) | a!=b && a != c && a != d && b != c && b != d && c != d);
    let pat = unique_four_iter.next().unwrap();
    //String::from_utf8(vec![pat.1.0,pat.1.1,pat.1.2,pat.1.3]).unwrap()
    pat.0 + WIN_SIZE
}

fn part2(input:&String)->usize
{
    const WIN_SIZE:usize = 14;
    let mut unique_fourteen_iter = input
        .as_bytes()
        .windows(WIN_SIZE)
        .enumerate()
        .filter(| (_i, window)| {
            let mut map = HashMap::new();
            for num in window.iter() {
                if map.contains_key(num) {
                    return false;
                } else {
                    map.insert(num,1);
                }
            }
            return true;
        });
    
    let pat = unique_fourteen_iter.next().unwrap();
    pat.0 + WIN_SIZE
}