use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;

extern crate lazy_static;
use regex::Regex;

type Passport = HashMap<String, String>;


pub fn run() {
    let input = load_input("day4".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let passports = get_passports(input);
    
    //println!("Passports: {:?}", passports);
    println!("Part1: {}", passports.iter().map(|p| if simple_valid_passport(p) {1} else {0}).sum::<i64>())
}

fn part2(input: &String) {
    let passports = get_passports(input);
    
    //println!("Passports: {:?}", passports);
    println!("Part2: {}", passports.iter().map(|p| if valid_passport(p) {1} else {0}).sum::<i64>())
}


fn get_passports(input: &String) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    let mut passport: Vec<String> = Vec::new();
    for line in input.lines() {
        //println!("Line: {:?}", line);
        if line != "" {
            &passport.push(line.to_string());
        } else {
            //println!("NEW Passport");
            // We have a new line => new passport
            let mut h_map = HashMap::new();
            for p_line in &passport {
                let pairs = p_line.split(" ");
                
                for p in pairs {
                    let fields = p.split(":").collect::<Vec<&str>>();
                    h_map.insert(fields[0].to_string(), fields[1].to_string());
                }
            }
            passport.clear();
            passports.push(h_map);
        }

    }

    return passports;
}

fn simple_valid_passport(passport: &Passport) -> bool {
    let required = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

    for req in required.iter() {
        if !passport.contains_key(&req.to_string()) {
            return false;
        }
    }

    return true;
}

fn valid_passport(passport: &Passport) -> bool {
    let required = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];

    

    for req in required.iter() {
        if !&passport.contains_key(&req.to_string()) {
            return false;
        }
    }
    //println!("Passport to test: {:?}", passport);
    for (key, val) in passport.iter() {
        //println!("Testing: {}", key);
        let (st, err) = valid_value(key, val);
        
        if !st {
            //println!("Failed with: {}",err);
            return st;
        }
    }

    return true;
}

fn valid_value(key: &String, val: &String) -> (bool, String) {
    lazy_static! {
        static ref RE_HCL: Regex = Regex::new("^#{1}[a-f0-9]{6}$").unwrap();
        static ref RE_PID: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }

    match key.as_str() {
        "byr" => {
            if val.len() > 4 {
                return (false, "Bad length".to_string());
            }
            match val.parse::<i32>() {
                Ok(n) => if n < 1920 || n > 2002 {return (false, "Out of bounds".to_string())},
                _ => return (false, "Not a number".to_string())
            }
        }
        "iyr" => {
            if val.len() > 4 {
                return (false, "Bad length".to_string());
            }
            match val.parse::<i32>() {
                Ok(n) => if n < 2010 || n > 2020 {return (false,"Out of bounds".to_string())},
                _ => return (false, "Not a number".to_string())
            }
        }
        "eyr" => {
            if val.len() > 4 {
                return (false, "Bad length".to_string());
            }
            match val.parse::<i32>() {
                Ok(n) => if n < 2020 || n > 2030 {return (false,"Out of bounds".to_string())},
                _ => return (false, "Not a number".to_string())
            }
        }
        "hgt" => {
            let l = val.len();
            match &val[l-2..l] {
                "cm" => {
                    match val[..l-2].parse::<i32>() {
                        Ok(n) => if n < 150 || n > 193 {return (false,"Out of bounds".to_string())}
                        _ => return (false, "Not a number".to_string())
                    }
                }
                "in" => {
                    match val[..l-2].parse::<i32>() {
                        Ok(n) => if n < 59 || n > 76 {return (false,"Out of bounds".to_string())}
                        _ => return (false, "Not a number".to_string())
                    }
                }
                _ => return (false, "Not cm or in".to_string())
            }
        }
        "hcl" => {
            if !RE_HCL.is_match(val) {
                return (false, "Regex failed!".to_string());
            }
        }
        "ecl" => {
            match val.as_str() {
                "amb" => (),
                "blu" => (),
                "brn" => (),
                "gry" => (),
                "grn" => (),
                "hzl" => (),
                "oth" => (),
                _ => return (false, "Not a valid color".to_string())
            }
        }
        "pid" => {
            if !RE_PID.is_match(val) {
                return (false, "Failed regex".to_string());
            }
        }
        "cid" => (), //println!("Ignoring CID"), 
        _ => println!("Not matched!")
    }

    return (true, "".to_string());
}
