use std::fs;
use std::collections::HashMap;

fn map_paths(map: &HashMap<String, Vec<String>>, path_so_far: &Vec<&str>, paths: &mut Vec<Vec<String>>, small_cave_visit: usize) {

//    println!("Path is {:?}", path_so_far);

    if let Some(doors) = map.get(*path_so_far.last().unwrap()) {

//        println!("Doors are {:?}", doors);

        for door in doors {
            if door == "end" {
//                println!("Found end");
                let mut full_path:Vec<String> = Vec::new();
                for step in path_so_far {
                    full_path.push(step.to_string());
                }
                full_path.push(door.clone());
                paths.push(full_path);

            } else {

                if door.chars().all(|c| matches!(c, 'a'..='z')) {
//                    println!("Checking {}", door);
                    if path_so_far.iter().any(|cave| cave == door) {
                        if small_cave_visit == 1 || door == "start" {
//                        println!("Already in path, quitting");
                        continue;
                        } else {
                            let mut new_path = path_so_far.clone();
                            new_path.push(door);
                            map_paths(map, &new_path, paths, 1);
                        }
                    } else {
                        let mut new_path = path_so_far.clone();
                        new_path.push(door);
                        map_paths(map, &new_path, paths, small_cave_visit);
                    }
                } else {
//                    println!("Checking {}", door);
                    let mut new_path = path_so_far.clone();
                    new_path.push(door);
                    map_paths(map, &new_path, paths, small_cave_visit);
                }
            }
        }
    }
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut map:HashMap<String, Vec<String>> = HashMap::new();

    for path in data_string.lines() {
        let (from, to) = path.split_once("-").unwrap();

// Insert path into map
        if let Some(doors) = map.get_mut(from) {
            doors.push(to.to_string());
        } else {
            map.insert(from.to_string(), vec![to.to_string()]);
        }
// Need to include path in both directions
        if let Some(doors) = map.get_mut(to) {
            doors.push(from.to_string());
        } else {
            map.insert(to.to_string(), vec![from.to_string()]);
        }
    }

//    println!("{:?}", map);

    let mut paths: Vec<Vec<String>> = Vec::new();

    map_paths(&map, &vec!["start"], &mut paths, 2);

    for path in paths.iter() {
        println!("{:?}", path);
    }

    println!("There are {} paths", paths.len());
}
