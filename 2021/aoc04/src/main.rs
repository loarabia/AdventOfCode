use std::str::FromStr;
use std::fs::{read_to_string}; 


fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.txt");
    let mut i = input.lines();

    let drawing = i.next().unwrap();
    let draws:Vec<i32> = drawing.split(',').map(|n| i32::from_str(n).unwrap()).collect();
    
    let b_iter = i.collect::<Vec<&str>>();

    // Build the boards
    let mut boards = Vec::<Board>::new();
    for (i,b) in b_iter.chunks(6).enumerate() {
        let b = Board::from_string(i as i32, b);
        boards.push(b);
    }

    // Part1
    'outer: for draw in &draws {
        for board in boards.iter_mut() {
            board.check_number(&draw);
            if board.has_winner() {
                println!("DRAW: {} -- WINNER: {:?}", draw, board.lines);
                println!("SUM: {}", board.sum_nums());
                println!("{}", board.sum_nums() * draw);
                break 'outer;
            }
        }
    }

    // Part2
    // Find candidate board (the last board that hasn't won)
    let mut cand_board:&mut Board = &mut Board{lines:vec![Vec::<i32>::new();0], id:0};
    let mut last_draw = 0;

    'oouter: for (i,draw) in draws.iter().enumerate() {
        last_draw = i;

        for board in boards.iter_mut() {
            board.check_number(&draw);
        }

        let c = boards.iter().filter(|b| !b.has_winner()).count();
        if c == 1 {
            cand_board = boards.iter_mut().filter(|b| !b.has_winner()).next().unwrap();
            break 'oouter;
        }
    }

    // Keep drawing until the candidate wins to get answer.
    for i in last_draw..draws.len() {
        println!("draw - {}",draws[i]);

        cand_board.check_number(&draws[i]);
        if cand_board.has_winner() {
            println!("{}", cand_board.sum_nums() * draws[i]);
            break;
        }
    }
}

struct Board {
    lines:Vec<Vec<i32>>,
    id:i32,
}

impl Board {

    fn from_string(id:i32, input:&[&str]) -> Board {
        // Bingo board is assumed to be square.
        let mut lines = vec![Vec::<i32>::new(); 2*(input.len()-1)];
        for (row,line) in input.iter().enumerate() {
            if line.len() == 0 { continue; }
            // println!("r-{}  {:?}", row, line);
            let num_strs = line.split_whitespace();
            for (col, num_str) in num_strs.enumerate() {
                let num = i32::from_str(num_str).unwrap();
                lines[row-1].push(num);
                lines[col+5].push(num);
            }
        }
        // println!("{:?}",lines);
        // Walk the input once pulling off the rows
        // Walk the input creating columns
        Board{lines, id}
    }

    fn has_winner(&self) -> bool {
        self.lines.iter().any(|i| i.len() == 0)
    }

    fn check_number(&mut self, num:&i32) {
        for line in self.lines.iter_mut() {
            if let Some(index) = line.iter().position(|item| item == num ) {
                line.remove(index);
            }
        }
    }

    fn sum_nums(&self) -> i32 {
        let mut result = 0;
        for i in 0..5 {
            result += self.lines[i].iter().sum::<i32>();
        }
        result
    }
}