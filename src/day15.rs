use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use adventofcode_utils::grid_to_string;

pub fn split_paragraphs(file_path: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(file_path).unwrap().split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>()

}
pub fn step(map: &mut Vec<Vec<char>>, instructions: &mut VecDeque<char>, mut pos: (usize, usize)) -> (usize, usize){

    let instruction= instructions.pop_front().unwrap();

    match instruction {
        '^' => {

        },
        'v' => {

        },
        '>' => {

        },
        '<' => {

        }
        _ => {
            panic!("Unknown instruction {}", instruction);
        }
    }

    pos

}

pub fn part1() {
    let paragraphs = split_paragraphs("inputs/day15.txt");
    let contents = paragraphs.iter().map(|s| s.lines().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    println!("{contents:?}");
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut instructions: VecDeque<char> = VecDeque::new();
    let mut robot_pos: (usize, usize) = (0, 0);

    for i in 0..contents[0].len() {
        map.push(Vec::new());
        contents[0][i].chars().enumerate().for_each(|(index, c)| {
            if c == '@' {
                robot_pos = (i, index);
            }
            map[i].push(c);

        });
    }

    for i in 0..contents[1].len() {
        contents[1][i].chars().for_each(|c| instructions.push_back(c));
    }
    println!("{}", grid_to_string(map));
    println!("{:?}", instructions);
    println!("{:?}", robot_pos);
}
pub fn part2() {
    let contents = fs::read_to_string("inputs/day15.txt").unwrap();
}