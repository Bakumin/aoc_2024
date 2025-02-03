use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use std::fs;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::{Duration, Instant};
use regex::Regex;
use itertools::Itertools;

#[derive(Copy, Clone)]
enum Ops {
    ADD,
    MULTIPLY,
    SUBTRACT,
    DIVIDE,
    CONCAT
}
impl Display for Ops {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self  {
            Ops::ADD => {
                f.write_str("ADD")
            }
            Ops::MULTIPLY => {
                f.write_str("MULTIPLY")
            }
            Ops::SUBTRACT => {
                f.write_str("SUBTRACT")
            }
            Ops::DIVIDE => {
                f.write_str("DIVIDE")
            }
            Ops::CONCAT => {
                f.write_str("CONCAT")

            }
        }
    }
}
impl Debug for Ops {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self  {
            Ops::ADD => {
                f.write_str("ADD")
            }
            Ops::MULTIPLY => {
                f.write_str("MULTIPLY")
            }
            Ops::SUBTRACT => {
                f.write_str("SUBTRACT")
            }
            Ops::DIVIDE => {
                f.write_str("DIVIDE")
            }
            Ops::CONCAT => {
                f.write_str("CONCAT")

            }

        }
    }
}

pub fn generate_permutations(in_vec: &Vec<Ops>, mut length: usize, mut out_vec: Vec<Vec<Ops>>) -> Vec<Vec<Ops>>{
    if length == 0 {
        return out_vec;
    };

    let mut n_o_v = vec![];
    for op in in_vec.iter() {
        let mut o = out_vec.clone();
        for mut ov in &mut o {
            ov.push(*op);
        }
        n_o_v.append(&mut o);
    }
    //println!("{out_vec:?}");
    length-=1;
    return generate_permutations(in_vec, length, n_o_v);

}
pub fn part1() {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day7.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;

    let vals = lines.map(|line| line.split_once(": ").unwrap()).map(|line| (usize::from_str(line.0).unwrap(), line.1.split(" ").map(|nbr| usize::from_str(nbr).unwrap_or(0)).collect::<Vec<usize>>())).collect::<Vec<(usize, Vec<usize>)>>();

    let used_ops = vec![Ops::ADD, Ops::MULTIPLY];


    println!("{vals:?}");

    for val in vals {
        let target = val.0;
        let permutations = generate_permutations(&used_ops, val.1.len(), vec![vec![Ops::ADD], vec![Ops::MULTIPLY]]);
        let mut found = false;
        println!("Target: {target}");
        println!("{:?}", permutations);
        //let possible_results = Vec::new();
        for var in permutations {
            let mut current_result = 0;
            for (op, v) in var.iter().zip(val.1.clone()) {
                println!("Op: {op}, value: {v}");
                match op {
                    &Ops::ADD => {
                        current_result+=v;
                    }
                    &Ops::MULTIPLY => {
                        current_result*=v;
                    },
                    _ => {
                        println!("something wrong");
                    }
                }
            }
            if current_result == target {
                found = true;
                break;
            }
        }
        if found {
            result+=val.0;
        }

    }



    println!("{result}");
    println!("{:?}", Instant::now().duration_since(timer));

}

pub fn part2() {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day7.txt").unwrap();
    let lines = contents.lines();
    let mut result = 0;

    let vals = lines.map(|line| line.split_once(": ").unwrap()).map(|line| (usize::from_str(line.0).unwrap(), line.1.split(" ").map(|nbr| usize::from_str(nbr).unwrap_or(0)).collect::<Vec<usize>>())).collect::<Vec<(usize, Vec<usize>)>>();

    let used_ops = vec![Ops::ADD, Ops::MULTIPLY, Ops::CONCAT];
    let mut permutation_cache: HashMap<usize, Vec<Vec<Ops>>> = HashMap::new();

    println!("{vals:?}");

    for mut val in vals {
        let target = val.0;
        let permutations;
        if !permutation_cache.contains_key(&val.1.len()) {
            permutations = generate_permutations(&used_ops, val.1.len()+1, vec![vec![Ops::ADD], vec![Ops::MULTIPLY], vec![Ops::CONCAT]]);
            permutation_cache.insert(val.1.len(), permutations.clone());
        }
        else {
            permutations = permutation_cache.get(&val.1.len()).unwrap().clone();

        }
        let mut found = false;
        println!("Target: {target}");
        //println!("{:?}", permutations.clone());
        //let possible_results = Vec::new();
        let starting_val = val.1.remove(0);
        for var in permutations {
            let mut current_result = starting_val;
            'inner: for (op, v) in var.iter().zip(val.1.clone()) {
                //println!("Op: {op}, value: {v}");
                match op {
                    &Ops::ADD => {
                        current_result+=v;
                    }
                    &Ops::MULTIPLY => {
                        current_result*=v;
                    },
                    &Ops::CONCAT => {
                        current_result = usize::from_str(&*format!("{current_result}{v}")).unwrap();
                    }
                    _ => {
                        println!("something wrong");
                    }
                }
                if current_result > target {
                    break 'inner;
                }
            }
            if current_result == target {
                println!("Target found: {}: {}", current_result, target);
                found = true;
                break;
            }
        }
        if found {
            result+=val.0;
        }

    }



    println!("{result}");
    println!("{:?}", Instant::now().duration_since(timer));

}
