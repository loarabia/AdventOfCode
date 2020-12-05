use std::fs::{read_to_string};
use std::str::FromStr;
use std::option::Option;

extern crate regex;
use regex::Regex;


struct GovId<'a> {
    byr:Option<u32>,
    iyr:Option<u32>,
    eyr:Option<u32>,
    hgt:Option<Box<&'a str>>,
    hcl:Option<Box<&'a str>>,
    ecl:Option<Box<&'a str>>,
    pid:Option<Box<&'a str>>,
    cid:Option<u32>,
}

fn unpack_u32(re:&regex::Regex, input:&str) ->Option<u32> {
    let caps = re.captures(input);
    let val = match caps {
        Some(cap) => {
            Some(u32::from_str(cap.get(1).unwrap().as_str()).expect("wanted u32 not num?!?"))
        },
        None => None,
    };
    val
}

fn unpack_str<'a>(re:&regex::Regex, input:&'a str) -> Option<Box<&'a str>> {
    let caps = re.captures(input);
    let val = match caps {
        Some(cap) => Some(Box::new(cap.get(1).unwrap().as_str())),
        None => None,
    };
    val
}

fn has_valid_byr(input:&str) -> bool {
    let byr_re:regex::Regex = Regex::new(r"byr:(\d{4})").unwrap();
    match unpack_u32(&byr_re, input) {
        Some(val) => {
            if val >= 1920 && val <= 2002 { true } else { false }
        },
        None => false
    }
}

fn has_valid_iyr(input:&str) -> bool {
    let iyr_re:regex::Regex = Regex::new(r"iyr:(\d{4})").unwrap();
    match unpack_u32(&iyr_re, input) {
        Some(val) => {
            if val >= 2010 && val <= 2020 { true } else { false }
        },
        None => false
    }
}

fn has_valid_eyr(input:&str) -> bool {
    let eyr_re:regex::Regex = Regex::new(r"eyr:(\d{4})").unwrap();
    match unpack_u32(&eyr_re, input) {
        Some(val) => {
            if val >= 2020 && val <= 2030 { true } else { false }
        },
        None => false
    }
}

fn has_valid_hgt(input:&str) -> bool {
    let hgt_re:regex::Regex = Regex::new(r"hgt:(\d+)(cm|in)").unwrap();
    let caps = hgt_re.captures(input);
    let mut units = "";
    let mut val = 0;
    let mut cap_str = "";
    let res = match caps {
        Some(cap) => {
            cap_str = cap.get(0).unwrap().as_str();
            units = cap.get(2).unwrap().as_str();
            val = u32::from_str(cap.get(1).unwrap().as_str()).expect("How is this not a number?!>");

            if units == "in" {
                if val >= 59 && val <= 76 { true } else { false }
            } else {
                if val >= 150 && val <= 193 { true } else { false }
            }
        },
        None => false,
    };
    // println!("{}, {}, {}, {}",cap_str, val, units, res);
    res
}

fn has_valid_hcl(input:&str) -> bool {
    let hcl_re:regex::Regex = Regex::new(r"hcl:#([0-9a-f]{6})").unwrap();
    let caps = hcl_re.captures(input);
    match caps{
        Some(cap) => {
            //println!("{}", cap.get(0).unwrap().as_str());    
            true
        },
        None => false,
    }
}

fn has_valid_ecl(input:&str) -> bool {
    let ecl_re:regex::Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let caps = ecl_re.captures(input);
    match caps{
        Some(_) => true,
        None => false,
    }
}

fn has_valid_pid(input:&str) -> bool {
    let pid_re:regex::Regex = Regex::new(r"pid:(\d{9})(\D|$)").unwrap();
    let caps = pid_re.captures(input);
    match caps {
        Some(_) => true,
        None => false,
    }
}

// 173 TOO HIGH 

impl<'a> GovId<'a> {
    fn new(input: &str) -> GovId {
        // println!("{}", input);
        // println!("####################");

        let byr_re:regex::Regex = Regex::new(r"byr:(\d{4})").unwrap();
        let iyr_re:regex::Regex = Regex::new(r"iyr:(\d+)").unwrap();
        let eyr_re:regex::Regex = Regex::new(r"eyr:(\d+)").unwrap();
        let hgt_re:regex::Regex = Regex::new(r"hgt:(\d+)").unwrap();
        let hcl_re:regex::Regex = Regex::new(r"hcl:#?(\w+)").unwrap();
        let ecl_re:regex::Regex = Regex::new(r"ecl:#?(\w+)").unwrap();
        let pid_re:regex::Regex = Regex::new(r"pid:#?(\w+)").unwrap();
        let cid_re:regex::Regex = Regex::new(r"cid:(\d+)").unwrap();

        GovId { 
            byr:unpack_u32(&byr_re, input),
            iyr:unpack_u32(&iyr_re,input),
            eyr:unpack_u32(&eyr_re,input),
            hgt:unpack_str(&hgt_re, input),
            hcl:unpack_str(&hcl_re, input),
            ecl:unpack_str(&ecl_re,input),
            pid:unpack_str(&pid_re,input),
            cid:unpack_u32(&cid_re, input),
        }
    }

    fn is_valid(&self) -> bool {
        // println!("{}", self.hcl != None);
        return self.byr != None 
            && self.iyr != None 
            && self.eyr != None 
            && self.hgt != None 
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }
}

fn part1(ids:&Vec<&str>) -> Option<u32> {
    let mut valid_count = 0;
    for id in ids {
        let gov_id = GovId::new(id);
        // println!("byr: {:?}, iyr: {:?}, eyr: {:?}, hgt:{:?}, hcl: {:?}, ecl:{:?}, pid:{:?},cid:{:?}",
            // gov_id.byr,
            // gov_id.iyr,
            // gov_id.eyr,
            // gov_id.hgt,
            // gov_id.hcl,
            // gov_id.ecl,
            // gov_id.pid,
            // gov_id.cid);
            // println!("####################\n\n");
        if gov_id.is_valid() {
            valid_count = valid_count + 1;
        }
    }

    Some(valid_count)
}

fn part2(ids:&Vec<&str>) -> Option<u32> {
    let mut valid_count = 0;
    for id in ids 
    {
        if has_valid_byr(id)
            && has_valid_iyr(id)
            && has_valid_eyr(id)
            && has_valid_hgt(id)
            && has_valid_hcl(id)
            && has_valid_ecl(id)
            && has_valid_pid(id) {
            valid_count += 1;
        }
    }

    Some(valid_count)
}

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");

    let id_data:Vec<&str> = input.split("\r\n\r\n").collect();
    // println!("{}", id_data.len());
    println!("{}",part1(&id_data).unwrap());   
    println!("{}",part2(&id_data).unwrap());
}
