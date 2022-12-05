use std::fs;
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let input2num = HashMap::from([('X', 1),('Y', 2),('Z', 3)]);
    let win = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);
    let mut score = 0;
    for line in input.lines() {
        let mut pair = line.chars();
        let opponent = pair.next().unwrap();
        pair.next();
        let me = pair.next().unwrap();
        score += *input2num.get(&me).unwrap();
        if me == *win.get(&opponent).unwrap() {
            score += 6;
        } else if me == *draw.get(&opponent).unwrap() {
            score += 3;
        } else {
            score += 0;
        }
    }
    score
}

fn part2(input: &str) -> usize {
    let input2num = HashMap::from([('X', 1),('Y', 2),('Z', 3)]);
    let win = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);
    let loss = HashMap::from([('A', 'Z'), ('B', 'X'), ('C', 'Y')]);
    let mut score = 0;
    for line in input.lines() {
        let mut pair = line.chars();
        let opponent = pair.next().unwrap();
        pair.next();
        let result = pair.next().unwrap();
        match result {
            'X' => {
                score += *input2num.get(loss.get(&opponent).unwrap()).unwrap();
            },
            'Y' => {
                score += *input2num.get(draw.get(&opponent).unwrap()).unwrap() + 3;
            },
            'Z' => {
                score += *input2num.get(win.get(&opponent).unwrap()).unwrap() + 6;
            }
            _ => unreachable!()
        }
    }
    score
}

pub fn run() {
    let input = fs::read_to_string("data/day02.txt").unwrap();
    println!("day02.part1 = {}", part1(&input));
    println!("day02.part2 = {}", part2(&input));
}