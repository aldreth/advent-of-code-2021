mod day01;
mod day02;

#[allow(dead_code)]
fn output_day1() {
    let inputs = day01::read_input_from_file("./inputs/input.txt".to_string());
    // part 1
    let mut count = day01::count_increases(inputs.to_vec());
    println!("There have been {} increases", count);
    // part 2
    count = day01::count_increases_in_blocks_of_three(inputs.to_vec());
    println!("There have been {} increases by block of three", count);
}

fn output_day2() {
    let inputs = day02::read_input_from_file("./inputs/day2input.txt");
    // part 1
    let position_and_result = day02::get_position_and_result(inputs);
    println!("x: {:?}", position_and_result);
    // part 2
    // count = day01::count_increases_in_blocks_of_three(inputs.to_vec());
    // println!("There have been {} increases by block of three", count);
}

fn main() {
    output_day2();
}
