use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    arg: i32
}

pub fn run() {
    let input = load_input("day8".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {

    let (instructions,_) = get_instructions(input);
    // for i in &instructions {
    //     println!("{:?}",i);
    // }
    let mut visited: HashMap<usize, i32> = HashMap::new();
    let mut count = 0;
    let mut index = 0;
    while index < instructions.len() {
        //println!("Index: {}", index);
        if visited.contains_key(&index) {
            break;
        } else {
            visited.insert(index, 0);
        }
        //println!("Inst: {:?}", instructions[index]);

        match instructions[index].op.as_str() {
            "nop" => (),
            "acc" => count += instructions[index].arg,
            "jmp" => index = ((index as i32) + instructions[index].arg - 1) as usize,
            op => println!("Unknown op: {}", op)
        }

        index += 1;
    }
    println!("Part1: {}", count);
}

fn part2(input: &String) {
    let (instructions, jmp_nop_index) = get_instructions(input);
    // for i in &instructions {
    //     println!("{:?}",i);
    // }
    
    for index in jmp_nop_index {
        let mut modified_instructions = instructions.clone();
        let inst = &modified_instructions[index];
        match inst.op.as_str() {
            "nop" => modified_instructions[index].op = "jmp".to_string(), 
            "jmp" => modified_instructions[index].op = "nop".to_string(),
            _ => ()
        }

        let (count, completion) = run_instructions(&modified_instructions);
        if completion {
            println!("Part2: {}", count);
            break;
        }
    }

    
}


fn run_instructions(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut visited: HashMap<usize, i32> = HashMap::new();
    let mut count = 0;
    let mut index = 0;
    while index < instructions.len() {
        //println!("Index: {}", index);
        if visited.contains_key(&index) {
            return (0, false);
        } else {
            visited.insert(index, 0);
        }
        //println!("Inst: {:?}", instructions[index]);

        match instructions[index].op.as_str() {
            "nop" => (),
            "acc" => count += instructions[index].arg,
            "jmp" => index = ((index as i32) + instructions[index].arg - 1) as usize,
            op => println!("Unknown op: {}", op)
        }

        index += 1;
    }
    return (count, true)
}

fn get_instructions(input: &String) -> (Vec<Instruction>, Vec<usize>) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut jmp_nop_index: Vec<usize> = Vec::new();
    for (index,line) in input.lines().enumerate() {
        let line_split = line.split(" ").collect::<Vec<&str>>();
        let op = line_split[0].trim().to_string();
        match op.as_str() {
            "jmp" => jmp_nop_index.push(index),
            "nop" => jmp_nop_index.push(index),
            _ => ()
        }
        
        instructions.push(Instruction {
            op: op,
            arg: line_split[1].trim().parse::<i32>().unwrap()
        });
    }

    return (instructions, jmp_nop_index);
}