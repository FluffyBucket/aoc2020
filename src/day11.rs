use std::{cmp::max, collections::HashMap};

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day11".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut old_layout = input.lines().map(
        |line| [vec!('.'),line.chars().collect::<Vec<char>>(),vec!('.')].concat()
    ).collect::<Vec<Vec<char>>>();
    old_layout.insert(0, vec!['.'].repeat(old_layout[0].len()));
    old_layout.push(vec!['.'].repeat(old_layout[0].len()));
    //print_layout(&layout);
    //println!("{}", layout[0][0]);
    let mut change = true;
    let mut new_layout = old_layout.to_vec();
    
    while change {
        change = false;
        // println!("Old:");
        // print_layout(&old_layout);
        for row_i in 0..old_layout.len() {
            
            for col_i in 0..old_layout[0].len() {
                let adj = count_adjacent(row_i, col_i, &old_layout);
                //println!("{},{}: adj: {}", row_i, col_i, adj);
                if old_layout[row_i][col_i] == 'L' && adj == 8 {
                    new_layout[row_i][col_i] = '#';
                    change = true;
                } else if old_layout[row_i][col_i] == '#' && adj <= 4 {
                    new_layout[row_i][col_i] = 'L';
                    change = true;
                }
            }
        }
        
        // println!("New:");
        // print_layout(&new_layout);

        old_layout = new_layout.to_vec();
    }

    let seat_count: i64 = new_layout.iter().map(|row| row.iter().map(|c| if *c == '#' {1} else {0}).sum::<i64>()).sum::<i64>();
    println!("Part1: {}", seat_count);
}

fn count_adjacent(row: usize, col: usize, layout: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row_i in sub_usize(row, 1)..(row+2) {
        //println!("\tRow: {}",row_i);
        match layout.get(row_i) {
            Some(r) => {
                for col_i in sub_usize(col, 1)..(col+2) {
                    //println!("\tCol: {}",col_i);
                    if !(row_i == row && col_i == col) {
                        match r.get(col_i) {
                            Some(c) => {
                                if *c == '.' || *c == 'L' { count+=1 }
                            }
                            _ => ()
                        }
                    }
                }
            }
            _ => ()
        }
    }
    return count;
}

fn sub_usize(val: usize, offset: usize) -> usize {
    match val.checked_sub(offset) {
        Some(n) => return n,
        _ => return 0
    }
}

fn part2(input: &String) {
    let mut old_layout = input.lines().map(
        |line| [vec!('.'),line.chars().collect::<Vec<char>>(),vec!('.')].concat()
    ).collect::<Vec<Vec<char>>>();
    old_layout.insert(0, vec!['.'].repeat(old_layout[0].len()));
    old_layout.push(vec!['.'].repeat(old_layout[0].len()));
    //print_layout(&layout);
    //println!("{}", layout[0][0]);
    let mut change = true;
    let mut new_layout = old_layout.to_vec();
    while change {
        change = false;
        // println!("Old:");
        // print_layout(&old_layout);
        for row_i in 0..old_layout.len() {
            
            for col_i in 0..old_layout[0].len() {
                //let adj = count_adjacent(row_i, col_i, &old_layout);
                let seen = seats_seen(row_i, col_i, &old_layout);
                //println!("{},{}: adj: {}", row_i, col_i, adj);
                if old_layout[row_i][col_i] == 'L' && seen == 0 {
                    new_layout[row_i][col_i] = '#';
                    change = true;
                } else if old_layout[row_i][col_i] == '#' && seen >= 5 {
                    new_layout[row_i][col_i] = 'L';
                    change = true;
                }
            }
        }
        
        // println!("New:");
        // print_layout(&new_layout);

        old_layout = new_layout.to_vec();
    }

    let seat_count: i64 = new_layout.iter().map(|row| row.iter().map(|c| if *c == '#' {1} else {0}).sum::<i64>()).sum::<i64>();
    println!("Part2: {}", seat_count);
}

fn seats_seen(row: usize, col: usize, layout: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for r in -1..2 {
        for c in -1..2 {
            //println!("{},{}",r,c);
            if !(r == 0 && c == 0) && seats_direction(row, col, layout, r, c){
                count += 1;
            }
        }
    }

    return count;
}

fn seats_direction(row: usize, col: usize, layout: &Vec<Vec<char>>, r_d: i32, c_d: i32) -> bool {
    let mut r_i: i32 = row as i32;
    let mut c_i: i32 = col as i32;
    let height = layout.len() as i32;
    let width = layout[0].len() as i32;
    loop {  
        r_i += r_d;
        c_i += c_d;

        if r_i < 0 || c_i < 0 {
            break;
        }
        if r_i > height-1 || c_i > width-1 {
            break;
        }

        if layout[r_i as usize][c_i as usize] == '#' {
            //println!("I SEE {}, {}", r_i, c_i);
            return true;
        } else if layout[r_i as usize][c_i as usize] == 'L' {
            //println!("I SEE {}, {}", r_i, c_i);
            break;
        }
        
    }

    return false;
}

fn print_layout(layout: &Vec<Vec<char>>) {
    for row in layout {
        for c in row {
            print!("{}", c);
        }
        print!("\n")
    }
}
