use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;



pub fn run() {
    let input = load_input("day6".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {

    let mut map: HashMap<char, i32> = HashMap::new();
    let mut sum = 0;
    for line in input.lines() {
        if line != "" {
            for c in line.chars() {
                map.insert(c, 1);
            }
        } else {
            // New group
            sum += map.len();
            map.clear();
        }
    }

    println!("Part1: {}", sum);
}

fn part2(input: &String) {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut group_size = 0;
    let mut sum = 0;
    for line in input.lines() {
        if line != "" {
            group_size += 1;
            for c in line.chars() {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
        } else {
            // New group
            for count in map.values() {
                if group_size == *count {
                    sum += 1;
                }
            }
            group_size = 0;
            map.clear();
        }
    }

    println!("Part2: {}", sum);
}