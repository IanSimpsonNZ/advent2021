use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let mut prev = u32::MAX;
    let mut increased = 0;

    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(num_str) = line {
                if let Ok(num) = num_str.parse::<u32>() {

                    if num > prev {
                        increased += 1;
                    }

                    prev = num;

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

    println!("Depth increased {} times", increased);
}
