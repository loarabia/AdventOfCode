use std::fs::{read_to_string};
use std::str::FromStr;

fn main() {

    let filename = "input.txt";
    let input = read_to_string(filename).expect("Reading the file failed");

    let module_masses :Vec<i32> = input.lines()
        .map(|line| i32::from_str(line).expect("Bad line item")).collect();

    //Part 1
    let fuel_mass:i32 = module_masses.iter()
            .map( |mass| required_fuel(mass))
            .sum();    
    println!("{}", fuel_mass);
    
    // Part 2
    let modules_mass:i32 = module_masses.iter().sum();

    let launch_mass_plus_fuel:i32 = module_masses.iter().
        map(|mass| grossed_up_mass(mass))
        .sum();
    
    println!("{}", launch_mass_plus_fuel - modules_mass);
}

// Part 1
fn required_fuel(mass: &i32) -> i32 {
    mass/3-2
}

// Part 2
fn grossed_up_mass(mass: &i32) -> i32 {
    if *mass <= 0 {
        return 0;
    }
    mass + grossed_up_mass( &(mass/3-2) ) 
}