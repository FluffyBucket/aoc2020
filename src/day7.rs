use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;



pub fn run() {
    let input = load_input("day7".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut rule_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let mut count = 0;
    for line in input.lines() {
        let split = line.split(" bags contain ").collect::<Vec<&str>>();
        let outer = split[0];
        let rules = split[1].split(", ").collect::<Vec<&str>>();
        //println!("Outer: {}", outer);

        let mut inner_rules: HashMap<String, i32> = HashMap::new();
        for rule in rules {
            //println!("\tRule: {}", rule);
            let r_split = rule.split(" ").collect::<Vec<&str>>();
            let num = r_split[0].parse::<i32>().unwrap_or(0);
            if num != 0 {
                let bag = r_split[1..r_split.len()-1].join(" ");
                inner_rules.insert(bag, num);
            }
        }

        rule_map.insert(outer.to_string(), inner_rules);
    }
    //println!("RULE MAP: {:?}", rule_map);

    for bag in rule_map.keys() {
        if can_hold(&"shiny gold".to_string(), bag, &rule_map) {
            count += 1;
        }
    }
    println!("Part1: {}", count);
}

fn part2(input: &String) {
    let mut rule_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in input.lines() {
        let split = line.split(" bags contain ").collect::<Vec<&str>>();
        let outer = split[0];
        let rules = split[1].split(", ").collect::<Vec<&str>>();
        //println!("Outer: {}", outer);

        let mut inner_rules: HashMap<String, i32> = HashMap::new();
        for rule in rules {
            //println!("\tRule: {}", rule);
            let r_split = rule.split(" ").collect::<Vec<&str>>();
            let num = r_split[0].parse::<i32>().unwrap_or(0);
            if num != 0 {
                let bag = r_split[1..r_split.len()-1].join(" ");
                inner_rules.insert(bag, num);
            }
        }

        rule_map.insert(outer.to_string(), inner_rules);
    }
    

    let mut count = 0;
    let rules = rule_map.get(&"shiny gold".to_string()).unwrap();
    for (inner_bag, amount) in rules {
        count += contains_amount(inner_bag, &rule_map) * amount;
    }

    println!("Part2: {}", count);
}

fn can_hold(bag: &String, outer: &String, rule_map: &HashMap<String, HashMap<String, i32>>) -> bool {
    //println!("RULE MAP: {:?}", rule_map);
    //println!("outer: {}", outer);
    let rules = rule_map.get(outer).unwrap();

    if rules.contains_key(bag) {
        return true;
    } else { 
        for rule_outer in rules.keys() {
            if can_hold(bag, rule_outer, rule_map) {
                return true;
            }
        }
    }

    return false;
}


fn contains_amount(bag: &String, rule_map: &HashMap<String, HashMap<String, i32>>) -> i32 {

    let rules = rule_map.get(bag).unwrap();
    let mut count = 1;
    for (inner_bag, amount) in rules {
        count += contains_amount(inner_bag, rule_map) * amount;
    } 

    return count;
}
 