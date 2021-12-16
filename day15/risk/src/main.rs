use std::fs;

fn get_neighbours(x: usize, y: usize, map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        result.push((x - 1, y));
    }

    if x < map[0].len() - 1 {
        result.push((x + 1, y));
    }

    if y > 0 {
        result.push((x, y - 1));
    }

    if y < map.len() - 1 {
        result.push((x, y + 1));
    }

    result
}

fn calc_path(squares: &Vec<(usize, usize)>, map: &Vec<Vec<usize>>, path: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {

    let mut next_squares: Vec<(usize, usize)> = Vec::new();

    for (x, y) in squares {
        for (nx, ny) in get_neighbours(*x, *y, map) {
            if nx == map[0].len() - 1 && ny == map.len() - 1 {
                continue;
            }

            let next_step = path[*y][*x] + map[*y][*x];
            if path[ny][nx] == 0 || path[ny][nx] > next_step {
                path[ny][nx] = next_step;
                if !next_squares.contains(&(nx, ny)) {
                    next_squares.push((nx, ny));
                }
            }
        }
    }

    next_squares
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut map: Vec<Vec<usize>> = data_string
                                .lines()
                                .map(|l| l.chars()
                                          .map(|c| c.to_digit(10).unwrap() as usize)
                                          .collect())
                                .collect();

    let cols = map[0].len();
    let rows = map.len();

    for r in 0..rows {
        for i in 0..4 {
            for c in 0..cols {
                let mut n = map[r][c + i * cols] + 1;
                if n > 9 {
                    n = 1;
                }
                map[r].push(n);
            }
        }
    }

    for i in 0..4 {
        for r in 0..rows {
            map.push(Vec::new());
            for c in 0..(cols * 5) {
                let mut n = map[r + rows * i][c] + 1;
                if n > 9 {
                    n = 1;
                }
                map[r + rows * (i + 1)].push(n);
            }
        }
    }



    let mut path: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];

    let mut squares = vec![(map[0].len() - 1, map.len() - 1)];
    loop {
        squares = calc_path(&squares, &map, &mut path);
        if squares.len() == 0 {
            break;
        }
    }
    println!("Answer is {}", path[0][0]);

}
