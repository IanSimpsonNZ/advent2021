use std::fs;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let crabs: Vec<isize> = data_string
        .split(",")
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_pos = *min;
    let mut min_dist = crabs.iter().fold(0, |acc, crab| acc + (crab - min).abs());
    println!("pos: {}  this_dist: {}", min, min_dist);

    for pos in *min+1..=*max {
        let this_dist = crabs.iter().fold(0, |acc, crab| acc + (crab - pos).abs());
        println!("pos: {}  this_dist: {}", pos, this_dist);

        if this_dist >= min_dist {
            break;
        }

        min_dist = this_dist;
        min_pos = pos;
    }

    println!("Min fuel is {}", min_dist);
}
