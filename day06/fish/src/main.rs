use std::fs;

const NUM_DAYS:usize = 80;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Could not open file");

    let mut fishes: Vec<u32> = data_string
        .split(",")
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    for _day in 1..=NUM_DAYS {
        let mut new_fishes: Vec<u32> = Vec::new();

        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fishes.push(8);
            } else {
                *fish -= 1;
            }
        }

        fishes.append(&mut new_fishes);
    }

    println!("{} fishes", fishes.len());
}
