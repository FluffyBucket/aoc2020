use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day9".to_string());
    //part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let numbers = input.lines().into_iter().collect::<Vec<&str>>();
    let mut working_numbers: Vec<i64> = Vec::new();
    for n in numbers[..25].into_iter()  {
        //println!("{}", n);
        working_numbers.push(n.parse::<i64>().unwrap())
    }
    //println!("END");
    for n in numbers[25..].into_iter() {
        //println!("{}", n);
        let num = n.parse::<i64>().unwrap();
        if !is_valid(&working_numbers, num) {
            println!("Num is not valid: {}", num);
            break;
        }
        working_numbers.push(num);
        working_numbers.remove(0);
    }
}

fn part2(input: &String) {
    let numbers = input.lines().into_iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut working_numbers: Vec<i64> = Vec::new();
    let target: i64 = 26134589;
    //let target: i64 = 127;

    for n in numbers {
        working_numbers.push(n);
        let mut sum = sum_numbers(&working_numbers);
        while sum > target {
            working_numbers.remove(0);
            sum = sum_numbers(&working_numbers);
        }
        if sum == target {
            println!("The sum: {}, Their sum: {}", sum, sum_min_max(&working_numbers));
            break;
        }
        
    }

}

fn is_valid(numbers: &Vec<i64>, number: i64) -> bool {

    for (index, first_n) in numbers.clone().into_iter().enumerate() {
        for second_n in numbers[index..].into_iter() {
            if number == first_n + second_n {
                return true;
            }
        }
    }

    return false;
}

fn sum_numbers(numbers: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }

    return sum;
}

fn sum_min_max(numbers: &Vec<i64>) -> i64 {
    let mut min = numbers[0].clone();
    let mut max = 0;
    for n in numbers {
        if n < &min {
            min = *n;
        }
        if n > &max {
            max = *n;
        }
    }

    return min + max;
}