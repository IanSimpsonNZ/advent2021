use std::fs;
use std::collections::{HashSet, HashMap};

struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
//    pub id: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point {
            x: x,
            y: y,
            z: z,
//            id: 0,
        }
    }

    pub fn print(&self) {
        print!("{},{},{}", self.x, self.y, self.z);
    }

    pub fn dist_sqr(&self, p: &Point) -> isize {
        let dx = p.x - self.x;
        let dy = p.y - self.y;
        let dz = p.z - self.z;

        dx * dx + dy * dy + dz * dz
    }
}

struct Triangulation {
    point: usize,
    neighbours: Vec<usize>,
}

struct Cube {
    pub num: usize,
    pub points: Vec<Point>,
    pub id_map: HashMap<isize, Vec<Triangulation>>,
}

impl Cube {
    pub fn new(num: usize) -> Self {
        Cube {
            num: num,
            points: Vec::new(),
            id_map: HashMap::new(),
        }
    }

    pub fn print(&self) {
        for point in &self.points {
            point.print();
            println!();
        }
    }

    pub fn print_map(&self) {
        for id in self.id_map.keys() {
            println!("id: {}", id);
            for t in self.id_map.get(id).unwrap() {
                self.points[t.point].print();
                print!(" -> ");
                self.points[t.neighbours[0]].print();
                print!("; ");
                self.points[t.neighbours[1]].print();
                println!();
            }
        }
    }


    pub fn create_ids(&mut self) {
        for i in 0..self.points.len() {
            let mut id_list: Vec<(isize, usize)> = Vec::new();

            for j in 0..self.points.len() {
                if i != j {
                    id_list.push((self.points[i].dist_sqr(&self.points[j]), j));
                }
            }

            id_list.sort_by(|a, b| a.0.cmp(&b.0));

            if id_list.len() >= 2 {
                let t = self.id_map
                            .entry(id_list[0].0 + id_list[1].0)
                            .or_insert_with(|| Vec::new());
                t.push(
                    Triangulation {
                        point: i,
                        neighbours: vec![id_list[0].1, id_list[1].1]
                    }
                );
            } else {
                panic!("Fewer than 2 points in cube {}", self.num);
            }
        }
    }
}


fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't open file");

    let mut cube_list: Vec<Cube> = Vec::new();

    for line in data_string.lines() {
        if line.contains("scanner") {
            cube_list.push(Cube::new(line
                                    .split_once("er ").unwrap().1
                                    .split_once(" ").unwrap().0
                                    .parse::<usize>().unwrap()
                                    ));
        } else {
            if line.len() != 0 {
                let coords:Vec<isize> = line.split(",")
                                             .map(|c_str| c_str.parse::<isize>().unwrap())
                                             .collect();
                cube_list
                    .last_mut()
                    .unwrap()
                    .points
                    .push(
                        Point::new(
                            coords[0],
                            coords[1],
                            coords[2]
                        )
                    )
            }
        }
    }


    for cube in cube_list.iter_mut() {
        cube.create_ids();

//        println!("--- scanner {} ---", cube.num);
//        cube.print_map();
//        println!();
   }

    let mut hit_list: Vec<HashSet<isize>> = Vec::new();
    let mut max_hits = 0;
    let mut max_hits_id = 0;

    let ids0: HashSet<isize> = cube_list[0].id_map.keys().cloned().collect();

    for cube_idx in 1..cube_list.len() {
        let ids1: HashSet<isize> = cube_list[cube_idx].id_map.keys().cloned().collect();

        let common = ids0.intersection(&ids1).cloned().collect();
        hit_list.push(common);

        println!("Cube {} has {} hits", cube_list[cube_idx].num, hit_list[cube_idx - 1].len());

        if hit_list[cube_idx - 1].len() > max_hits {
            max_hits = hit_list[cube_idx - 1].len();
            max_hits_id = cube_idx;
        }
    }

    println!("Cube {} has most hits ({})", cube_list[max_hits_id].num, max_hits);
    println!();
    for id in &hit_list[max_hits_id - 1] {
        let t_list = cube_list[0].id_map.get(id).unwrap();
        cube_list[0].points[t_list[0].point].print();
        println!();
    }
}
