use std::fs::read_to_string;
use std::str::FromStr;
use std::ops::RangeInclusive;
use std::collections::HashMap;

fn read_rule( line:&str) -> (&str , RangeInclusive<u32>, RangeInclusive<u32>) {
    let name = line.split(':').nth(0).unwrap();
    let p1_min = line.split(':').nth(1).unwrap().split("or").nth(0).unwrap().split('-').nth(0).unwrap(); 
    let p1_max = line.split(':').nth(1).unwrap().split("or").nth(0).unwrap().split('-').nth(1).unwrap(); 
    let p2_min = line.split(':').nth(1).unwrap().split("or").nth(1).unwrap().split('-').nth(0).unwrap(); 
    let p2_max = line.split(':').nth(1).unwrap().split("or").nth(1).unwrap().split('-').nth(1).unwrap(); 

    (name,
        (u32::from_str(p1_min.trim()).unwrap()..=u32::from_str(p1_max.trim()).unwrap()),
        (u32::from_str(p2_min.trim()).unwrap()..=u32::from_str(p2_max.trim()).unwrap())
    )
}

fn read_ticket(line: &str) -> Vec<u32> {

    let mut ticket_info = Vec::new();

    for number in line.split(',') {
        ticket_info.push(u32::from_str(number).unwrap());
    }
    ticket_info
}

fn find_invalid_values(rules:&Vec<RangeInclusive<u32>>, ticket:&Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    for info in ticket {
        if is_invalid_value(rules, info) {
            result.push(*info);
        }
    }
    result
}

fn is_invalid_value(rules:&Vec<RangeInclusive<u32>>, ticket_info:&u32) -> bool {
    !rules.iter().any( |rng| rng.contains(ticket_info))
}

fn is_invalid_ticket(rules:&Vec<RangeInclusive<u32>>, ticket:&Vec<u32>) -> bool {
    ticket.iter().any(|info| is_invalid_value(rules, info))
}

fn part1(input:&str) -> u32 {
    let rules_input = input.split("\r\n\r\n").nth(0).unwrap();
    let _santa_ticket = input.split("\r\n\r\n").nth(1).unwrap();
    let nearby_tickets = input.split("\r\n\r\n").nth(2).unwrap();

    // Build Rules
    let mut rules:Vec<RangeInclusive<u32>> = Vec::new();
    for line in rules_input.lines() {
        let (_name, r1, r2) = read_rule(line);
        rules.push(r1);
        rules.push(r2);
    }

    let mut sum:u32 = 0;
    // Read Nearby Tickets
    for line in nearby_tickets.lines().skip(1) {

        let ticket_values = read_ticket(line);
        sum += find_invalid_values(&rules, &ticket_values).iter().sum::<u32>();
    }

    sum
}

fn part2(input:&str) -> u64 {
    let rules_input = input.split("\r\n\r\n").nth(0).unwrap();
    let santa_ticket_info = input.split("\r\n\r\n").nth(1).unwrap();
    let nearby_tickets = input.split("\r\n\r\n").nth(2).unwrap();

    let mut rules:HashMap<&str, Vec<RangeInclusive<u32>>> = HashMap::new();
    let mut rules_v:Vec<RangeInclusive<u32>> = Vec::new(); // Feeling lazy
    for line in rules_input.lines() {
        let (name, r1, r2) = read_rule(line);
        rules.insert(name, vec![r1.clone(),r2.clone()]);
        rules_v.push(r1);
        rules_v.push(r2);
    }

    // Read Nearby Tickets
    let mut valid_tickets:Vec<Vec<u32>> = Vec::new();
    let mut ticket_len = 0;
    for line in nearby_tickets.lines().skip(1) {

        let ticket_values = read_ticket(line);
        ticket_len = ticket_values.len(); // Yup lots of silly extra work

        // Double negative == if is valid ticket
        if !is_invalid_ticket(&rules_v, &ticket_values) {
            valid_tickets.push(ticket_values);
        }
    }

    let mut ticket_field_candidates:Vec<Vec<&str>> = Vec::new();
    ticket_field_candidates.resize(ticket_len, Vec::new());
    // Go through all the tickets column by column.
    // If all values in the column match a rule, assume it is a candidate for the field.
    for (rule_name, rule_set) in rules {
        for i in 0..ticket_len {
            let all_valid = !valid_tickets.iter().any(|val| is_invalid_value(&rule_set, &val[i]));
            if all_valid {
                ticket_field_candidates[i].push(rule_name);
            }
        }
    }
    // println!("{:?}", ticket_field_candidates);

    let mut ticket_field:Vec<&str> = Vec::new();
    ticket_field.resize(ticket_len, "");
    for _i in 0..ticket_len {
        let mut field = "";
        for (i, candidates) in ticket_field_candidates.iter().enumerate() {
            if candidates.len() == 1 {
                ticket_field[i] = candidates[0];
                field = candidates[0];
            }
        }

        for candidates in ticket_field_candidates.as_mut_slice() {
            if let Some(i) = candidates.iter().position(|cand| *cand == field) {
                candidates.remove(i);   
            }
        }
    }

    // println!("{:?}", ticket_field);

    let mut santa_value:u64 = 1;
    let santa_ticket = read_ticket(santa_ticket_info.split("\r\n").nth(1).unwrap());
    for (i,field) in ticket_field.iter().enumerate() {
        if field.starts_with("departure") {
            santa_value *= santa_ticket[i] as u64;
        }
    }

    santa_value
}

fn main() {
    let filename = "input.txt";
    let input = read_to_string(filename).expect(&format!("Couldn't read file: {}", filename));
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
