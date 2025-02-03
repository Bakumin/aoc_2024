use std::fs;
use std::str::FromStr;
use regex::Regex;

pub fn part1()  {
    let mut contents = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;

    let regex = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();

    regex.captures_iter(&*contents).for_each(|found| {
        result += u32::from_str(found.name("first").unwrap().as_str()).unwrap() * u32::from_str(found.name("second").unwrap().as_str()).unwrap();

    });



    println!("{result}");
}

pub fn part2() {
    let mut contents = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;

    let regex = Regex::new(r"(mul\((?<first>\d{1,3}),(?<second>\d{1,3})\))|(?<enable>do\(\))|(?<disable>don't\(\))").unwrap();
    let mut enabled = true;

    regex.captures_iter(&*contents).for_each(|found| {
        if found.name("enable").is_some() {
            enabled = true;
            return;
        }
        else if found.name("disable").is_some() {
            enabled = false;
            return;
        }
        else if found.name("first").is_some() && enabled{
            result += u32::from_str(found.name("first").unwrap().as_str()).unwrap() * u32::from_str(found.name("second").unwrap().as_str()).unwrap();

        }



    });
    println!("{result}");

}