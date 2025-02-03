use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Sub;
use std::str::FromStr;
use std::time::Instant;

pub fn part1 () {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day8.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;
    let grid = lines.map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for line in grid.iter().enumerate() {
        for c in line.1.iter().enumerate() {
            if c.1 != &'.' {
                let counter = locations.entry(*c.1).or_insert(vec![]);
                counter.push((line.0 as isize, c.0 as isize));
            }
        }
    }

    let mut points: HashSet<(isize, isize)> = HashSet::new();

    for ch in locations.iter() {
        for i in 0..ch.1.len() {
            for j in 0..ch.1.len() {
                if i == j {
                    continue;
                }
                let v = (ch.1[i].0.sub(ch.1[j].0), ch.1[i].1.sub(ch.1[j].1));
                let p1 = (ch.1[i].0 + v.0, ch.1[i].1 + v.1);
                let p2 = (ch.1[j].0 - v.0, ch.1[j].1 - v.1);
                if(p1.0 >= 0 && p1.1 >= 0 && p1.0 < grid.len() as isize && p1.1 < grid[0].len() as isize) {
                    points.insert(p1);
                }
                if(p2.0 >= 0 && p2.1 >= 0 && p2.0 < grid.len() as isize && p2.1 < grid[0].len() as isize) {
                    points.insert(p2);
                }
            }
        }
    }


    println!("{points:?}");
    println!("{}", points.len());
}

pub fn part2(){
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day8.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;
    let mut grid = lines.map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for line in grid.iter().enumerate() {
        for c in line.1.iter().enumerate() {
            if c.1 != &'.' {
                let counter = locations.entry(*c.1).or_insert(vec![]);
                counter.push((line.0 as isize, c.0 as isize));
            }
        }
    }

    fn minify_vec(v1: isize, v2: isize) ->(f64, f64) {
        let f1 = v1 as f64;
        let f2 = v2 as f64;
        return (f1,f2);
        let magnitude = ((f1*f1) + (f2*f2)).sqrt();
        let base_vec = (f1/magnitude, f2/magnitude);
        return base_vec;
    }

    let mut points: HashSet<(isize, isize)> = HashSet::new();
    let mut vectors: Vec<((f64, f64), (f64, f64))> = Vec::new();
    for ch in locations.iter() {
        for i in 0..ch.1.len() {
            for j in 0..ch.1.len() {
                if i == j {
                    continue;
                }
                let v = (ch.1[i].0.sub(ch.1[j].0), ch.1[i].1.sub(ch.1[j].1));
                vectors.push(((ch.1[j].0 as f64, ch.1[j].1 as f64), minify_vec(v.0, v.1)));
            }
        }
    }

    for v in vectors.clone() {
        let initial_point = v.0;
        points.insert((initial_point.0 as isize, initial_point.1 as isize));
        let mut current_point = (initial_point.0, initial_point.1);
        while current_point.0 >= 0.0 && current_point.0 < grid.len() as f64 && current_point.1 >= 0.0 && current_point.1 < grid[1].len() as f64 {
            if current_point.0.fract() == 0.0 && current_point.1.fract() == 0.0 {
                points.insert((current_point.0 as isize, current_point.1 as isize));
            }
            current_point.0 += v.1.0;
            current_point.1 += v.1.1;

        }
        let mut current_point = (initial_point.0, initial_point.1);
        while current_point.0 >= 0.0 && current_point.0 < grid.len() as f64 && current_point.1 >= 0.0 && current_point.1 < grid[1].len() as f64 {
            if current_point.0.fract() == 0.0 && current_point.1.fract() == 0.0 {
                points.insert((current_point.0 as isize, current_point.1 as isize));
            }
            current_point.0 -= v.1.0;
            current_point.1 -= v.1.1;

        }
    }
    println!("{vectors:?}");
    println!("{points:?}");
    println!("{}", points.len());
    points.iter().for_each(|pos| {
        grid[pos.0 as usize][pos.1 as usize] = '#';
    });
    let lines = grid.into_iter().map(|row| {

        String::from_iter(row.iter())
    }).collect::<Vec<String>>();




    fs::write("outputs/day8.txt", lines.join("\n")).unwrap();

}