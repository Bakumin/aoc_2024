use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl Direction {
    pub fn rotate(self) -> Self{
        match self {
            Direction::NORTH => {
                Direction::EAST
            }
            Direction::EAST => {
                Direction::SOUTH
            }
            Direction::SOUTH => {
                Direction::WEST
            }
            Direction::WEST => {
                Direction::NORTH
            }
        }
    }
    pub fn turn_back(self) -> Self {
        match self {
            Direction::NORTH => {
                Direction::SOUTH
            }
            Direction::EAST => {
                Direction::WEST
            }
            Direction::SOUTH => {
                Direction::NORTH
            }
            Direction::WEST => {
                Direction::EAST
            }
        }
    }
}


pub fn part1() {

    let mut contents = fs::read_to_string("inputs/day6.txt").unwrap();
    let lines = contents.lines();
    let mut grid = lines.clone().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut starting_pos = (0, 0);
    let mut facing = Direction::NORTH;



    let mut i: usize = 0;
    for line in lines {
        match line.find("^") {
            None => {}
            Some(j) => {
                starting_pos = (i,j);
                break;
            }
        }
        i+=1;
    }
    println!("{:?}", starting_pos);
    println!("{:?}", grid);
    let mut simulation = || {
        let mut finished = false;
        let mut position = starting_pos;

        while !finished {
            match facing {
                Direction::NORTH => {
                    println!("North");
                    for x in (0..=position.0).rev() {
                        println!("{x}");
                        if grid[x][position.1] == '#' {
                            facing = Direction::EAST;
                            position = (x + 1,position.1);
                            println!("Turning East");
                            println!("{:?}", position);

                            break;
                        } else {
                            visited.insert((x, position.1));
                            if x == 0 {
                                finished = true;
                                println!("left the map");
                            }
                            continue;
                        }
                    }
                }
                Direction::EAST => {
                    for x in position.1..grid[position.0].len() {
                        if grid[position.0][x] == '#' {
                            facing = Direction::SOUTH;
                            position = (position.0,x - 1);
                            println!("Turning South");
                            println!("{:?}", position);

                            break;
                        } else {
                            visited.insert((position.0, x));
                            if x == grid[position.0].len()-1 {
                                finished = true;
                                println!("left the map");
                            }

                            continue;
                        }
                    }
                }
                Direction::SOUTH => {
                    for x in position.0..grid.len() {
                        if grid[x][position.1] == '#' {
                            facing = Direction::WEST;
                            position = (x - 1,position.1);
                            println!("Turning West");
                            println!("{:?}", position);


                            break;
                        } else {
                            visited.insert((x, position.1));
                            if x == grid.len()-1 {
                                finished = true;
                                println!("left the map");
                            }

                            continue;
                        }
                    }
                }
                Direction::WEST => {
                    for x in (0..=position.1).rev() {
                        if grid[position.0][x] == '#' {
                            facing = Direction::NORTH;
                            position = (position.0,x + 1);
                            println!("Turning North");
                            println!("{:?}", position);


                            break;
                        } else {
                            visited.insert((position.0, x));
                            if x == 0 {
                                finished = true;
                                println!("left the map");
                            }
                            continue;
                        }
                    }
                }
            }
        }

    };

    simulation();
    println!("{}", visited.len());



    visited.iter().for_each(|pos| {
        grid[pos.0][pos.1] = 'X';
    });
    let lines = grid.into_iter().map(|row| {

       String::from_iter(row.iter())
    }).collect::<Vec<String>>();




    fs::write("outputs/day6.txt", lines.join("\n")).unwrap();





}
#[derive(Debug, Eq)]
struct Visited(usize, usize, Direction);

impl Hash for Visited {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        (self.1 * 999).hash(state);
    }
}
impl PartialEq for Visited {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}

