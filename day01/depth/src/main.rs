use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let mut increased = 0;

    let mut win1:Vec<u32> = Vec::new();
    let mut win2:Vec<u32> = Vec::new();

    let mut line_count = 0;


    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(num_str) = line {
                if let Ok(num) = num_str.parse::<u32>() {

                    line_count += 1;

                    if line_count == 1 {
                        win1.push(num);
                        continue;
                    } else if line_count < 4 {
                        win1.push(num);
                        win2.push(num);
                        continue;
                    }


                    win2.push(num);

                    if win2.iter().sum::<u32>() > win1.iter().sum::<u32>() {
                        increased += 1;
                    }

                    win1 = win2.clone();

                    win2.remove(0);

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
