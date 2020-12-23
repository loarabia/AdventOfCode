use std::fs::read_to_string;
use std::cmp::min;

type Arr3D = Vec<Vec<Vec<bool>>>;

fn new(width:usize, height:usize, depth:usize) -> Arr3D {
    vec![vec![vec![false;width];height];depth]
}
fn width(arr:&Arr3D)  -> usize {
    arr[0][0].len()
}

fn height(arr:&Arr3D) -> usize {
    arr[0].len()
}

fn depth(arr:&Arr3D) -> usize {
    arr.len()
}

fn grow(arr:&Arr3D) -> Arr3D {

    // Otherwise resize +1 in each dimension and copy data in starting at 1,1,1
    // RESIZE
    let new_width = width(&arr) + 2;
    let new_height = height(&arr) + 2;
    let new_depth = depth(&arr) + 2;
    let mut new_arr = new(new_width, new_height, new_depth);

    // COPY with offest 1,1,1
    for x in 0..width(&arr) {
        for y in 0..height(&arr) {
            for z in 0..depth(&arr) {
                let val = get_val(&arr, x, y, z);
                set_val(&mut new_arr, val, x+1, y+1, z+1);
            }
        } 
    }
    return new_arr
}

fn is_active(arr:&Arr3D, x:usize, y:usize, z:usize) -> bool {
    get_val(arr, x, y, z)
}

fn get_val(arr:&Arr3D, x:usize, y:usize, z:usize) -> bool {
    arr[z][y][x]
}

fn set_val(arr:&mut Arr3D, val: bool, x:usize, y:usize, z:usize) {
    arr[z][y][x] = val;
}

fn count_enabled_cubes(arr:&Arr3D) -> u32 {
    let mut count = 0;
    for x in 0..width(&arr){
        for y in 0..height(&arr){
            for z in 0..depth(&arr){
                if is_active(arr, x, y, z) {
                    count += 1
                }
            }
        }
    }
    count
}

fn count_active_neighbors(arr:&Arr3D, x:usize, y:usize, z:usize) -> u32 {
    let mut count = 0;

    // Gather neighbors
    let left = x.saturating_sub(1);
    let right = min(x+1,width(&arr)-1);
    
    let top = y.saturating_sub(1);
    let bot = min(y+1,height(&arr)-1);

    let deepest = z.saturating_sub(1);
    let shallowest = min(z+1,depth(&arr)-1); 

    // Count Actives
    for xx in left..=right {
        for yy in top..=bot {
            for zz in deepest..=shallowest {
                // Don't count the cell itself
                if xx == x && yy == y && zz == z {
                    continue;
                }

                if is_active(&arr, xx, yy, zz) {
                    count += 1;
                }

            }
        }
    }
    count
}

fn print_cube(arr:&Arr3D) {
    for z in 0..depth(arr) {
        for y in 0..height(arr) {
            for x in 0..width(arr) {
                if is_active(&arr, x, y, z) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

}

fn read_plane(input:&str) -> Arr3D {
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().trim().len();
    
    let mut plane = new(width,height,1);

    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == '.' as u8 {
                set_val(&mut plane, false, x, y, 0);
            } else if  b == '#' as u8 {
                set_val(&mut plane, true, x, y, 0);
            
            }
        }
    }
    plane
}

fn run_rule(arr:Arr3D) -> Arr3D {
    let curr = grow(&arr);
    let mut next = new(width(&curr), height(&curr), depth(&curr));

    for x in 0..width(&next) {
        for y in 0..height(&next) {
            for z in 0..depth(&next) {

                let active = is_active(&curr, x, y, z);
                let neighbors = count_active_neighbors(&curr, x, y, z);

                if (!active) && neighbors == 3 {
                    set_val(&mut next, true, x, y, z);
                } else {
                    set_val(&mut next, false, x, y, z);
                }

                 if active && (neighbors == 2 || neighbors == 3)  {
                    set_val(&mut next, true, x, y, z);
                }
            }
        }
    }
    next
}

fn run_rules(iters:u32, arr:Arr3D) -> u32 {
    let mut cube = arr;
    for _i in 0..iters {
        cube = run_rule(cube);
    }
    count_enabled_cubes(&cube)
}

fn part1(input:&str) -> u32 {
    let cube = read_plane(input);
    run_rules(6, cube)
}

fn part2(input:&str) -> u32 {
    let hyper_cube = read_plane(input);
    0
}

fn main() {
    let filename = "input.txt";
    let input = read_to_string(filename).expect(&format!("Couldn't read {}",filename));
    
    println!("{}",part1(&input));   
    println!("{}",part2(&input));
}
