use helpers::helpers::load_input;
use crate::helpers;



pub fn run() {
    let input = load_input("day3".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut map: Vec<Vec<char>> = input.lines().map(
        |line| line.chars().map(
            |c| c
        ).collect()
    ).collect();
    
    
    println!("Part1: {}", tree_in_slope(&map, 3, 1));
}

fn part2(input: &String) {
    let mut map: Vec<Vec<char>> = input.lines().map(
        |line| line.chars().map(
            |c| c
        ).collect()
    ).collect();

    let res = [(1,1),(3,1),(5,1),(7,1),(1,2)].iter().map(|(r,d)| tree_in_slope(&map, *r as usize, *d as usize)).collect::<Vec<i64>>();

    println!("Part2: {}", 
        res.iter().product::<i64>()
    );
}

fn tree_in_slope(map: &Vec<Vec<char>>, right: usize, down: usize) -> i64 {
    let w = map[0].len();
    let h = map.len();
    let mut count = 0;
    let mut height_count = down;
    let mut width_count = right;
    loop {
        if height_count >= h {
            break;
        }
        if map[height_count][width_count] == '#' {
            //println!("Found at: {},{}", height_count, (height_count*right) % w);
            count += 1;
        }
        width_count = (width_count + right) % w;
        height_count += down;
    }

    return count;
}
