use std::fs::{read_to_string};
use std::str::FromStr;

fn part1(crab_locs:&Vec<i32>) -> i32 {
    let min = crab_locs.iter().min().unwrap();
    let max = crab_locs.iter().max().unwrap();
    let mut fuel_ratings:Vec<i32> = Vec::new();
    for loc in *min..=*max {
        let fuel_rating = crab_locs.iter().map(|c_loc| (c_loc - loc).abs()).sum();
        fuel_ratings.push(fuel_rating)
    }
    *fuel_ratings.iter().min().unwrap()
}

fn tri(i:i32) -> i32 { i*(i+1)/2 }

fn crab_fuel_cost(start:&i32, dest:&i32) -> i32 {
    let dist = (start-dest).abs();
    tri(dist)
}

fn part2(crab_locs:&Vec<i32>) -> i32 {
    let min = crab_locs.iter().min().unwrap();
    let max = crab_locs.iter().max().unwrap();
    let mut fuel_ratings:Vec<i32> = Vec::new();
    for loc in *min..=*max {
        let fuel_rating = 
            crab_locs
                .iter()
                .map(|c_loc| crab_fuel_cost(c_loc, &loc))
                .sum();
        fuel_ratings.push(fuel_rating)
    }
    *fuel_ratings.iter().min().unwrap()
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let locs: Vec<i32> = input.lines().next().unwrap().split(",").map(|i| i32::from_str(i).unwrap()).collect();

    println!("{}",part1(&locs));
    println!("{}",part2(&locs));
}