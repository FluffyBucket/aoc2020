use std::{collections::{HashMap}, vec};

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day16".to_string());
    part1(&input);
    part2(&input);
}


fn part1(input: &String) {
    let lines = input.lines().collect::<Vec<&str>>();
    // let str_rules = &lines[..3];
    // let ticket = &lines[4..6];
    // let others = &lines[8..];
    let str_rules = &lines[..20];
    let ticket = &lines[22..23][0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let others = &lines[25..];


    // for (f,s) in all_rule_pairs(str_rules) {
    //     //println!("{},{}", f,s);
    // }
    // println!();
    // for l in ticket {
    //     //println!("{}", l);
    // }
    // println!();
    let mut sum = 0;
    for l in others {
        //println!("{}", l);
        let numbers = l.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for num in numbers {
            if !valid_num(&num, &all_rule_pairs(str_rules)) {
                sum += num;
            }
        }
        
    }

    println!("Part1: {}", sum);
}

fn all_rule_pairs(list: &[&str]) -> Vec<(i32,i32)> {
    let mut rules = Vec::new();

    for line in list {
        let pairs = line.split(": ").nth(1).unwrap().split(" or ").collect::<Vec<&str>>();
        for pair in pairs {
            let numbers = pair.split("-").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            rules.push((numbers[0], numbers[1]))
        }
    }

    return rules;
}

// Yuck, I did not understand that the columns would match to multiple fields.
// Ugly solution because f*%¤ it
fn part2(input: &String) {
    let lines = input.lines().collect::<Vec<&str>>();
    // let str_rules = &lines[..3];
    // let ticket = &lines[5..6][0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // let others = &lines[8..];
    let str_rules = &lines[..20];
    let ticket = &lines[22..23][0].split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let others = &lines[25..];

    println!("{:?}", ticket);
    let mut valid_tickets = Vec::new();

    for l in others {
        //println!("{}", l);
        let numbers = l.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if valid_numbers(&numbers, &all_rule_pairs(str_rules)) {
            valid_tickets.push(numbers);
        }
        
    }

    valid_tickets.push(ticket.clone());

    let rules = get_rule_pairs(str_rules);

    let mut rule_field_map: HashMap<&str, Vec<usize>> = HashMap::new();
    // Go through all columns and find correct rule
    for c in 0..valid_tickets[0].len() {
        //println!("Checking column: {}", c);
        'rule_loop: for (name, (low1,high1, low2, high2)) in rules.clone() {
            //println!("Checking rule: {}", name);
            
                for ticket in valid_tickets.clone() {
                    if !((ticket[c] >= low1 && ticket[c] <= high1) || (ticket[c] >= low2 && ticket[c] <= high2)) {
                        //println!("\tRule: {} for col: {} Does not match: {} | \t{}-{} or {}-{}", name, c, ticket[c], low1,high1,low2, high2);
                        continue 'rule_loop;
                    }
                }

                if rule_field_map.contains_key(name) {
                    rule_field_map.get_mut(name).unwrap().push(c);
                } else {
                    rule_field_map.insert(name, vec![c]);
                }
            
        }
    }
    let mut r_p = vec!["";20];
    let mut temp = Vec::new();
    for (k,v) in rule_field_map {
        println!("{}: {:?}", k,v);
        temp.push((k,v));
    }

    temp.sort_by(|(_,v1),(_,v2)| v1.len().partial_cmp(&v2.len()).unwrap());
    for (k,v) in temp {
        println!("{}: {:?}", k,v);
        for n in v {
            if r_p[n] == "" {
                r_p[n] = k;
            }
        }
    }

    let mut sum = 1;
    for (i,n) in r_p.iter().enumerate() {
        println!("{}",n);
        if n.contains("departure") {
            sum *= ticket[i as usize] as u64;
        }
    }


    println!("Part2: {}", sum);
}

fn get_rule_pairs<'a>(list: &'a [&str]) -> Vec<(&'a str,(i32,i32,i32,i32))> {
    let mut rules = Vec::new();

    for line in list {
        //println!("{}", line);
        let mut split = line.split(": ");
        let name = split.nth(0).unwrap();
        let pairs = split.nth(0).unwrap().split(" or ");
        let numbers = pairs.map(|p| p.split("-").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>().concat();
        
        rules.push((name,
            (numbers[0], numbers[1], numbers[2], numbers[3]))
        );
        
    }

    return rules;
}

fn valid_num(num: &i32, rules: &Vec<(i32,i32)>) -> bool {
    for (lower,higher) in rules {
        if num >= lower && num <= higher {
            return true;
        }
    }

    return false;
}

fn valid_numbers(numbers: &Vec<i32>, rules: &Vec<(i32,i32)>) -> bool {
    for num in numbers {
        if !valid_num(num, &rules) {
            return false;
        }
    }

    return true;
}

