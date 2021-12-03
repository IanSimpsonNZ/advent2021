use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn count_ones(numbers: &Vec<String>, index: usize) -> usize {
    let mut num_ones = 0;

    for num_str in numbers {
            let bit = num_str.chars().nth(index).unwrap();
            if bit == '1' {
                num_ones += 1;
                    }
    }

    num_ones
}

fn filter_codes(codes: &Vec<String>, index: usize, most_sig: bool) -> String {

    if codes.len() > 1 {
        let mut filtered_list: Vec<String> = Vec::new();
        let mut most_common_bit = '0';
        let num_ones = count_ones(codes, index);
        let num_zeros = codes.len() - num_ones;
        if num_ones >= num_zeros {
            most_common_bit = '1';
        }

        let search_bit = if most_sig {
                            most_common_bit
                         } else {
                            if most_common_bit == '1' {
                                '0'
                            } else {
                                '1'
                            }
        };

        for code in codes {
            if code.chars().nth(index).unwrap() == search_bit {
                filtered_list.push(code.to_string());
            }
        }

        filter_codes(&filtered_list, index + 1, most_sig)

    } else {

        codes[0].to_string()

    }
}


fn main() {
    let mut codes: Vec<String> = Vec::new();


    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(num_str) = line {
                codes.push(num_str);

            } else {
                println!("Can't read line");
            }

        }

    } else {
        println!("Can't open input.txt");
    }


    let oxygen_val = isize::from_str_radix(&filter_codes(&codes, 0, true), 2).unwrap();
    let carbon_val = isize::from_str_radix(&filter_codes(&codes, 0, false), 2).unwrap();


    println!("Answer is oxygen: {} * carbon: {} = {}", oxygen_val, carbon_val, oxygen_val * carbon_val);
}
