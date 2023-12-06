use std::fs;

fn main() {
    let input_path = String::from("inputs/part2-input.txt");
    let file_contents = fs::read_to_string(input_path).expect("Unable to read file contents!");
    
    let games: u32 = file_contents.lines()
    .map(|line| { //iterates on every line in the file
        let segments: Vec<&str> = line.split(':').collect(); // Removed first 8 character of the line which contains the game id
        let line =segments[1];


        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let draws: Vec<&str> = line.split([',', ';']).collect();

        for draw in draws {
            let draw_split: Vec<&str> = draw.trim().split(' ').collect();
            let size = draw_split[0].parse::<u32>().unwrap();
            let colour = draw_split[1];

            match colour {
                "red" => {
                    if size > min_red {
                        min_red = size;
                    }
                },
                "green" => {
                    if size > min_green {
                        min_green = size;
                    }
                },
                "blue" => {
                    if size > min_blue {
                        min_blue = size;
                    }
                },
                _ => panic!("Unrecognised colour!") 
            }
        }

        let product = min_red * min_green * min_blue;

        product
    }).sum(); 

    println!("{games}");
}
