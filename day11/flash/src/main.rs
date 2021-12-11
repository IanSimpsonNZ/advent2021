use std::fs;

//struct octopus {
//    pub energy:u8,
//    pub
//}

fn get_neighbours(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        result.push((x - 1, y));
        if y > 0 {
            result.push((x - 1, y - 1));
        }
        if y < map.len() - 1 {
            result.push((x - 1, y + 1));
        }
    }

    if y > 0 {
        result.push((x, y - 1));
    }

    if y < map.len() - 1 {
        result.push((x, y + 1));
    }

    if x < map[0].len() - 1 {
        result.push((x + 1, y));
        if y > 0 {
            result.push((x + 1, y - 1));
        }
        if y < map.len() - 1 {
            result.push((x + 1, y + 1));
        }
    }

    result
}

fn ripple_flash(map: &mut Vec<Vec<u8>>, flashed: &mut [[bool; 10]; 10], x: usize, y: usize) -> usize {
    map[y][x] = 0;
    flashed[y][x] = true;

    let mut result = 1;

//    println!("x: {}  y: {}  n = {:?}", x, y, get_neighbours(map, x, y));

    for (nx, ny) in get_neighbours(map, x, y) {
        if !flashed[ny][nx] {
            map[ny][nx] += 1;
            if map[ny][nx] > 9 {
                result += ripple_flash(map, flashed, nx, ny);
            }
        }
    }

    result

}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut grid: Vec<Vec<u8>> = data_string
        .lines()
        .map(|l| l.chars()
                  .map(|c| c.to_digit(10).unwrap() as u8)
                  .collect())
        .collect();


    let mut num_flashes = 0;

    for _ in 1..=100 {

        let mut flashed = [[false; 10]; 10];

        for y_oct in grid.iter_mut() {
            for x_oct in y_oct.iter_mut() {
                *x_oct += 1;
            }
        }

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] > 9 && !flashed[y][x] {
                    num_flashes += ripple_flash(&mut grid, &mut flashed, x, y);
                }
            }
        }
    }

    println!("Num flashes = {}", num_flashes);
}
