use std::{fs::{read_to_string}, collections::HashMap};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{static ref RE_CD:Regex = Regex::new(r"\$ cd (.+)").unwrap();}
lazy_static!{static ref RE_LS:Regex = Regex::new(r"\$ ls").unwrap();}
lazy_static!{static ref RE_FILE:Regex = Regex::new(r"(\d+) (.+)").unwrap();}
lazy_static!{static ref RE_DIR:Regex = Regex::new(r"dir (.+)").unwrap();}


#[derive(Debug)]
enum TermLine {
    CD(String),
    LS,
    File(i32, String),
    Dir(String),
}

fn parse(input:&str) -> TermLine {
    let termline;

    if RE_CD.is_match(input) {
        let cd_cap = RE_CD.captures(input).unwrap();
        termline = TermLine::CD(cd_cap[1].to_string());

    } else if RE_LS.is_match(input) {
        termline = TermLine::LS;

    } else if RE_FILE.is_match(input) {
        let file_cap = RE_FILE.captures(input).unwrap();
        let size:i32 = file_cap[1].parse::<i32>().unwrap();
        termline = TermLine::File(size,file_cap[2].to_string());
        
    } else if RE_DIR.is_match(input) {
        let dir_cap = RE_DIR.captures(input).unwrap();
        termline = TermLine::Dir(dir_cap[1].to_string());

    } else {
        println!("{}",input);
        panic!("UNRECOGNIZED LINE TYPE");
    }

    termline
}

fn join_all(fs_context:&Vec<String>) -> String {
    let mut id = String::new();
    for part in fs_context.iter() {
        id.push_str(part);
        if part != "/" {
            id.push_str("/");
        }
    }
    id
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input)); 
}


fn build_dirs(input:&String) -> HashMap<String, i32> {

        // Context is what DIR you're in.
        let mut context:Vec<String> = Vec::new();
    
        let mut dirs:HashMap<String /* dirname */, i32 /* size */> = HashMap::new();
    
        for termline in input.lines().map(|line| parse(line)) {
            // println!("CMD{:?}", termline);
            match termline {
                TermLine::CD(name) => {
                    if name == ".." {
                        let child_dir = join_all(&context);
                        let child_size = dirs.get(&child_dir).unwrap();
    
                        context.pop();
    
                        let curr_dir = join_all(&context);
                        match dirs.get(&curr_dir) {
                            Some(curr_size) => dirs.insert(curr_dir, curr_size+child_size),
                            None => panic!(" SHOULD ALREADY HAVE SEEN {}", curr_dir),
                        };
    
                    } else {
                        // println!("INTO {}", join_one(&context, &name));
                        context.push(name);
                        dirs.insert(join_all(&context),0);
                    }
                },
                TermLine::LS => {/* WHO CARES? */},
    
                TermLine::File(size, _name) => {
                    let parent_dir = join_all(&context);
                    match dirs.get(&parent_dir) {
                        Some(parent_size) => dirs.insert(parent_dir, parent_size+size),
                        None => dirs.insert(parent_dir, size),
                    };
                    
                },
                TermLine::Dir(_name) => {/* WHO CARES? */ },
            }   
        }// FOR 
    
        // Walk up the context popping until root.
        while context.len() > 1 {
            
            let child_dir = join_all(&context);
            let child_size = dirs.get(&child_dir).unwrap();
    
            context.pop();
    
            let curr_dir = join_all(&context);
            match dirs.get(&curr_dir) {
                Some(curr_size) => dirs.insert(curr_dir, curr_size+child_size),
                None => panic!("SHOULD'VE already walked HERE"),
            };
    
        }

        dirs

}

fn part1(input:&String)->i32 {
    let dirs = build_dirs(input);

    dirs
        .iter()
        .filter(|(_k,v)| **v < 100000)
        .map(|(_k,v)| v)
        .sum()

}

fn part2(input:&String)->i32 {
    let dirs = build_dirs(input);
    const TOTAL_SPACE:i32 = 70000000;
    const NEEDED_SPACE:i32 = 30000000;
    let available_space:i32 = TOTAL_SPACE - *dirs.get("/").unwrap();
    let additional_needed = NEEDED_SPACE-available_space;
    
    let dir = dirs
        .iter()
        .filter(|(_k, v)| **v > additional_needed)
        .min_by_key(|(_k,v)| **v).unwrap();

    *dir.1
}