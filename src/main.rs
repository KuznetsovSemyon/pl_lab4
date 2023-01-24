use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Clone, Copy)]
struct Object {
    a: Point,
    b: Point,
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn check_point(seg: Object, p: Point) -> bool {
    if (p.x <= seg.a.x) & (p.y <= seg.a.y) & (p.x >= seg.b.x) & (p.y >= seg.b.y) |
       (p.x >= seg.a.x) & (p.y >= seg.a.y) & (p.x <= seg.b.x) & (p.y <= seg.b.y) 
    {
        return true;
    };
    return false;
}

fn find_distance(half_line: Object, segment: Object) -> f64 { 
    let afd = segment.b.y - segment.a.y;
    let bfd = segment.a.x - segment.b.x;
    let cfd = -1.0 * segment.a.x * segment.b.y + segment.a.y * segment.b.x;

    let v = half_line.b.x - half_line.a.x;
    let w = half_line.b.y - half_line.a.y;

    if (afd * v + bfd * w) != 0.0 {
        let t = (-1.0 * afd * half_line.a.x - bfd * half_line.a.y - cfd) / (afd * v + bfd * w);
        if t >= 0.0 {
            let x = half_line.a.x + v * t;
            let y = half_line.a.y + w * t;
            if check_point(Object{a: Point{x: segment.a.x, y: segment.a.y},b: Point{x: segment.b.x, y: segment.b.y} }, Point{x, y}) {
                let exp = (half_line.a.x - x).powf(2.0) + (half_line.a.y - y).powf(2.0);
                return exp.sqrt();
            } else {
                return 1.0;
            }
        } else {
            return -2.0;
        }
    } else {
        return -1.0;
    }
}

fn main() {
    let filename = "./data/inputData.txt";
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut object_list = Vec::new();
    
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let paresd_line: Vec<&str> = line.split(' ').collect();
        let parsed_coord1: Vec<&str> = paresd_line[0].split(',').collect();
        let parsed_coord2: Vec<&str> = paresd_line[1].split(',').collect();
        object_list.push(Object { a: Point{ x: parsed_coord1[0].parse().unwrap(),
                                           y: parsed_coord1[1].parse().unwrap()},
                                 b: Point{ x: parsed_coord2[0].parse().unwrap(),
                                           y: parsed_coord2[1].parse().unwrap()}});
    }

    let half_line = object_list[0];
    let mut nearest_segment = half_line;
    let mut nearest_d = -100.0;

    for sgt in 1..object_list.len() {
        let current_d = find_distance(half_line, object_list[sgt]);
        if ((nearest_d < 0.0) | (current_d < nearest_d)) & (current_d >= 0.0) {
            nearest_d = current_d;
            nearest_segment = object_list[sgt];
        };
    }

    if nearest_d > 0.0 {
        println!("{}, {}, {}, {}", nearest_segment.a.x, nearest_segment.a.y, nearest_segment.b.x, nearest_segment.b.y)
    } else {
        println!("");
    }
}