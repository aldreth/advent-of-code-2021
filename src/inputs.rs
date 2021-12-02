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
