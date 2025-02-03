use std::fs;
use regex::Regex;

pub fn part1() {
    let mut contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;

    let c = lines.map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    for i in 0..c.len() {
        for j in 0..c[i].len() {
            if c[i][j] == 'X' {
                //Horizontal Forward
                if i+3 < c.len() {
                    if (c[i+1][j] == 'M' && c[i+2][j] == 'A' && c[i+3][j] == 'S') {
                        result +=1;
                    }
                }
                //Horizontal Backward
                if i > 2 {
                    if (c[i-1][j] == 'M' && c[i-2][j] == 'A' && c[i-3][j] == 'S') {
                        result +=1;
                    }
                }
                //Vertical Forward
                if j+3 < c[i].len() {
                    if (c[i][j+1] == 'M' && c[i][j+2] == 'A' && c[i][j+3] == 'S') {
                        result +=1;
                    }
                }
                //Vertical Backward
                if j > 2 {
                    if (c[i][j-1] == 'M' && c[i][j-2] == 'A' && c[i][j-3] == 'S') {
                        result +=1;
                    }
                }
                //Diagonal topleft
                if j > 2 && i > 2  {
                    if (c[i-1][j-1] == 'M' && c[i-2][j-2] == 'A' && c[i-3][j-3] == 'S') {
                        result +=1;
                    }
                }
                //Diagonal botleft
                if j+3 < c[i].len() && i > 2  {
                    if (c[i-1][j+1] == 'M' && c[i-2][j+2] == 'A' && c[i-3][j+3] == 'S') {
                        result +=1;
                    }
                }

                //Diagonal topright
                if j > 2 && i+3 < c.len()  {
                    if (c[i+1][j-1] == 'M' && c[i+2][j-2] == 'A' && c[i+3][j-3] == 'S') {
                        result +=1;
                    }
                }
                //Diagonal botright
                if j+3 < c[i].len() && i+3 < c.len()  {
                    if (c[i+1][j+1] == 'M' && c[i+2][j+2] == 'A' && c[i+3][j+3] == 'S') {
                        result +=1;
                    }
                }



            }
        }
    }


    println!("{result}");
}

pub fn part2() {

    let mut contents = fs::read_to_string("inputs/day4.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;
    let c = lines.map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    for i in 1..c.len()-1 {
        for j in 1..c[i].len()-1 {
            if c[i][j] == 'A' {
                if ((c[i-1][j-1] == 'M' && c[i+1][j+1] == 'S') || (c[i-1][j-1] == 'S' && c[i+1][j+1] == 'M'))&&((c[i-1][j+1] == 'M' && c[i+1][j-1] == 'S') || (c[i-1][j+1] == 'S' && c[i+1][j-1] == 'M')) {
                    result+=1;
                }
            }
        }
    }
    println!("{result}");
}