use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum IncDec {
    INC,
    DEC
}

pub fn check_correctnes(l: i32, r: i32, inc: Option<IncDec>) -> (bool, IncDec) {

    return match inc {
        None => {
            let i;
            if r>l {
                i = IncDec::INC;
            }
            else {
                i = IncDec::DEC;
            }

            (l.abs_diff(r) < 4, i)
        }
        Some(IncDec::INC) => {
            (l < r && l > r+4, IncDec::INC)
        }
        Some(IncDec::DEC) => {
            (r < l && r > l+4, IncDec::DEC)
        }
    }


}

pub fn check_vec(l: &Vec<i32>) -> bool {
    let mut inc: Option<IncDec> = None;
    for i in 1..l.len() {
        if l[i-1] < l[i] && (inc.is_none() || inc.unwrap() == IncDec::INC) && (l[i-1] +4 > (l[i])) {

            inc = Some(IncDec::INC);
            continue;
        }
        else if l[i-1] > l[i] && (inc.is_none() || inc.unwrap() == IncDec::DEC) && l[i-1] < l[i] + 4 {
            inc = Some(IncDec::DEC);
            continue;
        }
        else {
            //println!("{} > {} +4(= {})", l[i-1], l[i], l[i] + 4);
            //println!("{:?}", l);
            return false
        }
    }
    true
}


pub fn part1() {
    let mut contents = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines = contents.lines();

    let good = lines.map(|line| line.split(" ").map(|entry| i32::from_str(entry).unwrap()).collect::<Vec<i32>>()).filter(|l| {
        check_vec(l)

    }).count();
    println!("{good}");

}

pub fn part2() {
    let mut contents = fs::read_to_string("inputs/day2.txt").unwrap();
    let lines = contents.lines();

    let good = lines.map(|line| line.split(" ").map(|entry| i32::from_str(entry).unwrap()).collect::<Vec<i32>>()).filter(|l| {
        let mut b = check_vec(l);
        if !b {
            for i in 0..l.len() {
                let mut n = l.clone();
                n.remove(i);
                if check_vec(&n) {
                    println!("Set b to True!");
                    b = true;
                }
            }
        }
        b
    }).count();
    println!("{good}");

}


/*
If a pair does not match:

the issue is i-1 IF i - i-2 works
the issue is i IF i+1 - i-1 works OR IF i is the last element

*/