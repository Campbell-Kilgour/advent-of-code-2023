use std::fs;

fn main() {
    let input_path = String::from("inputs/part1-input.txt");
    let file_contents = fs::read_to_string(input_path).expect("Unable to read file contents!");
    
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let games: Vec<bool> = file_contents.lines()
    .map(|line| {
        let segments: Vec<&str> = line.split(':').collect(); // Removed first 8 character of the line which contains the game id
        let line =segments[1];

        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let possible: Vec<bool> = line.split([',', ';'])
        .map(|colour| {
            let draw_split: Vec<&str> = colour.trim().split(' ').collect();
            let size_ref = draw_split[0];
            println!("{size_ref}");
            let size = draw_split[0].parse::<u32>().unwrap();
            let colour = draw_split[1];
            let flag: bool = match draw_split[1].trim() {
                "red" => {
                    if size > max_red {
                        false
                    } else {
                        true
                    }
                },
                "green" => {
                    if size > max_green {
                        false
                    } else {
                        true
                    }
                }
                "blue" => {
                    if size > max_blue {
                        false
                    } else {
                        true
                    }
                },
                _ => panic!("Unrecognised colour!")
            };

            flag

        }).collect();

        let flag = possible.iter().all(|flag| flag == &true );
        flag
    }).collect(); 

    let mut index = 1;
    let mut total = 0;

    for game in games {
        println!("{game}");
        if game {
            total = total + index;
        }
        index = index + 1;
    }

    println!("{total}");
}
