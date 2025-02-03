use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;
use regex::Regex;

pub fn part1() {
    let mut contents = fs::read_to_string("inputs/day5.txt").unwrap();
    let lines = contents.lines();

    let mut rules = Vec::new();
    let mut books = Vec::new();

    rules = lines.clone().filter(|line| line.contains("|")).map(|line| {
        let (before, after) = line.split_once("|").unwrap();
        (u64::from_str(before).unwrap(), u64::from_str(after).unwrap())
    }).collect();
    books = lines.filter(|line| line.contains(",")).map(|line| {
        line.split(",").map(|value| u64::from_str(value).unwrap()).collect::<Vec<u64>>()
    }).collect();
    let mut result = 0;

    books.iter().for_each(|book| {
        let mut correct = true;
        'inner: for i in 1..book.len() {
            if rules.iter().filter(|rule| rule.0 == book[i]).any(|rule| book[..i].contains(&rule.1))
            {
                correct = false;
                break 'inner
            }
        }
        if correct {
            result += book[book.len()/2];
        }
    });



    println!("{result}");

}

pub fn part2() {
    let mut contents = fs::read_to_string("inputs/day5.txt").unwrap();
    let lines = contents.lines();

    let mut rules = Vec::new();
    let mut books = Vec::new();

    rules = lines.clone().filter(|line| line.contains("|")).map(|line| {
        let (before, after) = line.split_once("|").unwrap();
        (u64::from_str(before).unwrap(), u64::from_str(after).unwrap())
    }).collect();
    books = lines.filter(|line| line.contains(",")).map(|line| {
        line.split(",").map(|value| u64::from_str(value).unwrap()).collect::<Vec<u64>>()
    }).collect();
    let mut result = 0;

    books.iter_mut().for_each(|mut book| {
        let mut correct = true;
        'inner: for i in 1..book.len() {
            if rules.iter().filter(|rule| rule.0 == book[i]).any(|rule| book[..i].contains(&rule.1))
            {
                correct = false;
                break 'inner
            }
        }
        if !correct {
            book.sort_by(|a, b| {
                let rule = rules.iter().find(|v| (v.0 == *a && v.1 == *b) || (v.1 == *a && v.0 == *b));
                match rule {
                    None => {
                        return Ordering::Equal;
                    }
                    Some(rule) => {
                        if rule.0 == *a  {
                            return Ordering::Greater
                        }
                        else {
                            return Ordering::Less
                        }
                    }
                }
            });
            result += book[book.len()/2];
        }
    });


    println!("{result}");
}