use std::collections::HashMap;

#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> u32 {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for l in input.lines() {
        let mut split = l.split("   ");
        let left = split.next().unwrap().parse::<u32>().unwrap();
        let right = split.next().unwrap().parse::<u32>().unwrap();
        lefts.push(left);
        rights.push(right);
    }
    lefts.sort();
    rights.sort();
    let mut sum = 0;
    for i in 0..lefts.len() {
        let left = lefts[i];
        let right = rights[i];
        sum += if left > right { left - right } else { right - left } as u32;
    }
    sum
}

#[aoc(day1, part2, Chars)]
pub fn part2(input: &str) -> u32 {
    let mut lefts = Vec::new();
    let mut rights =HashMap::<u32, u32>::new();
    for l in input.lines() {
        let mut split = l.split("   ");
        let left = split.next().unwrap().parse::<u32>().unwrap();
        let right = split.next().unwrap().parse::<u32>().unwrap();
        lefts.push(left);
        *rights.entry(right).or_default() += 1;
    }

    let mut sum = 0;
    for i in 0..lefts.len() {
        let left = lefts[i];
        let count = rights.get(&left).unwrap_or(&0);
        sum += left * count;
    }
    sum
}