use std::fs::read_to_string;
use std::str::FromStr;

fn part1(values:& Vec<u64>) -> Option<u64> {
    let mut win_start = 0;
    let mut win_end = 25;
    
    let mut result:Option<u64> = None;
    for idx in 25..values.len() {
        let mut found = false;
        for i in win_start..win_end {
            for j in i..win_end {
                if values[idx] == values[i] + values[j] {
                    found |= true;
                }
            }
        }

        if !found {
            result = Some(values[idx]);
            break;
        }

        win_start += 1;
        win_end += 1;
    }
    
    result
}

fn part2(values:& Vec<u64>, target:u64) -> Option<u64> {
    let mut start = 0;
    let mut end = 1;

    'outer : loop {
        for i in start..end {
            let sum:u64 = values[start..=end].iter().sum();

            if sum < target {
                end += 1;
            } else if sum > target {
                start += 1;
                end = start + 1;
            } else {
                break 'outer;
            }
        }
    }
    let min = values[start..=end].iter().min().unwrap();
    let max = values[start..=end].iter().max().unwrap();
    Some( min + max )
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let values = input.lines().map(|val| u64::from_str(val).unwrap()).collect();

    let number = part1(&values).unwrap();
    println!("{}",number);
    println!("{}",part2(&values, number).unwrap());
}