pub fn part2() {
    let timer = Instant::now();
    let mut contents = fs::read_to_string("inputs/day6.txt").unwrap();
    let lines = contents.lines();
    let mut grid = lines.clone().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut visited: HashSet<Visited> = HashSet::new();
    let mut starting_pos = (0, 0);
    let mut facing = Direction::NORTH;
    let mut turning_points: HashSet<(usize, usize, Direction)> = HashSet::new();



    let mut i: usize = 0;
    for line in lines {
        match line.find("^") {
            None => {}
            Some(j) => {
                starting_pos = (i,j);
                break;
            }
        }
        i+=1;
    }
    //println!("{:?}", starting_pos);
    //println!("{:?}", grid);
    let mut count = 0;
    let mut simulation = || {
        let mut finished = false;
        let mut position = starting_pos;

        while !finished {
            match facing {
                Direction::NORTH => {
                    println!("North");
                    for x in (0..=position.0).rev() {
                        //println!("{x}");
                        if grid[x][position.1] == '#' {
                            if turning_points.contains(&(x, position.1, facing)) {
                                count +=1;
                                finished = true;
                            }

                            turning_points.insert((x, position.1, facing));

                            facing = Direction::EAST;
                            position = (x + 1,position.1);

                            break;
                        } else {
                            visited.insert(Visited(x, position.1, facing));
                            if x == 0 {
                                finished = true;
                            }
                            continue;
                        }
                    }
                }
                Direction::EAST => {
                    for x in position.1..grid[position.0].len() {
                        if grid[position.0][x] == '#' {
                            if turning_points.contains(&(position.0, x, facing)) {
                                count +=1;
                                finished = true;
                            }

                            turning_points.insert((position.0, x,facing));

                            facing = Direction::SOUTH;

                            position = (position.0,x - 1);

                            break;
                        } else {
                            visited.insert(Visited(position.0, x, facing));
                            if x == grid[position.0].len()-1 {
                                finished = true;
                            }

                            continue;
                        }
                    }
                }
                Direction::SOUTH => {
                    for x in position.0..grid.len() {
                        if grid[x][position.1] == '#' {
                            if turning_points.contains(&(x, position.1, facing)) {
                                count +=1;
                                finished = true;
                            }

                            turning_points.insert((x, position.1 ,facing));

                            facing = Direction::WEST;
                            position = (x - 1,position.1);


                            break;
                        } else {
                            visited.insert(Visited(x, position.1, facing));
                            if x == grid.len()-1 {
                                finished = true;
                            }

                            continue;
                        }
                    }
                }
                Direction::WEST => {
                    for x in (0..=position.1).rev() {
                        if grid[position.0][x] == '#' {
                            if turning_points.contains(&(position.0, x, facing)) {
                                count +=1;
                                finished = true;
                            }
                            turning_points.insert((position.0, x,facing));

                            facing = Direction::NORTH;
                            position = (position.0,x + 1);


                            break;
                        } else {
                            visited.insert(Visited(position.0, x, facing));
                            if x == 0 {
                                finished = true;
                            }
                            continue;
                        }
                    }
                }
            }
        }

    };

    simulation();
    fn search(row: usize, column: usize, grid: &Vec<Vec<char>>, direction: &Direction) -> Option<(usize, usize, Direction)>{
        return match direction {
            Direction::NORTH => {
                grid.iter().enumerate().rev().find(|pos| pos.1[column+1] == '#' && pos.0 < row).and_then(|f| Some((f.0, column+1, *direction)))
            }
            Direction::EAST => {
                grid[row+1].iter().enumerate().find(|pos| pos.1 == &'#' && pos.0 > column).and_then(|f| Some((row+1, f.0, *direction)))

            }
            Direction::SOUTH => {
                grid.iter().enumerate().find(|pos| pos.1[column-1] == '#' && pos.0 > row).and_then(|f| Some((f.0, column-1, *direction)))
            }
            Direction::WEST => {
                grid[row-1].iter().enumerate().rev().find(|pos| pos.1 == &'#' && pos.0 < column).and_then(|f| Some((row-1, f.0, *direction)))
            }
        };



    }


    let mut find_stuff = || {
        /*THE PLANT:
            - Iterate through all visited Locations
            - Check if the pattern for them holds true
         */

        let mut block_positions = HashSet::new();
        //println!("{visited:?}");
        for pos in visited.iter() {

            let first_block = (pos.0, pos.1, pos.2);
            grid[pos.0][pos.1] = '#';
            //println!("First Block: {first_block:?}");
            let mut blocks = HashSet::new();
            blocks.insert(first_block);
            let mut current_block = first_block;
            'inner: loop {
                let res = search(current_block.0, current_block.1, &grid, &current_block.2.rotate());
                //println!("{res:?}");
                if res.is_none() {
                    break 'inner;
                }
                else if blocks.contains(&res.unwrap()) {
                    //println!("Found a thing");
                    if block_positions.contains(&(first_block.0, first_block.1)) {
                        //println!("But it was already in there");
                    }else {
                        block_positions.insert((first_block.0, first_block.1));
                        count+=1;

                    }
                    break 'inner;
                }
                current_block = res.unwrap();
                blocks.insert(current_block);
            }

            grid[pos.0][pos.1] = '.';




        }
        //println!("{block_positions:?}");

    };


    find_stuff();


    println!("{count}");
    println!("{:?}", Instant::now().duration_since(timer));
}