use std::{fs::{read_to_string}, collections::VecDeque};

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input.text");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn read_grid_width_and_height(input:&String) -> (usize, usize) {
    let width = input.lines().next().unwrap().as_bytes().len();
    let height = input.lines().count();
    (width, height)
}

fn make_grid<T:Default + Clone>(width:usize, height:usize, default:Option<T>) -> Vec<Vec<T>> {
    vec![vec![default.unwrap_or_default(); height]; width]
}

fn width<T>(grid:&Vec<Vec<T>>) -> usize {
    grid.len()
}

fn height<T>(grid:&Vec<Vec<T>>) -> usize {
    grid[0].len()
}

fn print_visited(visited:&Vec<Vec<bool>>, path:&Vec<Option<(usize,usize)>>, start:&(usize,usize), end:&(usize,usize)) {
    let mut grid = make_grid::<char>(width(visited), height(visited), Some('.'));


    for y in 0..height(&grid){
        for x in 0..width(&grid){
            if visited[x][y] { grid[x][y] = 'X'; }
        }
    }

    for step in path {
        match *step {
            Some( (x,y)) => { grid[x][y] = 'P'; },
            None => { break; }, 
        }
    }

    grid[end.0][end.1] = 'E';
    grid[start.0][start.0] = 'S';

    for y in 0..height(&grid) {
        for x in 0..width(&grid) {
            print!("{}", grid[x][y]);
        }
        println!();
    }
}

fn neighbors<T>(pt:&(usize,usize), grid:&Vec<Vec<T>> ) -> Vec<(usize,usize)> {
    let mut neighbors:Vec<(usize,usize)> = Vec::new();
    
    if pt.0 as i32 -1 >= 0   { neighbors.push((pt.0-1, pt.1))}
    if pt.0+1 < width(grid)  { neighbors.push((pt.0+1, pt.1))}
    if pt.1 as i32 -1 >= 0   { neighbors.push((pt.0, pt.1-1))}
    if pt.1+1 < height(grid) { neighbors.push((pt.0, pt.1+1))}

    neighbors
}

fn distance(graph:&Vec<Vec<u8>>, a:&(usize,usize), b:&(usize,usize)) -> i32 {
    graph[a.0][a.1] as i32 - graph[b.0][b.1] as i32

}

fn bfs(graph:&Vec<Vec<u8>>, start:&(usize,usize), end:&(usize,usize)) -> i32 {
    let mut queue:VecDeque<(usize,usize)> = VecDeque::new();

    let mut visited:Vec<Vec<bool>> = make_grid(width(graph), height(graph), None);
    let mut distances:Vec<Vec<i32>> = make_grid(width(graph), height(graph), Some(i32::MAX));
    let mut parent:Vec<Vec<Option<(usize,usize)>>> = make_grid(width(graph), height(graph), None);
    
    let mut v = *start;
    distances[start.0][start.1] = 0;
    parent[start.0][start.1] = None;
    queue.push_back(v);


   while !queue.is_empty(){

        v = queue.pop_front().unwrap();

        if v == *end {
            break;
        }

        visited[v.0][v.1] = true;

        let neighbors = neighbors(&v, graph);
        for neighbor in neighbors
            .iter()
            .filter(|n| distance(graph, &v, n) >= -1) {
            {
                if !visited[neighbor.0][neighbor.1] {
                    
                    visited[neighbor.0][neighbor.1] = true;
                    distances[neighbor.0][neighbor.1] = distances[v.0][v.1] + 1;
                    parent[neighbor.0][neighbor.1] = Some((v.0,v.1));

                    queue.push_back(*neighbor);
                }
            }
        }
    } // while
    let mut path:Vec<Option<(usize,usize)>> = Vec::new();

    let mut point = Some(*end);

    while point != None {
        point = parent[point.unwrap().0][point.unwrap().1];
        path.push(point);
    }
    // print_visited(&visited, &path, start, end);

    distances[end.0][end.1]
}

fn part1(input:&String) -> i32 {
    let (width,height) = read_grid_width_and_height(input);
    let mut forest = make_grid(width, height, None);

    let mut start = (0,0);
    let mut end = (0,0);

    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.as_bytes().iter().enumerate() {
            forest[x][y] = *cell;
            if cell == &b'S' {
                forest[x][y] = b'a';
                start = (x,y);
            } else if cell == &b'E' {
                forest[x][y] = b'z';
                end = (x,y);
            }
        } 
    }
    let result = bfs(&forest,&start,&end);
    result
}

fn part2(input: &String) -> i32 {
    let (width,height) = read_grid_width_and_height(input);
    let mut forest = make_grid(width, height, None);
    let mut a_locs:Vec<(usize,usize)> = Vec::new();    
    let mut end = (0,0);
    
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.as_bytes().iter().enumerate() {
            forest[x][y] = *cell;
            if cell == &b'S' || cell == &b'a' {
                a_locs.push((x,y));
            } else if cell == &b'E' {
                forest[x][y] = b'z';
                end = (x,y);
            }
        }
    }
    a_locs
        .iter()
        .map(|a| bfs(&forest,a,&end))
        .min().unwrap()
}