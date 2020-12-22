use std::collections::HashMap;

#[derive(Debug)]
struct PastTurns {
    newest:u32,
    oldest:Option<u32>,
}

impl PastTurns {
    fn new(newest_turn:u32) -> PastTurns {
        PastTurns { newest:newest_turn, oldest:None }
    }

    fn from_past_turn(newest_turn:u32, past_turns:&PastTurns ) -> PastTurns {
        PastTurns { newest:newest_turn, oldest:Some(past_turns.newest) }
    }
}

fn was_spoken_before(num:&u32, memory:&HashMap<u32, PastTurns> ) -> bool {
     memory.contains_key(num)
}

fn say_turns_apart(turns:&PastTurns) -> u32 {
    if turns.oldest == None {
        return 0
    } 
    return turns.newest - turns.oldest.unwrap();
}

fn remember(memory:&mut HashMap<u32, PastTurns>, spoken_num:&u32, turn:u32) {
    if let Some(past_turns) = memory.get(spoken_num) {
        let turn = PastTurns::from_past_turn(turn, past_turns);
        memory.insert(*spoken_num, turn);
    } else {
        let turn = PastTurns::new(turn);
        memory.insert(*spoken_num, turn);
    }
}

fn part1(stop:u32, seeds:&Vec<u32>) -> u32 {
    // map<num,last_seen> - tracks when the number was last spoken
    let mut memory:HashMap<u32, PastTurns> = HashMap::new();
    
    let mut last_spoken_num = 0;

    // take the first turns
    for (turn_num, start_num) in seeds.iter().enumerate() {
        last_spoken_num = *start_num;
        memory.insert(last_spoken_num, PastTurns::new(turn_num as u32) );
    }

    // Play the game
    for turn_num in seeds.len()..=stop as usize {

        // Is the last number in the map?
        if was_spoken_before(&last_spoken_num, &memory) {

            let spoken_num = say_turns_apart(memory.get(&last_spoken_num).unwrap());

            // Say the number
            //println!("{}", spoken_num);

            // Remember that you said it
            remember(&mut memory, &spoken_num, turn_num as u32);

            last_spoken_num = spoken_num;
        } else {
            // Say the number
            //println!("{}", 0);
            last_spoken_num = 0;
            memory.insert(last_spoken_num, PastTurns::new(turn_num as u32));
        }
    }
    last_spoken_num
}

fn main() {
    let seeds = vec![12,1,16,3,11,0];
    println!("{}",part1(2019, &seeds));
    println!("{}", part1(30000000-1, &seeds));

    assert!( part1(2019, &vec![0,3,6]) == 436);
    assert!( part1(2019, &vec![1,3,2,]) == 1);
    assert!( part1(2019, &vec![2,1,3,]) == 10);
    assert!( part1(2019, &vec![1,2,3,]) == 27);
    assert!( part1(2019, &vec![2,3,1,]) == 78);
    assert!( part1(2019, &vec![3,2,1,]) == 438);
    assert!( part1(2019, &vec![3,1,2,]) == 1836);

    // assert!( part1(30000000-1, &vec![0,3,6]) == 175594);    
}
