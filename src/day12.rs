use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Above,
    Below
}
impl Direction {
    pub fn rotate(&self) -> Direction {
        match self {
            Direction::Left => {
                return Direction::Above
            }
            Direction::Right => {
                return Direction::Below
            }
            Direction::Above => {
                return Direction::Right
            }
            Direction::Below => {
                return Direction::Left
            }
        }
    }
    pub fn rotate_rev(&self) -> Direction {
        match self {
            Direction::Left => {
                return Direction::Below
            }
            Direction::Right => {
                return Direction::Above
            }
            Direction::Above => {
                return Direction::Left
            }
            Direction::Below => {
                return Direction::Right
            }
        }
    }
    pub fn mov(&self, coords: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Left => {
                return (coords.0, coords.1-1)
            }
            Direction::Right => {
                return (coords.0, coords.1+1)
            }
            Direction::Above => {
                return (coords.0-1, coords.1)
            }
            Direction::Below => {
                return (coords.0+1, coords.1)
            }
        }
    }
    pub fn mov_until_hit(&self, grid: &Vec<Vec<char>>, value: &char, coords: (usize, usize), rev: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let mut coords = coords;
        while check_side(grid, value, &coords, *self) == Content::Match {
            coords = self.mov(coords);
            if rev {
                if check_side(grid, value, &coords, self.rotate()) == Content::Match {
                    return result;
                }
            }
            else {
                if check_side(grid, value, &coords, self.rotate_rev()) == Content::Match {
                    return result;
                }
            }
            result.push(coords);
        }



        result
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Content {
    Match,
    Mismatch,
    OutOfBounds
}

pub fn check_side(grid:&Vec<Vec<char>>, value: &char, coords: &(usize, usize), side: Direction) -> Content {
    match side {
        Direction::Above => {
            if coords.0 > 0 {
                if grid[coords.0 - 1][coords.1] == *value {
                    return Content::Match;
                }
                return Content::Mismatch;
            };
            return Content::OutOfBounds;
        }
        Direction::Below => {
            if coords.0 < grid.len() -1 {
                if grid[coords.0+1][coords.1] == *value{
                    return Content::Match;
                }
                return Content::Mismatch;
            }
            return Content::OutOfBounds;
        }
        Direction::Left => {
            if coords.1 > 0 {
                if grid[coords.0][coords.1-1] == *value {
                    return Content::Match;
                }
                return Content::Mismatch;
            }
            return Content::OutOfBounds;
        }
        Direction::Right => {
            if coords.1 < grid[coords.0].len() - 1 {
                if grid[coords.0][coords.1+1] == *value{
                    return Content::Match;
                }
                return Content::Mismatch;
            }
            return Content::OutOfBounds;
        }
    }
}

pub fn check_sides(grid: &Vec<Vec<char>>, value: &char, coords: &(usize, usize)) -> Vec<(Direction, Content, usize, usize)>{
    let mut found = Vec::new();

    let left =  check_side(grid, value, coords, Direction::Left);
    if left != Content::OutOfBounds {
        found.push((Direction::Left, left, coords.0, coords.1-1));
    }
    else {
        found.push((Direction::Left, left, 0, 0));
    }
    let right =  check_side(grid, value, coords, Direction::Right);
    if right != Content::OutOfBounds {
        found.push((Direction::Right, right, coords.0, coords.1+1));
    }
    else {
        found.push((Direction::Right, right, 0, 0));
    }

    let above = check_side(grid, value, coords, Direction::Above);
    if above != Content::OutOfBounds {
        found.push((Direction::Above, above, coords.0-1, coords.1));
    }
    else {
        found.push((Direction::Above, above, 0, 0));
    }

    let below =  check_side(grid, value, coords, Direction::Below);
    if below != Content::OutOfBounds {
        found.push((Direction::Below, below, coords.0+1, coords.1));
    }
    else {
        found.push((Direction::Below, below, 0, 0));
    }
    found
}

//950583
pub fn stuff(grid: &Vec<Vec<char>>, seen: &mut HashSet<(usize, usize)>, value: char, coords: (usize, usize), results: (usize, usize)) -> (usize, usize) {
    let mut poles = results.0;
    let mut fields = results.1 +1;

    let neighbors = check_sides(grid, &value, &coords);
    for n in neighbors.iter() {
        if n.1 == Content::Match {
            if !seen.contains(&(n.2, n.3)) {
                seen.insert((n.2, n.3));
                let (temp_poles, temp_fields) = stuff(grid, seen, value, (n.2, n.3), results);
                poles += temp_poles;
                fields += temp_fields;
            }
        } else{
            poles += 1;
        }
    }
    (poles, fields)
}

pub fn stuff_p2(grid: &Vec<Vec<char>>, seen: &mut HashSet<(usize, usize)>, fences: &mut HashSet<((usize,usize), Direction)>, value: char, coords: (usize, usize), results: (usize, usize)) -> (usize, usize) {
    let mut poles = results.0;
    let mut fields = results.1 +1;

    let neighbors = check_sides(grid, &value, &coords);
    for n in neighbors.iter() {
        if n.1 == Content::Match {
            if !seen.contains(&(n.2, n.3)) {
                seen.insert((n.2, n.3));
                let (temp_poles, temp_fields) = stuff_p2(grid, seen, fences, value, (n.2, n.3), results);
                poles += temp_poles;
                fields += temp_fields;
            }
        } else{
            if value == 'F' && coords == (3, 7) {
                println!("{fences:?}");
            }
            if !(check_side(grid, &value, &coords, n.0.rotate()) == Content::Match && n.0.rotate().mov_until_hit(grid, &value, (coords.0, coords.1), false).iter().any(|c| fences.contains(&((c.0, c.1), n.0)))) &&
                !(check_side(grid, &value, &coords, n.0.rotate_rev()) == Content::Match && n.0.rotate_rev().mov_until_hit(grid, &value, (coords.0, coords.1), true).iter().any(|c| fences.contains(&((c.0, c.1), n.0))))
            {
                poles += 1;
            }
            else {
                println!("{fences:?}");
            }
            fences.insert(((coords.0, coords.1), n.0));

        }
    }
    (poles, fields)}
pub fn part1(){
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day12.txt").unwrap();
    let grid: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut seen = HashSet::new();
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !seen.contains(&(i, j)) {
                seen.insert((i, j));
                let (poles, fields) = stuff(&grid, &mut seen, grid[i][j], (i,j), (0, 0));
                println!("{}: Fields {:?}, Poles {:?}, result: {}", grid[i][j], fields, poles, poles*fields);
                result += poles*fields;
            }
        }
    }
    println!("{}", result);
}

pub fn part2(){
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day12.txt").unwrap();
    let grid: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut seen = HashSet::new();
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !seen.contains(&(i, j)) {
                seen.insert((i, j));
                let mut fences = HashSet::new();
                let (poles, fields) = stuff_p2(&grid, &mut seen, &mut fences, grid[i][j], (i,j), (0, 0));
                println!("{}: Fields {:?}, Poles {:?}, result: {}", grid[i][j], fields, poles, poles*fields);
                result += poles*fields;
            }
        }
    }
    println!("{}", result);
}