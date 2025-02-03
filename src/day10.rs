use std::fs;
use std::time::Instant;

pub fn find_neighbors(grid: &Vec<Vec<u8>>, pos: (usize, usize), value: u8) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    if pos.0 > 0 {
        if grid[pos.0-1][pos.1] == value {
            results.push((pos.0-1, pos.1))
        }
    }
    if pos.0 < grid.len() - 1 {
        if grid[pos.0+1][pos.1] == value {
            results.push((pos.0+1, pos.1))
        }
    }

    if pos.1 > 0 {
        if grid[pos.0][pos.1-1] == value {
            results.push((pos.0, pos.1-1))
        }
    }

    if pos.1 < grid[0].len() - 1 {
        if grid[pos.0][pos.1+1] == value {
            results.push((pos.0, pos.1+1))
        }
    }

    results
}

pub fn part1 () {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day10.txt").unwrap();

    let grid: Vec<Vec<u8>> = contents.lines().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect()).collect();
    println!("{}", adventofcode_utils::grid_to_string(grid.clone()));
    let mut trailheads = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    println!("{trailheads:?}");

    let mut result = 0;
    for e in trailheads {
        let mut locations = vec![(e.0, e.1)];
        let mut score = 0;
        for i in 1..=9u8 {
            println!("{locations:?}");
            let mut new_locs = Vec::new();
            for location in locations.into_iter() {
                for element in find_neighbors(&grid, location, i) {
                    if !new_locs.contains(&element) {
                        new_locs.push(element);
                    }
                }
            }
            locations = new_locs;
        }
        result += locations.len();
    }






    println!("{result}")

}

pub fn part2 () {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day10.txt").unwrap();

    let grid: Vec<Vec<u8>> = contents.lines().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect()).collect();
    println!("{}", adventofcode_utils::grid_to_string(grid.clone()));
    let mut trailheads = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    println!("{trailheads:?}");

    let mut result = 0;
    for e in trailheads {
        let mut locations = vec![(e.0, e.1)];
        let mut score = 0;
        for i in 1..=9u8 {
            println!("{locations:?}");
            let mut new_locs = Vec::new();
            for location in locations.into_iter() {
                for element in find_neighbors(&grid, location, i) {
                    new_locs.push(element);
                }
            }
            locations = new_locs;
        }
        result += locations.len();
    }


    println!("{result}")
}
