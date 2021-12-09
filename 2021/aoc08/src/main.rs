use std::collections::HashSet;
use std::fs::read_to_string;

fn reading(line: &str) -> (&str, &str) {
    let parts: Vec<&str> = line.split('|').collect();
    (parts[0], parts[1])
}

fn part1(readings: &Vec<(&str, &str)>) -> u32 {
    let mut cnt = 0;
    let outputs = readings.iter().map(|r| r.1);
    for output in outputs {
        cnt += output
            .split_ascii_whitespace()
            .filter(|i| i.len() != 6 && i.len() != 5)
            .count();
    }
    cnt as u32
}

fn make_decoder<'a>(input: &'a str) -> Vec<&'a str> {
    let mut decoder = vec![""; 10];

    let mut l5_candidates = Vec::new();
    let mut l6_candidates = Vec::new();

    // Get the easy ones out of the way
    for digit in input.split_ascii_whitespace() {
        if digit.len() == 2 {
            decoder[1] = digit
        } else if digit.len() == 4 {
            decoder[4] = digit
        } else if digit.len() == 3 {
            decoder[7] = digit
        } else if digit.len() == 7 {
            decoder[8] = digit
        } else if digit.len() == 5 {
            l5_candidates.push(digit)
        } else if digit.len() == 6 {
            l6_candidates.push(digit)
        } else {
            panic!("Impossible digit length!")
        }
    }

    let set7: HashSet<u8> = HashSet::from_iter(decoder[7].bytes());

    // 3  is "easy" its a superset of 7 where 2 and 5 are not 7 int 5 == 2 and 7 int 3 == 2
    // but it doesn't tell us any more about which is top_r or bot_r
    // it does tell us which is mid and bot candidats
    for (i, digit) in l5_candidates.iter().enumerate() {
        let digit_set = HashSet::from_iter(digit.bytes());
        if digit_set.intersection(&set7).count() == 3 {
            decoder[3] = digit;
            l5_candidates.remove(i);
            break;
        }
    }

    // have already correct 1,3,4,7,8
    let set8: HashSet<u8> = HashSet::from_iter(decoder[8].bytes());
    let set7: HashSet<u8> = HashSet::from_iter(decoder[7].bytes());

    // Looking for 6
    let set8minus7: HashSet<u8> = HashSet::from_iter(set8.difference(&set7).map(|i| *i));
    assert!(set8minus7.iter().count() == 4);

    let set7minus8: HashSet<u8> = HashSet::from_iter(set7.difference(&set8).map(|i| *i));
    assert_eq!(set7minus8.iter().count(), 0);

    for (i, l6_cand) in l6_candidates.iter().enumerate() {
        let l6_cand_set: HashSet<u8> = HashSet::from_iter(l6_cand.bytes());

        if set8minus7.difference(&l6_cand_set).count() == 0 {
            decoder[6] = l6_cand;
            l6_candidates.remove(i);
            break;
        }
    }
    assert_eq!(l6_candidates.len(), 2);

    // Now that 6 is gone discover 0 and 9 by subtracting 4. 9-4 = 2 0-4 = 3
    let set4: HashSet<u8> = HashSet::from_iter(decoder[4].bytes());
    for (i, l6_cand) in l6_candidates.iter().enumerate() {
        let l6_cand_set: HashSet<u8> = HashSet::from_iter(l6_cand.bytes());
        if l6_cand_set.difference(&set4).count() == 2 {
            decoder[9] = l6_cand;
            l6_candidates.remove(i);
            break;
        }
    }

    decoder[0] = l6_candidates[0];

    // Numbers with 5 panels turned on 2 and 5 remain since we know 3
    // Now that we know 6, the difference between 5 and 6 is 1 but the diff between 6 and 2 is 2
    let set6: HashSet<u8> = HashSet::from_iter(decoder[6].bytes());
    let e1: HashSet<u8> = HashSet::from_iter(l5_candidates[0].bytes());

    if set6.difference(&e1).count() == 1 {
        decoder[5] = l5_candidates[0];
        decoder[2] = l5_candidates[1];
    } else {
        decoder[5] = l5_candidates[1];
        decoder[2] = l5_candidates[0];
    }

    decoder
}

fn decode(decoder: &Vec<&str>, digits: &str) -> u32 {
    let mut result: u32 = 0;
    let num_digits: u32 = digits.split_ascii_whitespace().count() as u32 - 1;
    for (i, digit) in digits.split_ascii_whitespace().enumerate() {
        let mut n: u32 = 0;
        let digit_set: HashSet<u8> = HashSet::from_iter(digit.bytes());

        for (num, code) in decoder.iter().enumerate() {
            let code_set: HashSet<u8> = HashSet::from_iter(code.bytes());
            if code_set == digit_set {
                n = num as u32;
                break;
            }
        }
        result += n * 10u32.pow(num_digits - i as u32);
    }
    result
}

fn part2(readings: &Vec<(&str, &str)>) -> u32 {
    let mut result = 0;
    for r in readings {
        let decoder = make_decoder(r.0);
        result += decode(&decoder, r.1);
    }
    result
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input file");
    let readings: Vec<(&str, &str)> = input.lines().map(|line| reading(line)).collect();
    println!("{}", part1(&readings));
    println!("{}", part2(&readings));
}
