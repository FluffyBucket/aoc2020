
use helpers::helpers::load_input;
use crate::helpers;



pub fn run() {
    let input = load_input("day2".to_string());
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut sum = 0;

    for line in input.lines() {
        let pol_pw = line.split(":").collect::<Vec<&str>>();
        let pw = pol_pw[1].trim();
        let lower = pol_pw[0].split("-").collect::<Vec<&str>>()[0].parse().unwrap();
        let upper = pol_pw[0].split("-").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
        let p_c = pol_pw[0].split("-").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[1].chars().next().unwrap();

        let mut count = 0;
        for c in pw.chars() {
            if c == p_c{
                count+=1;
            }
        }
        if count >= lower && count <= upper {
            sum += 1;
        }
    }

    println!("Part1: {}", sum);
}

fn part2(input: &String) {
    let mut count = 0;

    for line in input.lines() {
        let pol_pw = line.split(":").collect::<Vec<&str>>();
        let pw = pol_pw[1].trim();
        let pos1: usize = pol_pw[0].split("-").collect::<Vec<&str>>()[0].parse().unwrap();
        let pos2: usize = pol_pw[0].split("-").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
        let p_c = pol_pw[0].split("-").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[1].chars().next().unwrap();

        if (p_c == pw.chars().nth(pos1 - 1).unwrap()) ^ (p_c == pw.chars().nth(pos2 - 1).unwrap()) {
            count += 1;
        }
    }

    println!("Part2: {}", count);
}