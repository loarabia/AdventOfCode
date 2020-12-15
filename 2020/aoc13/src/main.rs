use std::fs::read_to_string;
use std::str::FromStr;

fn part1(leave_time:i32, bus_ids:Vec<i32>) -> i32 {
    
    let earliest_bus_id = bus_ids.iter().map(|bus_id| (bus_id, bus_id * (leave_time/bus_id +1 ))).min_by_key(|pair| pair.1).unwrap().0;
    let earliest_bus_wait = earliest_bus_id*((leave_time/earliest_bus_id)+1) - leave_time;

    // println!("bus {}, wait {}", earliest_bus_id, earliest_bus_wait);

    earliest_bus_id * earliest_bus_wait
}

fn part2(bus_ids:Vec<(i64/*idx*/, i64/*val*/)>) -> i64 {
    let mut timestamp = 0;
    let mut step = 1;

    for (offset,val) in bus_ids {
        while(timestamp + offset) % val != 0 {
            timestamp += step;
        }
        step *= val;
    }

    timestamp
    // let mut timestamp = bus_ids[0].1;
    // println!("{:?}", bus_ids);
    // for i in (1..) {
        // // println!("i {} \t\t timestamp {}", i, timestamp);
        // if bus_ids.iter().all(|(idx, val)| (timestamp + idx) % val == 0) {
            // break;
        // }
        // timestamp = bus_ids[0].1 * i;
    // }

    // timestamp
}

fn main() {
    let filename = "input.txt";
    let input = read_to_string(filename).expect( &format!("couldn't open file {}", filename).to_string());
    let leave_time = i32::from_str(input.lines().nth(0).unwrap()).unwrap();
    
    let mut bus_ids:Vec<i32> = Vec::new();
    
    for value in input.lines().nth(1).unwrap().split(',') {
        match i32::from_str(value) {
            Ok(num) => bus_ids.push(num),
            Err(_e)     => {/* NOP */ },
        };
    }

    let mut bus_schedule:Vec<(i64,i64)> = Vec::new();
    for (index, value) in input.lines().nth(1).unwrap().split(',').enumerate() {
        if let Ok(num) = i64::from_str(value) {
            bus_schedule.push((index as i64, num));
        }

    }
    println!("{}",part1(leave_time, bus_ids));
    println!("{}", part2(bus_schedule));
}
