
use std::{collections::HashMap, iter::repeat};

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day14".to_string());
    part1(&input);
    part2(&input);
}


fn part1(input: &String) {
    let mut memory: HashMap<u32, u64> = HashMap::new();
    let mut mask = String::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line[7..].to_string();
            //println!("Mask: {}", mask);

        } else {
            let split = line.split("] = ").collect::<Vec<&str>>();
            let value =  format!("{:#036b}",split[1].parse::<u64>().unwrap());
            let location = &split[0][4..].parse::<u32>().unwrap();
            let new_value = write_bits(&mask, &value);
            let mem_value = memory.entry(*location).or_insert(new_value);
            *mem_value = new_value;
        }
    }

    //println!("{:?}", memory);
    println!("Part1: {}", memory.values().sum::<u64>())
}

fn write_bits(mask: &String, value: &String) -> u64 {
    let mut new_value: Vec<char> = Vec::new();
    // println!("Mask: {}", mask);
    // println!("Value: {}", value);
    for (i, b) in mask.chars().enumerate() {
        //println!("{}, {}", i,b);
        match b {
            'X' => new_value.push(value.chars().nth(i).unwrap()),
            _   => new_value.push(mask.chars().nth(i).unwrap())
        }
    }

    return new_value.iter().zip(
        (0..(new_value.len() as u32)).collect::<Vec<u32>>().iter().rev()
    ).map(|(c,i)| if *c == '1' {(2 as u64).pow(*i)} else {0} ).sum::<u64>();
}

fn part2(input: &String) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line[7..].to_string();
            //println!("Mask: {}", mask);

        } else {
            let split = line.split("] = ").collect::<Vec<&str>>();
            let value =  split[1].parse::<u64>().unwrap();
            let location = format!("{:#036b}", split[0][4..].parse::<u64>().unwrap());

            let raw_adr = calc_addresses(&mask, &location);

            let addresses = raw_adr.iter()
                .map(|adr| adr.iter()
                    .zip((0..(adr.len() as u32)).collect::<Vec<u32>>().iter().rev())
                        .map(|(c,i)| if *c == '1' {(2 as u64).pow(*i)} else {0} ).sum::<u64>()
            ).collect::<Vec<u64>>();
            for adr in addresses {
                let mem_value = memory.entry(adr).or_insert(value);
                *mem_value = value;
            }
            // let new_value = write_bits(&mask, &value);
            // let mem_value = memory.entry(*location).or_insert(new_value);
            // *mem_value = new_value;
        }
    }

    //println!("{:?}", memory);
    println!("Part2: {}", memory.values().sum::<u64>());
}

fn calc_addresses(mask: &String, address: &String) -> Vec<Vec<char>> {
    let mut addresses: Vec<Vec<char>> = Vec::new();
    let mut adr = Vec::new();
    if address.len() == 1 {
        match mask.chars().nth(0).unwrap() {
            '0' => return vec![vec![address.chars().nth(0).unwrap()]],
            '1' => return vec![vec!['1']],
            _ => return vec![vec!['0'], vec!['1']]
        }
    }

    for (i, b) in mask.chars().enumerate() {
        
        match b {
            '0' => adr.push(address.chars().nth(i).unwrap()),
            '1' => adr.push(mask.chars().nth(i).unwrap()),
            _   => {
                let adrs = calc_addresses(&mask[i+1..].to_string(), &address[i+1..].to_string());
                for (i,n_adr) in adrs.iter().enumerate() {
                    addresses.push([adr.clone(), ['0'].to_vec(), n_adr.clone()].concat());
                    addresses.push([adr.clone(), ['1'].to_vec(), n_adr.clone()].concat());
                }
                return addresses;
            }
        
        }
        
    }

    addresses.push(adr);
    return addresses;
    // return addresses.iter()
    //     .map(|adr| adr.iter()
    //         .zip((0..(adr.len() as u32)).collect::<Vec<u32>>().iter().rev())
    //             .map(|(c,i)| if *c == '1' {(2 as u64).pow(*i)} else {0} ).sum::<u64>()
    // ).collect::<Vec<u64>>();
}

