use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::iter::successors;
use std::time::Instant;

pub fn step(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        }
        else{
            let digits = successors(Some(stone), |stone| (stone >= &10).then(|| stone/10)).count() as u32;
            if digits % 2 == 0 {
                new_stones.push(stone/(10_usize.pow(digits/2)));
                new_stones.push(stone % (10_usize.pow(digits/2)));
            }
            else {
                new_stones.push(stone * 2024);
            }
        }
    };

    new_stones
}
pub fn step_rec(stone: usize, steps: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if cache.contains_key(&(stone, steps)) {
        return *cache.get(&(stone, steps)).unwrap();
    }
    if steps == 0 {
        return 1;
    }
    if stone == 0 {
        let r = step_rec(1, steps-1, cache);
        cache.insert((stone, steps), r);
        return r;
    }
    else{
        let digits = successors(Some(stone), |stone| (stone >= &10).then(|| stone/10)).count() as u32;
        if digits % 2 == 0 {
            let r = step_rec(stone/(10_usize.pow(digits/2)), steps -1, cache) + step_rec(stone % (10_usize.pow(digits/2)), steps-1, cache);
            cache.insert((stone, steps), r);
            return r;
        }
        else {
            let r = step_rec(stone * 2024, steps - 1, cache);
            cache.insert((stone, steps), r);
            return r;
        }
    }
}

pub fn part1() {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day11.txt").unwrap();
    let mut stones = contents.split(' ').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    for _ in 0..25 {
        stones = step(stones);
    }

    println!("{:?}", stones.len());
    println!("Total time elapsed: {:?}", timer.elapsed());

}

pub fn part2() {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day11.txt").unwrap();
    let mut stones = contents.split(' ').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut stone_count = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        stone_count += step_rec(stone, 75, &mut cache);
        //println!("{stone_count}");
    }
    println!("{stone_count}");
    println!("Total time elapsed: {:?}", timer.elapsed());
}