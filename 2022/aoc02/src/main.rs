use core::panic;
use std::fs::{read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn round_to_score(opp_choice:Choice, player_choice:Choice) -> i32 {
    let mut score:i32 = 0;
    score += match did_player_win(&opp_choice, &player_choice) {
        None => 3,
        Some(true)=> 6,
        Some(false)=> 0,
    };

    score + player_choice as i32
}

fn line_to_score(line:&str) -> i32 {
    let guide_line = line.as_bytes();
    let opp_choice = read_opp_choice(guide_line);

    let player_choice = match guide_line[2] {
        b'X' => Choice::Rock,
        b'Y' => Choice::Paper,
        b'Z' => Choice::Scissors,
        _ => panic!("NOT POSSIBLE"),
    };

    round_to_score(opp_choice, player_choice)
}

fn did_player_win(opp_choice:&Choice, player_choice:&Choice) -> Option<bool> {
    match (opp_choice, player_choice) {
        (Choice::Rock, Choice::Rock) => None,
        (Choice::Paper, Choice::Paper) => None,
        (Choice::Scissors, Choice::Scissors) => None,
        (Choice::Rock, Choice::Scissors) => Some(false),
        (Choice::Paper, Choice::Rock) => Some(false),
        (Choice::Scissors, Choice::Paper) => Some(false),
        _ => Some (true),
     }
}

fn satisfy_roud(opp_choice:&Choice, player_wins:Option<bool>) -> Choice {
    match (opp_choice, player_wins) {
        (Choice::Rock, Some(true)) => Choice::Paper,
        (Choice::Paper, Some(true)) => Choice::Scissors,
        (Choice::Scissors, Some(true)) => Choice::Rock,
        (Choice::Rock, Some(false)) => Choice::Scissors,
        (Choice::Paper, Some(false)) => Choice::Rock,
        (Choice::Scissors, Some(false)) => Choice::Paper,
        (Choice::Rock, None) => Choice::Rock,
        (Choice::Paper, None) => Choice::Paper,
        (Choice::Scissors, None) => Choice::Scissors,
    }     
}

fn read_opp_choice(line:&[u8]) -> Choice {
    match line[0] {
        b'A' => Choice::Rock,
        b'B' => Choice::Paper,
        b'C' => Choice::Scissors,
        _ => panic!("NOT POSSIBLE"),
    }
}

fn line_to_score_tweaked(line:&str) -> i32 {
    let guide_line = line.as_bytes();
    let opp_choice = read_opp_choice(guide_line);

    let should_player_win = match guide_line[2] {
        b'X' => Some(false),
        b'Y' => None,
        b'Z' => Some(true),
        _ => panic!("NOT POSSIBLE"),
    };

    let player_choice = satisfy_roud(&opp_choice, should_player_win);

    round_to_score(opp_choice, player_choice)
}

#[derive(Copy,Clone)]
enum Choice {
    Rock=1,
    Paper=2,
    Scissors=3,
}

fn part1(input:&String)->i32{
    input
        .lines()
        .map(|line| line_to_score(line))
        .sum()
}
fn part2(input:&String)->i32{
    input
        .lines()
        .map(|line| line_to_score_tweaked(line))
        .sum()
}
