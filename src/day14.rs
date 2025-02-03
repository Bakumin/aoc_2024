use std::fmt::{Display, Formatter};
use std::fs;
use std::io::stdin;
use regex::Regex;

pub fn part1() {
    let contents = fs::read_to_string("inputs/day14.txt").unwrap();
    let re_post = Regex::new(r"p=(?<x>-?\d+),(?<y>-?\d+)").unwrap();
    let re_velo = Regex::new(r"v=(?<x>-?\d+),(?<y>-?\d+)").unwrap();

    let mut pos: Vec<(usize, usize)> = Vec::new();

    let mut scores = (0, 0, 0, 0);
    let wide = 101;
    let tall = 103;

    for robot in re_post.captures_iter(&contents).zip(re_velo.captures_iter(&contents)) {
        let pos_x = robot.0.name("x").unwrap().as_str().parse::<isize>().unwrap();
        let pos_y = robot.0.name("y").unwrap().as_str().parse::<isize>().unwrap();
        let vec_x = robot.1.name("x").unwrap().as_str().parse::<isize>().unwrap();
        let vec_y = robot.1.name("y").unwrap().as_str().parse::<isize>().unwrap();
        //let fin_pos = ((((pos_x+ (100*vec_x))).rem_euclid(11) as usize, ((pos_y+(100*vec_y))).rem_euclid(7) as usize));

        let fin_pos = ((((pos_x+ (100*vec_x))).rem_euclid(wide) as usize, ((pos_y+(100*vec_y))).rem_euclid(tall) as usize));
        if fin_pos.0 != (wide / 2) as usize && fin_pos.1 != (tall / 2) as usize {
            match (fin_pos.0 < (wide/2) as usize, fin_pos.1 < (tall/2) as usize) {
                (true, true) => {
                    scores.0+=1;
                }
                (false, true) => {
                    scores.1+=1;
                }
                (true, false) => {
                    scores.2+=1;
                }
                (false, false) => {
                    scores.3+=1;
                }
            }
        }

        pos.push(fin_pos);
    }
    println!("{pos:?}");
    println!("{scores:?}");
    println!("Result: {}", scores.0 * scores.1 * scores.2 * scores.3);

}

struct Robot {
    pos: (isize, isize),
    vec: (isize, isize),
    width: isize,
    height: isize
}
impl Robot {
    pub fn step(&mut self) -> (isize, isize){
        self.pos = ((self.pos.0 + self.vec.0).rem_euclid(self.width), (self.pos.1 + (self.vec.1)).rem_euclid(self.height));
        self.pos
    }
}

pub fn part2() {
    let contents = fs::read_to_string("inputs/day14.txt").unwrap();
    let re_post = Regex::new(r"p=(?<x>-?\d+),(?<y>-?\d+)").unwrap();
    let re_velo = Regex::new(r"v=(?<x>-?\d+),(?<y>-?\d+)").unwrap();

    let mut pos: Vec<(usize, usize)> = Vec::new();

    let mut score = 0;
    let wide = 101;
    let tall = 103;
    let mut robots = Vec::new();
    
    for robot in re_post.captures_iter(&contents).zip(re_velo.captures_iter(&contents)) {
        let pos_x = robot.0.name("x").unwrap().as_str().parse::<isize>().unwrap();
        let pos_y = robot.0.name("y").unwrap().as_str().parse::<isize>().unwrap();
        let vec_x = robot.1.name("x").unwrap().as_str().parse::<isize>().unwrap();
        let vec_y = robot.1.name("y").unwrap().as_str().parse::<isize>().unwrap();

        robots.push(Robot {
            pos: (pos_x, pos_y),
            vec: (vec_x, vec_y),
            width: wide,
            height: tall,
        });
    }

    loop {
        let mut grid = vec![vec![' '; wide as usize]; tall as usize];

        for robot in &mut robots {
            let pos = robot.step();
            grid[pos.1 as usize][pos.0 as usize] = '#';
        }
        score+=1;
        println!("{score}:\n{}", adventofcode_utils::grid_to_string(grid));
        stdin().read_line(&mut String::new()).unwrap();
    }
}