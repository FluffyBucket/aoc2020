use std::{cmp::{max, min}, collections::{HashMap}, vec};

use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day17".to_string());
    part1(&input);
    part2(&input);
}


type Layer = Vec<Vec<bool>>;

// Vec coords: z,y,x
type Dimension = Vec<Layer>;


type Dimension4D = Vec<Vec<Vec<Vec<bool>>>>;

struct Coordinate  {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

struct Coordinate4D  {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

fn part1(input: &String) {
    let cube_size = input.lines().nth(0).unwrap().len();
    // Size will grow by 2 each cycle. (1 in each direction)
    let final_size = cube_size + 6*2;

    let mut dimension = vec![vec![vec![false; final_size]; final_size]; final_size];

    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '#' => dimension[1+(final_size-cube_size)/2][y + (final_size-cube_size)/2][x + (final_size-cube_size)/2] = true,
                _ => ()
            }
        }
    }
    println!("C size {}", cube_size);
    println!("F size {}", final_size);

    //print_dimension(dimension);
    //print_layer(&dimension[(final_size/2)]);
    let mut new_dimension = dimension.clone();
    for i in 0..6 {
        println!("Cycle: {}",i);
        let mut count = 0;
        for z in 0..final_size {
            for y in 0..final_size {
                for x in 0..final_size {
                    let n = get_active_neighbors(Coordinate {x:x as i32,y: y as i32,z: z as i32}, &dimension);
                    
                    let cube = dimension.get(z).unwrap().get(y).unwrap().get(x).unwrap();
                    let new_cube = new_dimension.get_mut(z).unwrap().get_mut(y).unwrap().get_mut(x).unwrap();
                    match cube {
                        true => if !(n == 2 || n == 3) {*new_cube = false} else {*new_cube = true; count+=1},
                        _ => if n == 3 {*new_cube = true; count += 1} else {*new_cube = false}
                    }
                }
            }   
        }
        println!("Count: {}", count);
        dimension = new_dimension.clone();
    }
    //print_dimension(new_dimension);
}

fn get_active_neighbors(c: Coordinate, d: &Dimension) -> i32{
    let mut count = 0;
    for z in (c.z -1)..(c.z+2) {
        for y in (c.y -1)..(c.y+2) {
            for x in (c.x -1)..(c.x+2) {
                if !(x == c.x && y == c.y && z == c.z) {
                    if let Some(y_vec) = d.get(max(z, 0) as usize) {
                        if let Some(x_vec) = y_vec.get(max(y, 0) as usize) {
                            if let Some(n) = x_vec.get(max(x, 0) as usize) {
                                if *n {
                                    count += 1;
                                }
                            }    
                        }
                    }
                }
            }
        }
    }

    return count;
}

fn part2(input: &String) {
    let cube_size = input.lines().nth(0).unwrap().len();
    // Size will grow by 2 each cycle. (1 in each direction)
    let final_size = cube_size + 6*2;

    let mut dimension = vec![
        vec![vec![vec![false; final_size]; final_size]; final_size]
        ; final_size];

    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '#' => dimension[1+(final_size-cube_size)/2][1+(final_size-cube_size)/2][y + (final_size-cube_size)/2][x + (final_size-cube_size)/2] = true,
                _ => ()
            }
        }
    }
    println!("C size {}", cube_size);
    println!("F size {}", final_size);

    //print_dimension(dimension);
    //print_layer(&dimension[(final_size/2)]);
    let mut new_dimension = dimension.clone();
    for i in 0..6 {
        println!("Cycle: {}",i);
        let mut count = 0;
        for w in 0..final_size {
            for z in 0..final_size {
                for y in 0..final_size {
                    for x in 0..final_size {
                        let n = get_active_neighbors_4D(Coordinate4D {x:x as i32,y: y as i32,z: z as i32, w: w as i32}, &dimension);
                        
                        let cube = dimension.get(w).unwrap().get(z).unwrap().get(y).unwrap().get(x).unwrap();
                        let new_cube = new_dimension.get_mut(w).unwrap().get_mut(z).unwrap().get_mut(y).unwrap().get_mut(x).unwrap();
                        match cube {
                            true => if !(n == 2 || n == 3) {*new_cube = false} else {*new_cube = true; count+=1},
                            _ => if n == 3 {*new_cube = true; count += 1} else {*new_cube = false}
                        }
                    }
                }   
            }
        }
        println!("Count: {}", count);
        dimension = new_dimension.clone();
    }
    //print_dimension(new_dimension);

}

fn get_active_neighbors_4D(c: Coordinate4D, d: &Dimension4D) -> i32{
    let mut count = 0;
    for w in (c.w -1)..(c.w+2) {
        for z in (c.z -1)..(c.z+2) {
            for y in (c.y -1)..(c.y+2) {
                for x in (c.x -1)..(c.x+2) {
                    if !(x == c.x && y == c.y && z == c.z && w == c.w) {
                        if let Some(z_vec) = d.get(max(w, 0) as usize) {
                            if let Some(y_vec) = z_vec.get(max(z, 0) as usize) {
                                if let Some(x_vec) = y_vec.get(max(y, 0) as usize) {
                                    if let Some(n) = x_vec.get(max(x, 0) as usize) {
                                        if *n {
                                            count += 1;
                                        }
                                    }    
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return count;
}


fn print_layer(l: &Layer) {
    for row in l {
        for col in row {
            match col {
                true => print!("#"),
                false => print!(".")
            }
        }
        print!("\n");
    }

}

fn print_dimension(d: Dimension) {
    let size = d[0].len()/2;
    for (i,l) in d.iter().enumerate() {
        println!("Layer: {}\n", (i as i32)-(size as i32));
        print_layer(l)
    }
}


