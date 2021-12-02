use std::fs;

pub fn read_input_from_file(filename: String) -> Vec<u32> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<u32>().expect("Not a valid u32"))
        .collect()
}

#[cfg(test)]
#[test]
fn reads_file_correctly() {
    let inputs = read_input_from_file("./inputs/test.txt".to_string());
    assert_eq!(
        inputs,
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    );
}

pub fn count_increases(measurements: Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    let mut current = u32::MAX;
    for measurement in measurements {
        if measurement > current {
            count += 1;
        }
        current = measurement;
    }
    count
}

#[cfg(test)]
#[test]
fn test_count_increases() {
    let count = count_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    assert_eq!(7, count);
}

pub fn count_increases_in_blocks_of_three(measurements: Vec<u32>) -> u32 {
    let mut iter = measurements.iter();

    let mut first = iter.next().expect("not a number");
    let mut second = iter.next().expect("not a number");
    let mut third = iter.next().expect("not a number");

    let mut current = first + second + third;
    let mut count: u32 = 0;

    for measurement in iter {
        first = second;
        second = third;
        third = measurement;

        let total = first + second + third;
        if total > current {
            count += 1;
        }
        current = total
    }
    count
}

#[cfg(test)]
#[test]
fn test_count_increases_in_blocks_of_three() {
    let count =
        count_increases_in_blocks_of_three(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    assert_eq!(5, count);
}
