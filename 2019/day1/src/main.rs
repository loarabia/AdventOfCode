use std::fs::{read_to_string};
use std::str::FromStr;

fn main() {

    let filename = "input.txt";
    let input = read_to_string(filename).expect("Reading the file failed");

    //Part 1
    let launch_mass_plus_fuel:i32 = input.lines()
            .map(|line| required_fuel(i32::from_str(line).expect("Bad line item")))
            .sum();
    
    println!("{}", launch_mass_plus_fuel);
    
    // Part 2
    let module_mass:i32 = input.lines()
            .map(|line| i32::from_str(line).expect("Bad line item"))
            .sum();

    let launch_mass_plus_fuel:i32 = input.lines()
            .map(|line| grossed_up_mass(i32::from_str(line).expect("Bad line item")))
            .sum();
    
    println!("{}", launch_mass_plus_fuel - module_mass);
}

// Part 1
fn required_fuel(mass: i32) -> i32 {
    mass/3-2
}

// Part 2
fn grossed_up_mass(mass: i32) -> i32 {
    if mass <= 0 {
        return 0;
    }
    mass + grossed_up_mass(mass/3-2) 
}