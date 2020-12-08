use std::fs::{read_to_string};
use std::str::FromStr;
use std::option::Option;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

extern crate regex;
use regex::Regex;
use regex::RegexSet;

const ITEM_RE:&str = r"(?P<num>\d+) (?P<type>\w+ \w+) bag";
const TERM_RE:&str = r"no other bag";

fn part1(input: &str) -> Option<usize> {

    let re = Regex::new(r"(?P<head>\w+ \w+) bags contain (?P<tail>.*)$").unwrap();
    let item_re = Regex::new(ITEM_RE).unwrap();
    let term_re = Regex::new(TERM_RE).unwrap();
    let re_set = RegexSet::new(&[ITEM_RE,TERM_RE]).unwrap();

    let mut chld_prnt_map:HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {

        // Begin Parsing
        let c = re.captures(line)?;
        let parent = c.name("head")?.as_str();
        let tail = c.name("tail")?.as_str();

        // Handle the tail.
        // Tails come in a few forms
        //  ITEM        - "1 faded gold bag,"
        //  ITEMS       - "5 pale brown bags."
        //  TERMINAL    - "no other bags."
        //  LIST        - "2 light turquoise bags, 2 clear cyan bags, 4 dark cyan bags, 4 dotted orange bags."
        for item in tail.split(",") {
            let bags:Vec<_> = re_set.matches(item).into_iter().collect();
            
            if bags.len() > 1 {
                panic!("These Regexes shouldn't overlap. {:?}", bags);   
            }
            
            match bags[0] {
                0 => { 
                    let child = item_re.captures(item)?.name("type")?.as_str();
                    match chld_prnt_map.get_mut(child) {
                        Some(vals) => { 
                            vals.push(parent);        
                        },
                        None => {
                            let mut vals:Vec<&str> = Vec::new();
                            vals.push(parent);
                            chld_prnt_map.insert(child, vals);
                        },
                    };
                },
                1 => {  },
                _ => panic!("Impossible regexSet id: {}", bags[0]),

            } // match bags[0]
        } // for split(',')
    } // for lines

    // Walk the parents to find all that can hold a "shiny gold" bag
    let mut queue:Vec<&str> = Vec::new();
    let mut set:HashSet<&str> = HashSet::new();

    // queue the first items from shiny gold bag. Then we'll walk the tree.
    queue.append(chld_prnt_map.get_mut("shiny gold")?); 
    
    // Walk the tree.
    while let Some(node) = queue.pop() {
        // println!("ql:{} \t CURRENT NODE:{}", queue.len(), node);
        set.insert(node);
        if let Some(next_nodes) = chld_prnt_map.get_mut(node) {
            queue.append(next_nodes);
        }
        // println!("\t\t NEXT NODES: {:?}", next_nodes);
    } 

    Some(set.len())
}

fn part2(input: &str) -> Option<usize> {
    let re = Regex::new(r"(?P<head>\w+ \w+) bags contain (?P<tail>.*)$").unwrap();
    let item_re = Regex::new(ITEM_RE).unwrap();
    let term_re = Regex::new(TERM_RE).unwrap();
    let re_set = RegexSet::new(&[ITEM_RE,TERM_RE]).unwrap();

    let mut parent_child_map:HashMap<&str, Vec<(&str, u32)>> = HashMap::new();

    for line in input.lines() {

        // Begin Parsing
        let c = re.captures(line)?;
        let parent = c.name("head")?.as_str();
        let tail = c.name("tail")?.as_str();

        // Handle the tail.
        // Tails come in a few forms
        //  ITEM        - "1 faded gold bag,"
        //  ITEMS       - "5 pale brown bags."
        //  TERMINAL    - "no other bags."
        //  LIST        - "2 light turquoise bags, 2 clear cyan bags, 4 dark cyan bags, 4 dotted orange bags."
        for item in tail.split(",") {
            let bags:Vec<_> = re_set.matches(item).into_iter().collect();
            
            if bags.len() > 1 {
                panic!("These Regexes shouldn't overlap. {:?}", bags);   
            }
            
            match bags[0] {
                0 => { 
                    let caps = item_re.captures(item)?;
                    
                    let num = u32::from_str(caps.name("num")?.as_str()).unwrap();
                    let child = caps.name("type")?.as_str();

                    match parent_child_map.get_mut(parent) {
                        Some(vals) => { 
                            vals.push((child, num));        
                        },
                        None => {
                            let mut vals:Vec<(&str, u32)> = Vec::new();
                            vals.push((child,num));
                            parent_child_map.insert(parent, vals);
                        },
                    };
                },
                1 => {  },
                _ => panic!("Impossible regexSet id: {}", bags[0]),

            } // match bags[0]
        } // for split(',')
    } // for lines

    // Walk the parents to find all that can hold a "shiny gold" bag
    let mut stack:Vec<(&str, u32)> = Vec::new();
    let mut result = 0;
    let mut multiplier = 1;

    // queue the first items from shiny gold bag and add the bags. Then we'll walk the tree.
    for (node, num) in parent_child_map.get_mut("shiny gold")? {
        // println!("Node:{} Num:{}", node, num); 
        result = result + (*num * multiplier); 
        stack.push( (node, *num * multiplier));
    } 
    
    // Walk the tree.
    while let Some( (node, mult) ) = stack.pop() {

        println!("Node:{} Num:{}", node, mult); 

        if let Some(next_nodes) = parent_child_map.get_mut(node) {
            for (child, cnt) in next_nodes {
                // println!("\t\t Node:{} Num:{} Mult:{}", child, cnt, mult);
                result = result + (*cnt * mult);
                stack.push( (child, *cnt * mult) );
            }
        } 
    } 
    // 145 too low
    Some(result as usize)
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    println!("{}",part1(&input).unwrap());   
    println!("{}",part2(&input).unwrap());
}
