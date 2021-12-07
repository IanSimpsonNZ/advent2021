use std::fs;
use std::cmp;

fn calc_fuel(from:isize, to:isize) -> isize {
    let mut step = 1;
    let mut result = 0;
    let f = cmp::min(from, to);
    let t = cmp::max(from, to);
    for _ in f..t {
        result += step;
        step += 1;
    }

    result
}


fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let crabs: Vec<isize> = data_string
        .split(",")
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_dist = crabs.iter().fold(0, |acc, crab| acc + calc_fuel(*crab, *min));


    for pos in *min+1..=*max {
        let this_dist = crabs.iter().fold(0, |acc, crab| acc + calc_fuel(*crab, pos));

        if this_dist >= min_dist {
            break;
        }

        min_dist = this_dist;
    }

    println!("Min fuel is {}", min_dist);
}
