use helpers::helpers::load_input;
use crate::helpers;



pub fn run() {
    let input = load_input("day1".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    // Nice we test every combination twice if we dont find a correct one
    'outer: for first_s in input.lines() {
        let num1 = first_s.parse::<i32>().unwrap();
        for second_s in input.lines() {
            let num2 = second_s.parse::<i32>().unwrap();
            if num1 + num2 == 2020 {
                println!("Part1: {}", num1 * num2);
                break 'outer;
            }
        }
    }
}

fn part2(input: &String) {
    // Nice we test every combination thrice if we dont find a correct one
    'outer: for first_s in input.lines() {
        let num1 = first_s.parse::<i32>().unwrap();
        for second_s in input.lines() {
            let num2 = second_s.parse::<i32>().unwrap();
            for third_s in input.lines() {
                let num3 = third_s.parse::<i32>().unwrap();
                if num1 + num2 + num3 == 2020 {
                    println!("Part2: {}", num1 * num2 * num3);
                    break 'outer;
                }
            }
        }
    }
}