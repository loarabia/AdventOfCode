use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("File access error");

    let width = 25;
    let height = 6;

    run_part1(&contents, width, height);
    run_part2(&contents, width, height);
}

fn count_digit(digit:u8, data:&[u8]) -> i32 {
    data.iter().fold(0i32, |acc, item| 
        if digit == *item { acc + 1} else { acc + 0})

}

fn run_part1(contents:&String, w:usize, h:usize){

    let check_layer = contents.as_bytes()
        .chunks(w*h)
        .min_by(|layer1, layer2|
            count_digit('0' as u8, layer1).cmp( &count_digit('0' as u8, layer2))).unwrap();

    println!("{},{},{}", 
    count_digit('0' as u8, check_layer),
    count_digit('1' as u8, check_layer),
    count_digit('2' as u8, check_layer));
}

fn run_part2(contents:&String, w:usize, h:usize){

    let mut img:Vec<i32> = vec!['2' as i32; w*h];

    for layer in contents.as_bytes().chunks(w*h) {
        for i in 0..layer.len() {
            if img[i] == '2' as i32 {
                img[i] = layer[i] as i32;
            }
        }
    }

    let mut wrap = 0;
    for p in img {
        match p {
            48 => print!(" "),
            49 => print!("#"),
            v => {print!("{}",v)},
        }
        wrap +=1;

        if wrap == w {
            println!();
            wrap = 0;
        }
    }
}