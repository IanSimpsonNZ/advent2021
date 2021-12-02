use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let mut horiz = 0;
    let mut depth = 0;


    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(command_str) = line {

                let mut split = command_str.split_whitespace();
                let command = split.next().unwrap();
                let num_str = split.next().unwrap();

                if let Ok(num) = num_str.parse::<u32>() {

                    match command.chars().next().unwrap() {
                        'f' => {
                            horiz += num;
                        },
                        'd' => {
                            depth += num;
                        },
                        'u' => {
                            depth -= num;
                        },
                        _ => {
                            println!("Invaid command: {}", command);
                        }
                    }


                } else {
                    println!("Couldn't parse '{}'", num_str);
                }

            } else {
                println!("Can't read line");
            }

        }

    } else {
        println!("Can't open input.txt");
    }

    println!("Horiz = {}; Depth = {}; Answer = {}", horiz, depth, horiz * depth);
}
