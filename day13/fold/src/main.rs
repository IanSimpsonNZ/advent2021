use std::fs;
use std::collections::HashSet;

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut line = data_string.lines();

    let mut coords: HashSet<[u32; 2]> = HashSet::new();

// Get points
    loop {
        if let Some((x_str, y_str)) = line
                                .next()
                                .unwrap()
                                .split_once(",")
        {
            coords.insert([x_str.parse::<u32>().unwrap(),
                           y_str.parse::<u32>().unwrap()]);
        } else {
            break;
        }
    }

// Get folds
    let folds: Vec<(char, u32)> = line
                                    .map(|l| l.split_once("=").unwrap())
                                    .map(|(s1, s2)| (s1.chars().last().unwrap(),
                                                     s2.trim().parse::<u32>().unwrap()))
                                    .collect();


    let fold_axis = if folds[0].0 == 'x' { 0 } else { 1 };

    let fold_line = folds[0].1;

    let mut after_fold:HashSet<[u32; 2]> = HashSet::new();

    for point in coords {
       let mut new_point = point.clone();
       if point[fold_axis] > fold_line {
            new_point[fold_axis] = 2 * fold_line - new_point[fold_axis];
       }
       after_fold.insert(new_point);
    }

    println!("Answer is {}", after_fold.len());
}
