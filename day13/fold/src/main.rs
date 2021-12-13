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


// Do the folds
    for fold in folds {

        let fold_axis = if fold.0 == 'x' { 0 } else { 1 };

        let fold_line = fold.1;

        let mut after_fold:HashSet<[u32; 2]> = HashSet::new();

        for point in coords {
            let mut new_point = point.clone();
            if point[fold_axis] > fold_line {
                new_point[fold_axis] = 2 * fold_line - new_point[fold_axis];
            }
            after_fold.insert(new_point);
        }

        coords = after_fold;
    }


// print answer

    let mut max_x = 0;
    let mut max_y = 0;

    for point in coords.iter() {
        if point[0] > max_x {
            max_x = point[0];
        }

        if point[1] > max_y {
            max_y = point[1];
        }
    }

    let mut answer = vec![vec![' '; max_x as usize + 1]; max_y as usize + 1];
    for point in coords {
        answer[point[1] as usize][point[0] as usize] = '*';
    }

    for line in answer {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }

}
