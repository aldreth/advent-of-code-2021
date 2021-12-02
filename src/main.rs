use std::fs;

fn count_increases(measurements: Vec<u32>) -> u32 {
    let mut increases: u32 = 0;
    let mut current = u32::MAX;
    for measurement in measurements {
        if measurement > current {
            increases += 1;
        }
        current = measurement;
    }
    increases
}

fn read_input_from_file(filename: String) -> Vec<u32> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.parse::<u32>().expect("Not a valid u32"))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn finds_seven_inputs_for_the_test_file() {
        let inputs = super::read_input_from_file("./inputs/test.txt".to_string());
        let count = super::count_increases(inputs.to_vec());
        assert_eq!(7, count);
    }
}

fn main() {
    let inputs = read_input_from_file("./inputs/input.txt".to_string());
    let count = count_increases(inputs.to_vec());
    println!("There have been {} increases", count);
}
