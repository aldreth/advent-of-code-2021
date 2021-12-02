mod inputs;

#[allow(dead_code)]
fn count_increases(measurements: Vec<u32>) -> u32 {
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

fn count_increases_in_blocks_of_three(measurements: Vec<u32>) -> u32 {
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

fn main() {
    let inputs = inputs::read_input_from_file("./inputs/input.txt".to_string());
    let count = count_increases_in_blocks_of_three(inputs.to_vec());
    println!("There have been {} increases", count);
}
