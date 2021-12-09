use std::fs;

fn get_neigbours(map: &Vec<Vec<u8>>, x: usize, y:usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    if y > 0 {
        result.push((x, y - 1));
    }

    if y < map.len() - 1 {
        result.push((x, y + 1));
    }

    if x > 0 {
        result.push((x - 1, y));
    }

    if x < map[0].len() - 1 {
        result.push((x + 1, y));
    }

    result
}
fn check_low(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Option<u8> {

    for (nx, ny) in get_neigbours(&map, x, y) {
        if map[y][x] >= map[ny][nx] {
            return None;
        }
    }

    Some(map[y][x])
}

fn get_size(map: &Vec<Vec<u8>>, checked: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut result = 0;

    if !checked[y][x] {
        checked[y][x] = true;
        if map[y][x] < 9 {
            result += 1;
        }
    }

    for (nx, ny) in get_neigbours(&map, x, y) {
        if !checked[ny][nx] && map[ny][nx] < 9 {
                result += get_size(map, checked, nx, ny);
        }
    }

    result
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let map:Vec<Vec<u8>> = data_string
        .lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
         )
        .collect();


    let mut basins: Vec<usize> = Vec::new();
//    let mut result:usize = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let Some(_) = check_low(&map, x, y) {

//                result += low as usize + 1;

                let mut checked = vec![vec![false; map[0].len()]; map.len()];

                basins.push(get_size(&map, &mut checked, x, y));
            }
        }
    }

//    println!("{}", result);
    basins.sort();
    basins.reverse();

    println!("{:?}", basins);
    println!("{}", basins[0] * basins[1] * basins[2]);
}
