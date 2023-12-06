use std::fs;

fn main() {
    let input_path = String::from("inputs/part2-input.txt");
    let file_contents = fs::read_to_string(input_path).expect("Unable to read file contents!");

    let string_digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let output = file_contents.lines().map(|line| {

        let numbers: Vec<u32> = line.chars().filter_map(|char| {



            match char.to_digit(10) {
                None => None,
                Some(num) => Some(num),
            }

        }).collect();
        
        let output_vec = vec![*numbers.first().unwrap(), *numbers.last().unwrap()];
        output_vec
    }).map(|line| {

        let first = line.first().unwrap();
        let last = line.last().unwrap();
wdw
        let output = format!("{first}{last}").parse::<u32>().unwrap();
        output
    });

    let output: u32  = output.sum();

    println!("{output}");

}
