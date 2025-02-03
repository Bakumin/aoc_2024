use std::fs;
use std::str::FromStr;

pub fn part1() {
    let mut contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines = contents.lines();


    let l = lines.map(|line| {
        let (l,r) = line.split_once("   ").unwrap();
        (i32::from_str(l).unwrap(), i32::from_str(r).unwrap())
    }).collect::<Vec<(i32, i32)>>();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for row in l {
        left.push(row.0);
        right.push(row.1);
    };

    left.sort();
    right.sort();


    let mut result = 0;
    for i in 0..left.len() {
        println!("left: {}, right: {}, diff: {}", left[i], right[i], left[i].abs_diff(right[i]));
        result += left[i].abs_diff(right[i]);
    }
    println!("{result}");
}

pub fn part2() {
    let mut contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines = contents.lines();


    let l = lines.map(|line| {
        let (l,r) = line.split_once("   ").unwrap();
        (i32::from_str(l).unwrap(), i32::from_str(r).unwrap())
    }).collect::<Vec<(i32, i32)>>();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for row in l {
        left.push(row.0);
        right.push(row.1);
    };

    left.sort();
    right.sort();


    let mut result = 0;
    for i in 0..left.len() {
        result+= left[i] * right.iter().filter(|v| v == &&left[i]).count() as i32;

    }
    println!("{result}");
}