use helpers::helpers::load_input;
use crate::helpers;

pub fn run() {
    let input = load_input("day18".to_string());
    part1(&input);
    part2(&input);
}

enum Op {
    Add,
    Mul,
}

fn part1(input: &String) {
    let mut expr = input.trim_end().split(" ").collect::<Vec<&str>>();
    println!("{:?}", expr);

    expr.reverse();
    
    println!("{:?}", evaluate2(&expr));


    println!("{:?}", evaluate("1 + 1 + 1"));
}

fn evaluate2(expr: &Vec<&str>) -> i64 {
    let mut sum = 0;
    for (i,e) in expr.iter().enumerate() {
        println!("{}, sum: {}", e, sum);
        match expr[i] {
            "+" => return sum + evaluate2(&expr[i+1..].to_vec()),
            "*" => {
                let eval = evaluate2(&expr[i+1..].to_vec());
                //println!("{} * {}", sum, eval);
                return sum * eval
            },
            "(" => {
            },
            ")" => {
            },
            s => {
                let n = s.parse::<i64>().unwrap(); 
                sum = n;
            }
        }
    }


    return sum;
}


fn evaluate(input: &str) -> Option<i64> {
    let mut value = None;
    let mut op = Op::Add;
    let mut mem: Option<i64> = None;
    let mut str_num = "".to_owned();
    let mut num: Option<i64> = None;
    for (i,c) in input.chars().enumerate() {
        match c {
            ' ' => {
                if str_num != "" {
                    num = Some(str_num.parse::<i64>().unwrap());
                }
                str_num = "".to_owned();

                match mem {
                    None => mem = num,
                    Some(other) => {
                        match op {
                            Op::Add => value = Some(value.unwrap_or(0) + (other + num.unwrap_or(0))),
                            Op::Mul => value = Some(value.unwrap_or(0) + (other * num.unwrap_or(1))),
                        }
                        mem = None;
                        num = None;
                    }
                }
            },
            '(' => {
                let eval = evaluate(&input[i..]).unwrap_or(0);
                match op {
                    Op::Add => value = Some(value.unwrap_or(0) + eval),
                    Op::Mul => value = Some(value.unwrap_or(0) * eval),
                }
                
            }
            ')' => return value,
            '+' => op = Op::Add,
            '*' => op = Op::Mul,
            n => {
                str_num = format!("{}{}", str_num, n);
            }
        }
    }

    return value
}

fn part2(input: &String) {


}



