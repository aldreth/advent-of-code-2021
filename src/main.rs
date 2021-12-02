mod day01;

fn output_day1() {
    let inputs = day01::read_input_from_file("./inputs/input.txt".to_string());
    // part 1
    let mut count = day01::count_increases(inputs.to_vec());
    println!("There have been {} increases", count);
    // part 2
    count = day01::count_increases_in_blocks_of_three(inputs.to_vec());
    println!("There have been {} increases by block of three", count);
}

fn main() {
    output_day1();
}
