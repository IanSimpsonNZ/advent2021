use std::fs;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let crabs: Vec<isize> = data_string
        .split(",")
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let mut min_dist = crabs.iter().fold(0, |acc, crab| {
                                                let gap = (*crab - *min).abs();
                                                acc + gap * (gap + 1) / 2
                                            }
                                    );

    for pos in *min+1..=*max {
        let this_dist = crabs.iter().fold(0, |acc, crab| {
                                                let gap = (*crab - pos).abs();
                                                acc + gap * (gap + 1) / 2
                                            }
                                    );

        if this_dist >= min_dist {
            break;
        }

        min_dist = this_dist;
    }

    println!("Min fuel is {}", min_dist);
}
