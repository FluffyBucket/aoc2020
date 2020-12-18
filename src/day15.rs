use std::collections::{HashMap};

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day15".to_string());
    part1(&input);
    part2(&input);
}

#[derive(Debug,Clone,Copy)]
struct Num {
    pub current: u64,
    pub prev: u64,
    pub count: u64
}

fn part1(input: &String) {
    let mut numbers: HashMap<u64,Num> = HashMap::new();
    let mut last_num = 0;
    // Insert starting numbers
    // Does not handle duplicates in the starting numbers
    for (i,line) in input.split(",").enumerate() {
        let num = line.parse::<u64>().unwrap();
        last_num = num;
        numbers.insert(num, Num {current: i as u64, prev: 0, count: 1});
    }

    let len = numbers.len();
    
    for i in len..2020 {
        //println!("Turn: {}, last_num: {}", i, last_num);
        if numbers.contains_key(&last_num) {
            let num = *numbers.get(&last_num).unwrap();
            //println!("\tLastNum: {:?}", num);
            let mut new = 0;
            if num.count > 1 {
                new = num.current - num.prev;
            }
            //println!("\tAdding: {}", new);
            let new_num = numbers.entry(new).or_insert(Num {current: i as u64, prev: 0, count: 1});
            new_num.count += 1;
            new_num.prev = new_num.current;
            new_num.current = i as u64;
            last_num = new;
        } else {
            numbers.insert(0, Num {current: i as u64, prev: 0, count: 1});
        }
    }
    println!("Part1: last num: {}", last_num);

    //println!("Numbers: {:?}", numbers);
}

// Solution from:
// https://github.com/LinAGKar/advent-of-code-2020-rust/blob/main/day15b/src/main.rs
// A vec is faster???
// Wow amazing a lot faster than my code. Orders of magnitudes faster.
fn part2(input: &String) {
    const ITERATIONS: usize = 30000000;

    let numbers: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let (&last_number, rest) = numbers.split_last().unwrap();
    // This does not actual allocate a vec of size iterations
    let mut number_timestamps = vec![None; ITERATIONS];
    for (&i, n) in rest.iter().zip(1..) {
        number_timestamps[i] = Some(n);
    }
    
    let (_, last_number) = (numbers.len()..ITERATIONS).fold(
        (number_timestamps, last_number),
        |(mut number_timestamps, last_number), curr_time| {
            let new_number = if let Some(last_time) = number_timestamps[last_number] {
                curr_time - last_time
            } else {
                0
            };
            number_timestamps[last_number] = Some(curr_time);
            (number_timestamps, new_number)
        },
    );

    println!("Part2: {}", last_number);

    //println!("Numbers: {:?}", numbers);


}

