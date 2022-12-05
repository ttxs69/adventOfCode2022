use std::cmp::Reverse;
use std::fs;

fn part1(input:&str) -> usize {
    let mut total = 0;
    let mut max_total = 0;
    for line in input.lines() {
        if line.is_empty() {
            max_total = if total > max_total {
                total
            } else {
                max_total
            };
            total = 0;
        } else {
            total += line.parse::<usize>().unwrap();
        }
    }
    max_total
}

fn part2(input:&str) -> usize {
    let mut each_total = Vec::new();
    let mut total = 0;
    
    for line in input.lines() {
        if line.is_empty() {
            each_total.push(total);
            total = 0;
        } else {
            total += line.parse::<usize>().unwrap();
        }
    }

    each_total.sort_by_key(|k| Reverse(*k));
    each_total[0] + each_total[1] + each_total[2]
}

pub fn run() {
    let input = fs::read_to_string("data/day01.txt").unwrap();
    println!("day01.part1 = {}", part1(&input));    
    println!("day01.part2 = {}", part2(&input));   
}