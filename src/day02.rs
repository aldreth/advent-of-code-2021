use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub enum Directions {
    Forward,
    Up,
    Down,
}

fn match_direction(word: &str) -> Directions {
    match word {
        "forward" => Directions::Forward,
        "up" => Directions::Up,
        "down" => Directions::Down,
        _ => panic!("Doesn't match a direction"),
    }
}

#[test]
fn test_match_direction() {
    assert_eq!((Directions::Forward), match_direction("forward"));
    assert_eq!((Directions::Up), match_direction("up"));
    assert_eq!((Directions::Down), match_direction("down"));
}

#[test]
#[should_panic]
fn test_match_direction_panics() {
    assert_eq!((Directions::Down), match_direction("other"));
}

fn parse_line(line: &str) -> (Directions, u32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Expecting two parts");
    }
    let direction = match_direction(parts[0]);
    let distance = parts[1].parse::<u32>().expect("Not a valid u32");
    (direction, distance)
}

#[test]
fn test_parse_line() {
    let distance: u32 = 15;
    assert_eq!((Directions::Forward, distance), parse_line("forward 15"));
}

pub fn read_input_from_file(filename: &str) -> Vec<(Directions, u32)> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| parse_line(line))
        .collect()
}

pub fn get_position_and_result(inputs: Vec<(Directions, u32)>) -> (u32, u32, u32) {
    let mut x = 0;
    let mut y = 0;
    for (direction, distance) in inputs {
        match direction {
            Directions::Forward => x += distance,
            Directions::Down => y += distance,
            Directions::Up => y -= distance,
        }
    }
    (x, y, x * y)
}

#[test]
fn test_get_position_and_result() {
    let inputs = read_input_from_file("./inputs/day2test.txt");
    let position_and_result = get_position_and_result(inputs);
    assert_eq!((15, 10, 150), position_and_result);
}

pub fn get_position_and_result_part_2(inputs: Vec<(Directions, u32)>) -> (u32, u32, u32) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (direction, distance) in inputs {
        match direction {
            Directions::Forward => {
                x += distance;
                y += aim * distance;
            }
            Directions::Down => aim += distance,
            Directions::Up => aim -= distance,
        }
    }
    (x, y, x * y)
}

#[test]
fn test_get_position_and_result_part_2() {
    let inputs = read_input_from_file("./inputs/day2test.txt");
    let position_and_result = get_position_and_result_part_2(inputs);
    assert_eq!((15, 60, 900), position_and_result);
}
