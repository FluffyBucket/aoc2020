use std::{cmp::max, collections::HashMap};

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day12".to_string());
    part1(&input);
    part2(&input);
}



fn part1(input: &String) {
    let mut curr_dir = 0;
    let mut east = 0;
    let mut north = 0;
    for line in input.lines() {
        let cmd = line.chars().nth(0);
        let num = &line[1..].parse::<i64>().unwrap();
        match cmd {
            Some('N') => {
                north += num;
            }
            Some('S') => {
                north -= num;
            }
            Some('E') => {
                east += num;
            }
            Some('W') => {
                east -= num;
            }
            Some('L') => {
                curr_dir = new_direction(curr_dir, true, *num);
            }
            Some('R') => {
                curr_dir = new_direction(curr_dir, false, *num);
            }
            Some('F') => {
                match curr_dir {
                    0 => east += num,
                    90 => north += num,
                    180 => east -= num,
                    270 => north -= num,
                    _ => println!("Err wrong dir value {}", curr_dir)
                }
            }
            _ => ()
        }
    }

    println!("Part1: n: {}, e: {}, manhat: {}", north, east, (north.abs() + east.abs()));

}

fn new_direction(curr_dir: i64, left: bool, deg: i64) -> i64 {
    if left {
        return (curr_dir + deg) % 360;
    } else {
        return (((curr_dir - deg) % 360) + 360)%360;
    }
}

struct Point {
    pub east: i64,
    pub north: i64
}

impl Point {
    fn rotate(&mut self, deg: i64) {
        let x = self.east.clone();
        let y = self.north.clone();
        if deg != 0 {
            // Clockwise
            if deg.is_negative() {
                self.east = y;
                self.north = -x;
                self.rotate(deg+90);
            } 
            // Counter clockwise
            else {
                self.east = -y;
                self.north = x;
                self.rotate(deg-90);
            }
        }
    }
}

fn part2(input: &String) {
    let mut waypoint = Point {east: 10, north: 1};
    let mut ship = Point {east: 0, north: 0};
    //println!("{}, {}", waypoint.east, waypoint.north); 


    for line in input.lines() {
        //println!("CMD: {}", line);
        let cmd = line.chars().nth(0);
        let num = &line[1..].parse::<i64>().unwrap();
        match cmd {
            Some('N') => {
                waypoint.north += num;
            }
            Some('S') => {
                waypoint.north -= num;
            }
            Some('E') => {
                waypoint.east += num;
            }
            Some('W') => {
                waypoint.east -= num;
            }
            Some('L') => {
                waypoint.rotate(*num);
            }
            Some('R') => {
                waypoint.rotate(-*num);
            }
            Some('F') => {
                ship.east += num * waypoint.east;
                ship.north += num * waypoint.north;
            }
            _ => ()
        }
        // println!("Waypoint: {},{}", waypoint.east, waypoint.north);
        // println!("Ship: {},{}", ship.east, ship.north);
    }

    println!("Part2: n: {}, e: {}, manhat: {}", ship.north, ship.east, (ship.north.abs() + ship.east.abs()));

    

}
