use std::{f64, fs};
use itertools::Itertools;
use regex::Regex;

pub fn part1() {
    let contents = fs::read_to_string("inputs/day13.txt").unwrap();

    let re_a = Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_b = Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    let mut results = 0;
    let mut row_count = 0;

    for row in re_a.captures_iter(contents.as_ref()).zip(re_b.captures_iter(contents.as_ref())).zip(re_prize.captures_iter(contents.as_ref())) {
        let mut i = 0;
        let mut j = 0;
        let mut lowest_score = f64::MAX;
        let a = row.1.name("x").unwrap().as_str().parse::<f64>().unwrap();
        let b = row.0.1.name("x").unwrap().as_str().parse::<f64>().unwrap();
        let c = row.0.0.name("x").unwrap().as_str().parse::<f64>().unwrap();

        let d = row.1.name("y").unwrap().as_str().parse::<f64>().unwrap();
        let e = row.0.1.name("y").unwrap().as_str().parse::<f64>().unwrap();
        let f = row.0.0.name("y").unwrap().as_str().parse::<f64>().unwrap();
        for counter in 0..=100 {
            let r1 = (a - (b * counter as f64))/c;
            let r2 = ( d-  (e* counter as f64))/f;
            if r1 == r2 && r1 >= 0.0 && r2 >= 0.0 && r1 < 101.0 && r2 < 101.0 {
                let i = counter as f64;
                let j = r2;
                println!("{counter},{r1} matches for Row {row_count}");
                if (3.0*j + i)  < lowest_score {
                    lowest_score = 3.0*j + i;
                }
                else {
                    println!("Duplicate Match:\n({a}-{b}*{counter})/{c} = {r1}\n({d}-{e}*{counter})/{f} = {r2}");
                }
            }
        }
        row_count+=1;
        if lowest_score < f64::MAX {
            results += lowest_score as i32;
        }
    }

    println!("{}", results);

}

pub fn part2() {
    let contents = fs::read_to_string("inputs/day13.txt").unwrap();

    let re_a = Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_b = Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    let mut results = 0;
    let mut row_count = 0;

    for row in re_a.captures_iter(contents.as_ref()).zip(re_b.captures_iter(contents.as_ref())).zip(re_prize.captures_iter(contents.as_ref())) {
        let mut i = 0;
        let mut j = 0;
        let mut lowest_score = f64::MAX;
        let a = row.1.name("x").unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0 ;
        let b = row.0.1.name("x").unwrap().as_str().parse::<f64>().unwrap();
        let c = row.0.0.name("x").unwrap().as_str().parse::<f64>().unwrap();

        let d = row.1.name("y").unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;
        let e = row.0.1.name("y").unwrap().as_str().parse::<f64>().unwrap();
        let f = row.0.0.name("y").unwrap().as_str().parse::<f64>().unwrap();

        let x = ((d*c- a*f)/ (-b*f +c*e));
        //let x = -1.0 * ((d*c- a*f)/ (b*f -c*e));
        println!("(({d}*{c} - {a}*{f} / ({c}*{e} - {b} * {f})");

        if x > 0.0 && x.fract() == 0.0 {
            let y = (a - (b * x))/c;
            println!("Checking {row_count}: X: {x}, Y: {y}");
            println!("Results:\nX: {}\nY: {}\nCorrect:X: {}, Y: {}", b*x+c*y, e*x+f*y, a, d);
            if y > 0.0 && y.fract() == 0.0 {
                results += x as usize + y as usize * 3;
                println!("CONFIRMED {row_count}: X: {x}, Y: {y}");

            }
        }
        row_count+=1;
    }

    println!("{}", results);

}