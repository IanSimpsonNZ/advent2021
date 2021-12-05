use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        let mut result:Vec<Point> = Vec::new();

        if self.start.x == self.end.x {    // Vertical line
            let mut start = self.start.y;
            let mut end = self.end.y;
            if start > end {
                start = self.end.y;
                end = self.start.y;
            }

            for y in start..=end {
                result.push( Point { x: self.start.x, y: y } );
            }
        } else if self.start.y == self.end.y {   // Horizontal line
            let mut start = self.start.x;
            let mut end = self.end.x;
            if start > end {
                start = self.end.x;
                end = self.start.x;
            }

            for x in start..=end {
                result.push( Point { x: x, y: self.start.y } );
            }
        } else if (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs() {  // Diagonal line
            let mut x = self.start.x;
            let mut y = self.start.y;
            let mut x_delta = 1;
            let mut y_delta = 1;

            if self.start.x > self.end.x {
                x_delta = -1;
            }

            if self.start.y > self.end.y {
                y_delta = -1;
            }

            loop {
                result.push( Point { x: x, y: y } );
                if x == self.end.x {
                    break;
                }

                x += x_delta;
                y += y_delta;
            }

        }

        result
    }
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Unable to read file");

    let lines: Vec<Line> = data_string
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(start_str, end_str)|
            (start_str.split_once(",").unwrap(),
             end_str.split_once(",").unwrap()
             )
            )
        .map(|((start_str_x, start_str_y), (end_str_x, end_str_y))|
            Line {
                start: Point { x: start_str_x.parse::<i32>().unwrap(),
                               y: start_str_y.parse::<i32>().unwrap()},
                end:   Point { x: end_str_x.parse::<i32>().unwrap(),
                               y: end_str_y.parse::<i32>().unwrap()}
            })
        .collect();


    let mut map:HashMap<Point, u32> = HashMap::new();

    for line in lines {
        for point in line.get_points() {
            if let Some(n) = map.get_mut(&point) {
                *n += 1;
            } else {
                map.insert(point, 1);
            }
        }
    }

    println!("Answer is {}",
        map.values()
        .fold(0, |acc, x| if *x > 1 { acc + 1 } else { acc } )
        );
}
