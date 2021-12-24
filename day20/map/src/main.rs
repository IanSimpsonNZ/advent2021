use std::fs;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn is_inside(&self, top_left: &Point, bottom_right: &Point) -> bool {
        self.x >= top_left.x &&
        self.x <= bottom_right.x &&
        self.y >= top_left.y &&
        self.y <= bottom_right.y
    }
}

#[derive(Clone)]
struct Map {
    points: HashMap<Point, u8>,
    top_left: Point,
    bottom_right: Point,
    infinite: u8,
}

impl Map {
    pub fn new() -> Self {
        Map {
            points: HashMap::new(),
            top_left: Point{x: 0, y: 0},
            bottom_right: Point{x: 0, y: 0},
            infinite: 0,
        }
    }

    pub fn print(&self) {
        for y in self.top_left.y..=self.bottom_right.y {
            for x in self.top_left.x..=self.bottom_right.x {
                if let Some(_) = self.points.get(&Point {x: x, y: y}) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("Infinite is {}", self.infinite);
        println!();

    }

    pub fn insert(&mut self, p: &Point) {
        if p.x < self.top_left.x { self.top_left.x = p.x; }
        if p.y < self.top_left.y { self.top_left.y = p.y; }
        if p.x > self.bottom_right.x { self.bottom_right.x = p.x; }
        if p.y > self.bottom_right.y { self.bottom_right.y = p.y; }
        self.points.insert(*p, 1);
    }

    fn calc(&self, pixel: &Point, enhancer: &Vec<u8>) -> bool {
        let mut index = 0;

        for p in get_square(pixel) {
            index *= 2;
            if p.is_inside(&self.top_left, &self.bottom_right) {
                if let Some(_) = self.points.get(&p) {
                    index += 1;
                }
            } else {
                index += self.infinite as usize;
            }
        }

        enhancer[index] == b'#'

    }

    fn _check_infinite(&mut self, enhancer: &Vec<u8>) -> bool {
        let mut new_map = self.clone();
        let mut go_again = false;

        for x in self.top_left.x - 1 ..= self.bottom_right.x + 1 {
            let top_point = Point {x: x, y: self.top_left.y - 1};
            let bottom_point = Point {x: x, y: self.bottom_right.y + 1};

            if self.calc(&top_point, enhancer) {
                new_map.insert(&top_point);
                go_again = self.infinite == 0;
            }

            if self.calc(&bottom_point, enhancer) {
                new_map.insert(&bottom_point);
                go_again = self.infinite == 0;
            }
        }

        for y in self.top_left.y ..= self.bottom_right.y {
            let left_point = Point{ x: self.top_left.x - 1, y: y };
            let right_point = Point{ x: self.bottom_right.x + 1, y: y };

            if self.calc(&left_point, enhancer) {
                new_map.insert(&left_point);
                go_again = self.infinite == 0;
            }

            if self.calc(&right_point, enhancer) {
                new_map.insert(&right_point);
                go_again = self.infinite == 0;
            }
        }

        self.points = new_map.points;
        self.top_left = new_map.top_left;
        self.bottom_right = new_map.bottom_right;

        self.print();
        println!();

        for _ in 1..10000000 {}

        go_again
    }

    pub fn enhance(&self, enhancer: &Vec<u8>) -> Self {
        let mut new_image = Map::new();

        for x in self.top_left.x - 1 ..= self.bottom_right.x + 1 {
            for y in self.top_left.y - 1 ..= self.bottom_right.y + 1 {
//                for p in get_square(&Point{x: x as isize, y: y as isize}) {
                let p = Point {x: x as isize, y: y as isize};
                    if self.calc(&p, enhancer) {
                        new_image.insert(&p);
                    }
//                }
            }
        }

        new_image.infinite = if enhancer[if self.infinite == 1 { 511 } else { 0 }] == b'#' { 1 } else { 0 };

//        while new_image.check_infinite(enhancer) {}


        new_image
    }
}

fn get_square(p: &Point) -> Vec<Point> {
    vec![
        Point {x: p.x - 1, y: p.y - 1},
        Point {x: p.x    , y: p.y - 1},
        Point {x: p.x + 1, y: p.y - 1},
        Point {x: p.x - 1, y: p.y},
        Point {x: p.x    , y: p.y},
        Point {x: p.x + 1, y: p.y},
        Point {x: p.x - 1, y: p.y + 1},
        Point {x: p.x    , y: p.y + 1},
        Point {x: p.x + 1, y: p.y + 1}
    ]
}

fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut lines = data_string.lines();

    let enhancer = lines.next().unwrap().to_string().into_bytes();
    lines.next();


    let mut image = Map::new();
    for (y, l) in lines.enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                image.insert(&Point{x: x as isize, y: y as isize});
            }
        }
    }

    println!();

    image.print();

    println!();
    println!();

    for i in 0..50 {
        image = image.enhance(&enhancer);
        println!("{}...", i);
    }

    image.print();

    println!();
    println!();

    println!("Image has {} lit pixels", image.points.len());
}
