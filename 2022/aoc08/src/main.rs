use std::{fs::{read_to_string}};

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn make_grid<T:Default>(width:usize) -> Vec<Vec<T>> {
    let mut outer:Vec<Vec<T>> = Vec::new();
    for _x in 0..width {
        let mut inner:Vec<T> = Vec::new();
        for _y in 0..width {
            inner.push( T::default());
        }
        outer.push(inner);
    }
    outer
}

fn map_to_height(c:i8) -> i8 {    
    c - b'0' as i8
}

fn mark_visible_trees_from_top(visibility:&mut Vec<Vec<bool>>, forest:&Vec<Vec<i8>>){
    let grid_height = forest[0].len();
    let mut height = -1;
    for x in 0..grid_height {
        for y in 0..grid_height {
            if forest[x][y] > height {
                visibility[x][y] |= true;
                height = forest[x][y];
            }
        }
        height = -1;
    }
}

fn mark_visible_trees_from_bot(visibility:&mut Vec<Vec<bool>>, forest:&Vec<Vec<i8>>){
    let grid_height = forest[0].len();
    let mut height = -1;
    for x in 0..grid_height {
        for y in 0..grid_height {
            if forest[x][grid_height-y -1] > height {
                visibility[x][grid_height-y -1] |= true;
                height = forest[x][grid_height-y -1];
            }
        }
        height = -1;
    }
}

fn mark_visible_trees_from_lft(visibility:&mut Vec<Vec<bool>>, forest:&Vec<Vec<i8>>){
    let grid_height = forest[0].len();
    let mut height = -1;
    for y in 0..grid_height {
        for x in 0..grid_height {
            if forest[x][y] > height {
                visibility[x][y] |= true;
                height = forest[x][y];
            }
        }
        height = -1;
    }
}

fn mark_visible_trees_from_rght(visibility:&mut Vec<Vec<bool>>, forest:&Vec<Vec<i8>>){
    let grid_height = forest[0].len();
    let mut height = -1;
    for y in 0..grid_height {
        for x in 0..grid_height {
            if forest[grid_height - x - 1][y] > height {
                visibility[grid_height - x - 1][y] |= true;
                height = forest[grid_height - x - 1][y];
            }
        }
        height = -1;
    }
}

fn calculate_scenic_score(x:usize,y:usize,grid:&Vec<Vec<i8>>) -> i32 {
    let up_score = calculate_scenic_score_up(x, y, grid);
    let dwn_score = calculate_scenic_score_down(x, y, grid);
    let lft_score = calculate_scenic_score_left(x, y, grid);
    let rgt_score = calculate_scenic_score_right(x, y, grid);

    let scenic_score = up_score * dwn_score * lft_score * rgt_score;

    scenic_score
}

fn calculate_scenic_score_up(x:usize, y:usize, grid:&Vec<Vec<i8>>) -> i32 {
    let mut score = 0;
    let height = grid[x][y];
    for i in 0..y {
        let neighbor_height = grid[x][y-i-1];
        score += 1;
        if neighbor_height >= height {
            break;
        }
    }
    score
}

fn calculate_scenic_score_down(x:usize, y:usize, grid:&Vec<Vec<i8>>) -> i32 {    
    let grid_height = grid[0].len();
    let mut score = 0;
    let height = grid[x][y];
    for i in y+1..grid_height {
        let neighbor_height = grid[x][i];
        score += 1;
        if neighbor_height >= height {
            break;
        }
    }
    score
}

fn calculate_scenic_score_left(x:usize, y:usize, grid:&Vec<Vec<i8>>) -> i32 {    
    let mut score = 0;
    let height = grid[x][y];
    for i in 0..x {
        let neighbor_height = grid[x-i-1][y];
        score += 1;

        if neighbor_height >= height {
            break;
        }
    }
    score
   
}

fn calculate_scenic_score_right(x:usize, y:usize, grid:&Vec<Vec<i8>>) -> i32 {
    let grid_height = grid[0].len();
    let mut score = 0;
    let height = grid[x][y];
    for i in x+1..grid_height {
        let neighbor_height = grid[i][y];
        score += 1;

        if neighbor_height >= height {
            break;
        }
    }
    score
}


fn part1(input:&String) -> usize {
    let height = input.lines().next().unwrap().as_bytes().len();
    let mut grid:Vec<Vec<i8>> = make_grid::<i8>(height);
    let mut visibility:Vec<Vec<bool>> = make_grid::<bool>(height);

    for (y, row) in input.lines().enumerate(){
        for (x,col) in row.as_bytes().iter().enumerate() {
            grid[x][y] = map_to_height((*col) as i8);
        }
    }//for


    let mut visible_trees = 0;

    mark_visible_trees_from_top(&mut visibility, &grid);
    mark_visible_trees_from_lft(&mut visibility, &grid);
    mark_visible_trees_from_rght(&mut visibility, &grid);
    mark_visible_trees_from_bot(&mut visibility, &grid);


    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if visibility[x][y] == true {
                visible_trees += 1;
            }
        }
    }


    visible_trees
}

fn part2(input: &String) -> i32 {

    let height = input.lines().next().unwrap().as_bytes().len();
    let mut grid:Vec<Vec<i8>> = make_grid::<i8>(height);

    for (y, row) in input.lines().enumerate(){
        for (x,col) in row.as_bytes().iter().enumerate() {
            grid[x][y] = map_to_height((*col) as i8);
        }
    }//for

    let mut most_scenic_score = 0;
    for x in 0..height {
        for y in 0..height {
            let score = calculate_scenic_score(x, y, &grid);
            if score > most_scenic_score {
                most_scenic_score = score;
            }
        }
    }

    most_scenic_score
}