use std::collections::{HashMap, HashSet};
use std::{fs, usize};
use std::cmp::Ordering;
use std::ops::Sub;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;

pub fn part1 () {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day9.txt").unwrap().chars().map(|b| b.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    let mut result = 0;
    let example = "2333133121414131402";
    //contents = example.chars().map(|b| b.to_digit(10).unwrap() as u8).collect();
    let mut line = Vec::new();
    println!("{:?}", Instant::now().duration_since(timer));

    for x in contents.iter().enumerate() {
        if x.0 %2 == 0 {
            for i in 0..*x.1 {
                line.push(x.0/2);
            }
        }
        else {
            for i in 0..*x.1 {
                line.push(usize::MAX);
            }
        }
    }
    println!("{:?}", Instant::now().duration_since(timer));

    for mut x in (0..line.len()).rev() {
        if line[x] < usize::MAX {
            let index = line.iter().find_position(|p| p == &&usize::MAX).unwrap().0.clone();
            if x > index {
                line.swap(x, index);
            }
        }
    }
    println!("{:?}", Instant::now().duration_since(timer));

    result = line.iter().enumerate().filter(|(pos, value)| value < &&usize::MAX).map(|(pos, value)| pos*value).sum();



    //println!("{line:?}");
    println!("{result}");
    println!("{:?}", Instant::now().duration_since(timer));

}

pub fn part2() {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day9.txt").unwrap().chars().map(|b| b.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    let mut result = 0;
    let example = "2333133121414131402";
    //contents = example.chars().map(|b| b.to_digit(10).unwrap() as u8).collect();
    let mut line = Vec::new();
    let mut file_table = Vec::new();
    let mut pos = 0;
    let mut highest_id = 0;
    for x in contents.iter().enumerate() {
        if x.0 %2 == 0 {
            for i in 0..*x.1 {
                line.push(x.0/2);
            }

            file_table.push((x.0/2, pos, *x.1));
            pos+=*x.1 as usize;
            highest_id = x.0/2;
        }
        else {
            for i in 0..*x.1 {
                line.push(usize::MAX);
            }
            file_table.push((usize::MAX, pos, *x.1));
            pos+=*x.1 as usize;
        }
    }

    fn swaps (v: (usize, usize, u8), l: (usize, usize, u8)) -> Vec<(usize, usize, u8)> {
        let mut r_v = Vec::new();
        let mut new_v = (v.0, l.1, v.2);
        r_v.push(new_v);
        match l.2.cmp(&v.2) {
            Ordering::Equal => {
                let new_l = (l.0, v.1, v.2);
                r_v.push(new_l);
            }
            Ordering::Greater => {
                let new_l = (l.0, v.1, v.2);
                let new_new_l = (l.0, l.1 + v.2 as usize, l.2 - v.2);
                r_v.push(new_l);
                r_v.push(new_new_l);
            }
            _ =>  {}
        };

        return r_v;


    };

    for mut x in (1..=highest_id).rev() {
        let mut v = file_table.iter().rev().find_position(|p| p.0 == x).map(|x| (x.0, x.1.to_owned().clone())).clone();
        let mut l = file_table.iter().find_position(|p| p.0 == usize::MAX && p.2 >= v.unwrap_or((0, (0,0, u8::MAX))).1.2).map(|x| (x.0, x.1.to_owned().clone())).clone();
        match (l, v) {
            (None, _) | (_, None) => {
                continue;
            }
            (Some(mut l), Some(mut v)) => {
                v.0 = file_table.len() - v.0-1;
                let l = l.clone().to_owned();
                //println!("{:?}", file_table);
                if v.0 < l.0 {
                    continue;
                }
                file_table.remove(l.0);
                file_table.remove(v.0 -1);
                for x in swaps(v.1, l.1) {
                    file_table.insert(file_table.len(), x);
                }
                //*f.1 = v.1;
            }
        }

        file_table.sort_by(|v, v2| v.1.cmp(&v2.1));




    }


    for x in 0..file_table.len() {
        if file_table[x].0 != usize::MAX {
            for i in 0..file_table[x].2 {
                result+= file_table[x].0 * (file_table[x].1 +i as usize);
            }
        }

    }

    /*for mut x in (0..line.len()).rev() {
        if line[x] < usize::MAX {
            let index = line.iter().find_position(|p| p == &&usize::MAX).unwrap().0.clone();
            if x > index {
                line.swap(x, index);
            }
        }
    }

    result = line.iter().enumerate().filter(|(pos, value)| value < &&usize::MAX).map(|(pos, value)| pos*value).sum();
*/

    /*
    [file_id, pos, len]
    ..
    [file_id, pos, len]

    for in file_id :
        find file_id = MAX & len < file_id
            if len same:
                swap pos
            else:
                swap pos, copy file_id=MAX, increment pos, decrement len
     */
    println!("{line:?}");
    println!("{file_table:?}");
    println!("{result}");
    println!("{:?}", Instant::now().duration_since(timer));

}