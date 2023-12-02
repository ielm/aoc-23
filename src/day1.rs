use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> io::Result<()> {
    println!("Day 1: Trebuchet?!\n");
    part1()?;
    part2()?;

    Ok(())
}

pub fn part1() -> io::Result<()> {
    let path = Path::new("src/resources/day1.txt");
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let first = line.chars().find(|c| c.is_ascii_digit());
        let last = line.chars().rev().find(|c| c.is_ascii_digit());

        if let (Some(f), Some(l)) = (first, last) {
            let value = format!("{}{}", f, l).parse::<i32>().unwrap();
            total_sum += value;
        }
    }

    println!("Part 1:");
    println!("total_sum: {}\n", total_sum);
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let path = Path::new("src/resources/day1.txt");
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let digit_map = create_digit_map();
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let first = find_first_digit(&line, &digit_map);
        let last = find_last_digit(&line, &digit_map);

        if let (Some(f), Some(l)) = (first, last) {
            let value = format!("{}{}", f, l).parse::<i32>().unwrap();
            total_sum += value;
        }
    }

    println!("Part 2:");
    println!("total_sum: {}\n", total_sum);
    Ok(())
}

fn create_digit_map() -> HashMap<String, char> {
    let mut map = HashMap::new();
    map.insert("zero".to_string(), '0');
    map.insert("one".to_string(), '1');
    map.insert("two".to_string(), '2');
    map.insert("three".to_string(), '3');
    map.insert("four".to_string(), '4');
    map.insert("five".to_string(), '5');
    map.insert("six".to_string(), '6');
    map.insert("seven".to_string(), '7');
    map.insert("eight".to_string(), '8');
    map.insert("nine".to_string(), '9');
    map
}

fn find_first_digit(line: &str, digit_map: &HashMap<String, char>) -> Option<char> {
    let mut digit_str = String::new();

    // create set of chars of last char in the keys of digit_map with no duplicates
    let mut last_chars = digit_map
        .keys()
        .map(|s| s.chars().last().unwrap())
        .collect::<Vec<char>>();

    last_chars.dedup();

    let mut first_chars = digit_map
        .keys()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    first_chars.sort();
    first_chars.dedup();

    for c in line.chars() {
        if c.is_alphabetic() {
            digit_str.push(c);
            for i in (0..digit_str.len()).rev() {
                let sub_str = &digit_str[i..];
                if !first_chars.contains(&sub_str.chars().next().unwrap()) {
                    continue;
                } else if let Some(digit) = digit_map.get(sub_str) {
                    digit_str.clear();
                    return Some(*digit);
                }
            }
        } else {
            digit_str.clear();
            if c.is_ascii_digit() {
                return Some(c);
            }
        }
    }
    None
}

fn find_last_digit(line: &str, digit_map: &HashMap<String, char>) -> Option<char> {
    let mut digit_str = String::new();
    for c in line.chars().rev() {
        if c.is_alphabetic() {
            digit_str.insert(0, c);
            for i in 0..digit_str.len() {
                let sub_str = &digit_str[0..i + 1];
                if let Some(digit) = digit_map.get(sub_str) {
                    return Some(*digit);
                }
            }
        } else if c.is_ascii_digit() {
            return Some(c);
        } else {
            digit_str.clear();
            if c.is_ascii_digit() {
                return Some(c);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part_2() {
        let string = "rrzbgtfrrqkspsix3rkpzddzrbcrzvxzstjbqhmqq";
        let digit_map = create_digit_map();
        let first = find_first_digit(string, &digit_map);
        let last = find_last_digit(string, &digit_map);

        assert_eq!(first, Some('6'));
        assert_eq!(last, Some('3'));
    }
}
