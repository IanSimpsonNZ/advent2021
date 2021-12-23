use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Clone)]
struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point {
            x: x,
            y: y,
            z: z,
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

    pub fn rotate(&self, r_num: usize) -> Self {
        match r_num {
            1 => Point::new(self.x, self.y, self.z),
            2 => Point::new(self.z, self.y, -self.x),
            3 => Point::new(-self.x, self.y, -self.z),
            4 => Point::new(-self.z, self.y, self.x),
            5 => Point::new(-self.x, -self.y, self.z),
            6 => Point::new(self.z, -self.y, self.x),
            7 => Point::new(self.x, -self.y, -self.z),
            8 => Point::new(-self.z, -self.y, -self.x),
            9 => Point::new(self.x, self.z, -self.y),
            10 => Point::new(-self.y, self.z, -self.x),
            11 => Point::new(-self.x, self.z, self.y),
            12 => Point::new(self.y, self.z, self.x),
            13 => Point::new(self.x, -self.z, self.y),
            14 => Point::new(self.y, -self.z, -self.x),
            15 => Point::new(-self.x, -self.z, -self.y),
            16 => Point::new(-self.y, -self.z, self.x),
            17 => Point::new(-self.y, self.x, self.z),
            18 => Point::new(self.z, self.x, self.y),
            19 => Point::new(self.y, self.x, -self.z),
            20 => Point::new(-self.z, self.x, -self.y),
            21 => Point::new(self.y, -self.x, self.z),
            22 => Point::new(self.z, -self.x, -self.y),
            23 => Point::new(-self.y, -self.x, -self.z),
            24 => Point::new(-self.z, -self.x, self.y),
            _ => panic!("Invalif rotation code"),
        }
    }


    pub fn _add(&self, p: &Point) -> Self {
        Point::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }


    pub fn sub(&self, p: &Point) -> Self {
        Point::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }

    pub fn translate(&self, t: &Translation) -> Self {
        self.rotate(t.rotation).sub(&t.shift)
    }

    pub fn manhattan(&self, p: &Point) -> isize {
        (self.x - p.x).abs() +
        (self.y - p.y).abs() +
        (self.z - p.z).abs()
    }
}

struct Triangulation {
    point: usize,
    neighbours: Vec<usize>,
}

struct Translation {
    rotation: usize,
    shift: Point,
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

    pub fn _print(&self) {
        for point in &self.points {
            point.print();
            println!();
        }
    }

    pub fn _print_map(&self) {
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

    pub fn merge(&mut self, c: &Cube, t: &Translation) {
        for new_point in &c.points {
            let translated = new_point.translate(t);
            if !self.points.contains(&translated) {
                self.points.push(translated);
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
    let mut scanner_pos: Vec<Point> = Vec::new();

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
    }

    let mut universe = cube_list.swap_remove(0);
    scanner_pos.push(Point::new(0, 0, 0));

    while cube_list.len() > 0 {
        let mut hit_list: Vec<HashSet<isize>> = Vec::new();
        let mut max_hits = 0;
        let mut max_hits_id = 0;

        let ids0: HashSet<isize> = universe.id_map.keys().cloned().collect();

        for cube_idx in 0..cube_list.len() {
            let ids1: HashSet<isize> = cube_list[cube_idx].id_map.keys().cloned().collect();

            let common = ids0.intersection(&ids1).cloned().collect();
            hit_list.push(common);

            println!("Cube {} has {} hits", cube_list[cube_idx].num, hit_list[cube_idx].len()); // -1

            if hit_list[cube_idx].len() > max_hits {  // -1
                max_hits = hit_list[cube_idx].len(); // -1
                max_hits_id = cube_idx;
            }
        }

        println!("Cube {} has most hits ({})", cube_list[max_hits_id].num, max_hits);

        const NUM_ROTATIONS: usize = 24;
        let mut max_hits = 0;
        let mut max_tran = Translation {
            rotation: 0,
            shift: Point::new(0, 0, 0)};

        for r in 1..=NUM_ROTATIONS {
            let matched_ids = &mut hit_list[max_hits_id].iter(); // -1
            let id = matched_ids.next().unwrap();

            let mut num_hits = 0;
            let t_list = universe.id_map.get(&id).unwrap();
            let p1 = &universe.points[t_list[0].point];
            let t_list = cube_list[max_hits_id].id_map.get(&id).unwrap();
            let p2 = &cube_list[max_hits_id].points[t_list[0].point];

            let shift = p2.rotate(r).sub(&p1);

            let trans = Translation {
                rotation: r,
                shift: shift,
            };
            for next_id in &mut *matched_ids {
                let t_list = universe.id_map.get(&next_id).unwrap();
                let p1 = &universe.points[t_list[0].point];
                let t_list = cube_list[max_hits_id].id_map.get(&next_id).unwrap();
                let p2 = &cube_list[max_hits_id].points[t_list[0].point];
                if *p1 == p2.translate(&trans) {
                    num_hits += 1;
                }
            }

            if num_hits > max_hits {
                max_hits = num_hits;
                max_tran = trans;
            }

        }

        print!("Best match is rotation {}, shift ", max_tran.rotation);
        max_tran.shift.print();
        scanner_pos.push(max_tran.shift.clone());
        println!();
        println!();

        universe.merge(&mut cube_list.swap_remove(max_hits_id), &max_tran);
        universe.create_ids();
    }

    println!("There are {} beacons", universe.points.len());

    let mut max_manhattan = 0;
    for a in 0..scanner_pos.len() {
        for b in 0..scanner_pos.len() {
            if a != b {
                let m = scanner_pos[a].manhattan(&scanner_pos[b]);
                if m > max_manhattan {
                    max_manhattan = m;
                }
            }
        }
    }

    println!("Max Manhattan distance is {}", max_manhattan);
}
