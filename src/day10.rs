use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day10".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut numbers = input.lines().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    numbers.sort();
    let mut one_diff = 0;
    // Start at one as the last adapter will always have a diff of three
    let mut three_diff = 1;
    let mut current_jolts = 0;
    for n in numbers {
        match n - current_jolts  {
            1 => one_diff += 1,
            2 => println!("Errrrr: {}", n),
            3 => three_diff += 1,
            diff => println!("Too big of a diff: {}, n: {}",diff,n)
        }
        current_jolts = n;
    }

    println!("Part1: one: {}, three: {}, the mult: {}", one_diff, three_diff, one_diff * three_diff);
}

fn part2(input: &String) {
    let mut numbers = input.lines().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    //numbers.sort_by(|a, b| b.cmp(a));
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers[numbers.len()-1]+3);

    let mut computed: HashMap<i64,i64> = HashMap::new();

    println!("Part2: {}", leaf_count(0,&numbers, &mut computed));
}

fn print_list(numbers: &Vec<i64>) {
    for (i,n) in numbers.iter().enumerate() {
        println!("{}: {}",i,n);
    }
}

fn leaf_count(i: usize, numbers: &Vec<i64>, computed: &mut HashMap<i64,i64>) -> i64 {
    let mut count = 0;
    let leafs = get_leafs(i, &numbers);
    if leafs.len() == 0 {
        // I am leaf
        //println!("leaf: {}", numbers[i]);
        return 1;
    } else {
        
        for leaf in leafs {
            //println!("{} has leaf {}", numbers[i], numbers[leaf]);
            
            match &computed.get(&numbers[leaf]) {
                Some(n) => count += n.clone(),
                None => {
                    let l_count = leaf_count(leaf, &numbers, computed);
                    computed.insert(numbers[leaf], l_count); 
                    count += l_count;
                }
            }
        }
    }

    return count;
}

fn get_leafs(i: usize, numbers: &Vec<i64>) -> Vec<usize> {
    let mut edges = Vec::new();
    for i_diff in 1..4 {
        if i+i_diff < numbers.len() && numbers[i+i_diff] - numbers[i] < 4 {
            //println!("{} has edge: {}", numbers[i], numbers[i+i_diff]);
            edges.push(i+i_diff);
        }
    }

    return edges;
}
