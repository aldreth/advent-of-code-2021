mod inputs;

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

#[cfg(test)]
#[test]
fn test_count_increases() {
    let count = count_increases([199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec());
    assert_eq!(7, count);
}

fn main() {
    let inputs = inputs::read_input_from_file("./inputs/input.txt".to_string());
    let count = count_increases(inputs.to_vec());
    println!("There have been {} increases", count);
}
