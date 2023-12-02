use utils::read_lines;
use std::char;

pub fn solve_part1(filename: &str) -> u32 {
    let lines = read_lines(filename);
    let mut sum = 0;
    for line in lines {
        let digits: Vec<u32> = line
            .matches(|c| char::is_numeric(c))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let head = digits[0];
        let tail = digits[digits.len() - 1];
        // println!("{line} -> {:?} -> {}{}", digits, head, tail);
        sum += (head.to_string() + &tail.to_string()).parse::<u32>().unwrap();
    }
    sum
}

pub fn solve_part2(filename: &str) -> u32 {
    let lines = read_lines(filename);
    let mut sum = 0;
    for line in lines {
        let digits: Vec<u32> = line
            .replace("nineight", "9")
            .replace("eightwo", "8")
            .replace("eighthree", "8")
            .replace("twone", "2")
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
            .matches(|c| char::is_numeric(c))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let head = digits[0];
        let tail = digits[digits.len() - 1];
        // println!("{line} -> {:?} -> {}{}", digits, head, tail);
        sum += (head.to_string() + &tail.to_string()).parse::<u32>().unwrap();
    }
    sum
}

