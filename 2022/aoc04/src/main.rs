use std::fs::{read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn break_into_tasks(line:&str) -> ((i32,i32), (i32,i32)) {
    let mut pair = line.split(',');
    
    let mut task1 = pair.next().unwrap().split('-');
    let beg1 = task1.next().unwrap().parse::<i32>().unwrap();
    let end1 = task1.next().unwrap().parse::<i32>().unwrap();

    let mut task2 = pair.next().unwrap().split('-');
    let beg2 = task2.next().unwrap().parse::<i32>().unwrap();
    let end2 = task2.next().unwrap().parse::<i32>().unwrap();

    ((beg1,end1),(beg2,end2))
}

fn count_subsets( beg1:i32, end1:i32, beg2:i32, end2:i32) -> i32 {

    if beg1 >= beg2 && end1 <= end2 {
        return 1;
    } else if beg2 >= beg1 && end2 <= end1 {
        return 1;
    } else {
        return 0;
    }
}

fn count_partial_subsets(beg1:i32, end1:i32, beg2:i32, end2:i32) -> i32 {
    if beg1 >= beg2 && beg1 <= end2 {
        return 1;
    } else if end1 >= beg2 && end1 <= end2 {
        return 1;
    } else if beg2 >= beg1 && beg2 <= end1 {
        return 1;
    } else if end2 >= beg1 && end2 <= end1 {
        return 1;
    } else {
        return 0;
    }
}


fn part1(input:&String)->i32{

    input
        .lines()
        .map( |line| break_into_tasks(line))
        .map( |tasks| count_subsets(tasks.0.0, tasks.0.1, tasks.1.0, tasks.1.1) )
        .sum()
}

// 2686 TOO HIGH okay I have a comparison bug somewhere.
fn part2(input:&String)->i32{
    input
        .lines()
        .map( |line| break_into_tasks(line))
        .map( |tasks| count_partial_subsets(tasks.0.0, tasks.0.1, tasks.1.0, tasks.1.1) )
        .sum()
}
