use std::fs::read_to_string;
use std::str::Bytes;

/******************************************************************************
 * Long time since I wrote a recursive descent parser but that seems like a fun
 * way to handle this one.
 * ***************************************************************************/

 /*****************************************************************************
 * LEXING
 * Symbols = { NUM, LPAREN, RPAREN, PLUS, MULT}
 * Note numbers are all single digit too.
 *****************************************************************************/
#[derive(Debug, PartialEq)]
 enum Token {
    NUM(u8),
    LPAREN,
    RPAREN,
    PLUS,
    MULT,
    IGNORE,
}

fn lex(bytes:&mut Bytes) -> Option<Token> {

    let mut tok = None; 
    if let Some(b) = bytes.next() {
        match b as char {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => tok = Some(Token::NUM(b as u8 -'0' as u8)),
            '(' => tok = Some(Token::LPAREN),
            ')' => tok = Some(Token::RPAREN),
            '+' => tok = Some(Token::PLUS),
            '*' => tok = Some(Token::MULT),
            ' ' => tok = Some(Token::IGNORE), 
            _ => tok = None,

        }
    }
    tok
}

/******************************************************************************
 * PARSING
 * 
 * Grammar
 *  Expression -> Term {("+"|"*") Term}
 *  Term -> number
 *      | "(" Expression ")"
 * 
 * ***************************************************************************/
fn expr() -> u32 {
   // Expect TERM
   // EXPECT ONE OR MORE OF THE NEXT
   //   Expect + OR *
   //   Expect TERM
   // Perform TERM OP TERM
   0
}

fn term(curr_tok:&Token) -> u32 {
    // EXPECT NUMBER 
    // OR
    // Expect LPAREN
    // Expect EXPR
    // Expect RPAREN
    // Return value of EXPR

    match curr_tok {
        Token::NUM(val) => return *val as u32,
        Token::LPAREN => return expr(),
        _ => panic!("Unexpected token {:?}", curr_tok),
    }
}

fn part1(input:&str) -> u32 {
    for line in input.lines() {
        let mut bytes = line.bytes();
        while let Some(tok) = lex(&mut bytes) {
            if tok == Token::IGNORE { continue; }
            print!("{:?} ", tok);
        }
        println!();
    }
    0
}

fn part2(_input:&str) -> u32 {
    0
}

fn main() {
    let filename = "input2.txt";
    let input = read_to_string(filename).expect(&format!("Couldn't read {}",filename));
    
    println!("{}",part1(&input));   
    println!("{}",part2(&input));
}
