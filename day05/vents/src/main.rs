use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn print(&self) {
        print!("({},{})", self.x, self.y);
    }
}

#[derive(Debug)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn print(&self) {
        self.start.print();
        print!(" to ");
        self.end.print();
    }

    pub fn get_points(&self) -> Vec<Point> {
        let mut result:Vec<Point> = Vec::new();

        if self.start.x == self.end.x {
            let mut start = self.start.y;
            let mut end = self.end.y;
            if start > end {
                start = self.end.y;
                end = self.start.y;
            }

            for y in start..=end {
                result.push( Point { x: self.start.x, y: y } );
            }
        } else if self.start.y == self.end.y {
            let mut start = self.start.x;
            let mut end = self.end.x;
            if start > end {
                start = self.end.x;
                end = self.start.x;
            }

            for x in start..=end {
                result.push( Point { x: x, y: self.start.y } );
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

/*
    for l in lines {
        l.print();
        print!("  {:?}", l.get_points());
        println!();
    }
*/
    let mut map:HashMap<Point, u32> = HashMap::new();

    for line in lines {
        for point in line.get_points() {
            if let Some(n) = map.get_mut(&point) {
                *n += 1;
            } else {
//                print!("Inserting ");
//                point.print();
//                println!();
                map.insert(point, 1);
            }
        }
    }

//    println!("Values = {:?}", map.values());

    println!("Answer is {}",
        map.values()
        .fold(0, |acc, x| if *x > 1 { acc + 1 } else { acc } )
        );
}
