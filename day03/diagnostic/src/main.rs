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

fn main() {
    let mut codes: Vec<String> = Vec::new();


    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(num_str) = line {
                codes.push(num_str);
//                num_lines += 1;

            } else {
                println!("Can't read line");
            }

        }

    } else {
        println!("Can't open input.txt");
    }


    let mut oxygen = codes.clone();
    let mut index = 0;

    while oxygen.len() > 1 {
        let mut most_common_bit = '0';
        let num_ones = count_ones(&oxygen, index);
        let num_zeros = oxygen.len() - num_ones;
        if num_ones >= num_zeros {
            most_common_bit = '1';
        }

        let mut tmp:Vec<String> = Vec::new();
        for code in oxygen {
            let mut chars = code.chars();
            if chars.nth(index).unwrap() == most_common_bit {
                tmp.push(code);
            }
        }

        oxygen = tmp;
        println!("{:?}", oxygen);
        println!();
        index += 1;
    }


    let oxygen_val = isize::from_str_radix(&oxygen[0], 2).unwrap();

    let mut carbon = codes.clone();
    index = 0;

    while carbon.len() > 1 {
        let mut least_common_bit = '1';
        let num_ones = count_ones(&carbon, index);
        let num_zeros = carbon.len() - num_ones;
        if num_ones >= num_zeros {
            least_common_bit = '0';
        }

        let mut tmp:Vec<String> = Vec::new();
        for code in carbon {
            let mut chars = code.chars();
            if chars.nth(index).unwrap() == least_common_bit {
                tmp.push(code);
            }
        }

        carbon = tmp;
        println!("{:?}", carbon);
        println!();
        index += 1;
    }




    let carbon_val = isize::from_str_radix(&carbon[0], 2).unwrap();


    println!("Answer is oxygen: {} * carbon: {} = {}", oxygen_val, carbon_val, oxygen_val * carbon_val);
}
