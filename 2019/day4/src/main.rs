use std::assert;

fn main() {
    let min = 271973;
    let max = 785961;

    run_part1(min, max);
    run_part2(min, max);
}

fn run_part1(min:i32, max:i32) 
{
    let mut count = 0;
    for value in min..max{
        let strval = value.to_string();
        if has_eq_adjacents(&strval) && are_digits_increasing(&strval) {
            count += 1;
        }
    }
    println!("pt1:{}",count);
}

fn run_part2(min:i32, max:i32) 
{
    let mut count = 0;
    for value in min..max{
        let strval = value.to_string();
        if contains_double(&strval) && are_digits_increasing(&strval) {
            count += 1;
        }
    }
    println!("pt2:{}",count);
}

fn contains_double(value:&str) -> bool {
    let mut count = 1;
    // Adding NULL which won't match anything to the end of the byte slice 
    // so that I don't have to do an extra comparison outside the loop for the 
    // last two digits.
    for pair in [value.as_bytes(), &[0]].concat().windows(2) {
        if pair[0] != pair[1] && count == 2 {
            return true
        } else if pair[0] != pair[1] {
            count = 1;
        } else {
            count += 1;
        }
    }
    false
}

fn has_eq_adjacents(value:&str) -> bool {
    assert!(value.bytes().len() == 6);
 
    for pair in value.as_bytes().windows(2) {
        if pair[0] == pair[1] {
            return true
        }
    }
    return false     
}

fn are_digits_increasing(value:&str) -> bool {
    assert!(value.bytes().len() == 6);

    for pair in value.as_bytes().windows(2) {
        if pair[0] > pair [1] {
            return false
        }
    }
    return true
}
