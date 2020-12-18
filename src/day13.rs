use modinverse::*;

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day13".to_string());
    part1(&input);
    part2(&input);
}



fn part1(input: &String) {
    let depart_time = input.lines().into_iter().nth(0).unwrap().parse::<i64>().unwrap();
    let busses = input.lines().into_iter().nth(1).unwrap().split(",").collect::<Vec<&str>>();
    let mut earliest_time = i64::MAX;
    let mut bus = 0;
    for b in busses {
        match b {
            "x" => (),
            s => {
                let n = s.parse::<i64>().unwrap();
                let time = n - (depart_time % n);
                if time < earliest_time {
                    earliest_time = time;
                    bus = n;
                }
            }
            _ => ()
        }
    }

    println!("Part1: time: {}, bus: {}, the mult: {}", earliest_time, bus, earliest_time * bus);
}

// Not really my solution. I am ashamed... :'(
fn part2(input: &String) {

    let bus_data = input.lines().into_iter().nth(1).unwrap().split(",").collect::<Vec<&str>>();
    let mut busses: Vec<(i64,i64)> = Vec::new();

    for (i,b) in bus_data.iter().enumerate() {
        match *b {
            "x" => (),
            s => {
                let n = s.parse::<i64>().unwrap();
                busses.push((i as i64, n));
            }
            _ => ()
        }
    } 
    println!("{:?}", busses);

    let product = busses
        .iter()
        .map(|(_,id)| id)
        .product::<i64>();

    let res = busses
        .iter()
        .map(|(i,id)| {
            let product_without_id = product / id;
            let modular_inverse = modinverse(product_without_id, *id).unwrap();
            (id - i % id) * product_without_id * modular_inverse
        })
        .sum::<i64>()
        % product;

    println!("Part2: {}", res)
}

