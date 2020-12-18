use std::collections::HashMap;

use helpers::helpers::load_input;
use crate::helpers;



pub fn run() {
    let input = load_input("day5".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut max = 0;
    for line in input.lines() {
        //println!("The row: {}", &line[..7]);
        let row = find_row(&line[..7].to_string(), 0, 127);
        let col = find_column(&line[7..].to_string(), 0, 7);

        let sum = row * 8 + col;
        if sum > max {
            max = sum;
        }
        
        //println!("ROW is: {}", row);
        //println!("COL is: {}", col);
    }

    println!("Max: {}", max);
}

fn part2(input: &String) {
    let mut rows: HashMap<usize, HashMap<usize, usize>> = HashMap::new();


    for line in input.lines() {
        //println!("The row: {}", &line[..7]);
        let row_i = find_row(&line[..7].to_string(), 0, 127);
        let col_i = find_column(&line[7..].to_string(), 0, 7);

        let id = row_i * 8 + col_i;
        if rows.contains_key(&row_i) {
            let mut row = rows.get_mut(&row_i).unwrap();
            row.insert(col_i, id);
        } else {
            let mut row: HashMap<usize, usize> = HashMap::new();
            row.insert(col_i, id);
            rows.insert(row_i, row);
        }
        
        //println!("ROW is: {}", row);
        //println!("COL is: {}", col);
    }

    //print_plane_seats(rows);
    for row_i in 0..128 {
        match rows.get(&row_i) {
            Some(row) => {
                if row.len() == 7 {
                    for col_i in 0..8 {
                        match row.get(&col_i) {
                            None => println!("EMPTY SEAT: {}", row_i * 8 + col_i),
                            _ => ()
                        }
                    }
                }
            }
            None => ()
        }
    }
}

fn print_plane_seats(rows: HashMap<usize, HashMap<usize,usize>>) {

    for row_i in 0..128 {
        match rows.get(&row_i) {
            Some(row) => {
                print_plane_col(row);
                print!("\n");
            }
            None => println!("_ _ _ _ _ _ _ _ ")
        }
    }


}

fn print_plane_col(row: &HashMap<usize, usize>) {
    for col_i in 0..8 {
        match row.get(&col_i) {
            Some(_) => print!("X "),
            None => print!("_ ")
        }
    }
}

fn find_row(row: &String, left: usize, right: usize) -> usize {
    //println!("Row: {}, left: {}, right: {}", row, left, right);

    if left == right {
        return left;
    }

    match &row[..1] {
        "F" => return find_row(&row[1..].to_string(), left, right - ((right+1)-left)/2),
        "B" => return find_row(&row[1..].to_string(), left + ((right+1)-left)/2, right),
        _ => panic!("NOTHING! Err")
    }
}

fn find_column(col: &String, left: usize, right: usize) -> usize {
    //println!("Col: {}, left: {}, right: {}", col, left, right);

    if left == right {
        return left;
    }

    match &col[..1] {
        "L" => return find_column(&col[1..].to_string(), left, right - ((right+1)-left)/2),
        "R" => return find_column(&col[1..].to_string(), left + ((right+1)-left)/2, right),
        _ => panic!("NOTHING! Err")
    }
}