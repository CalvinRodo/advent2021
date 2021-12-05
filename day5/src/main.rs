use line_drawing::Bresenham;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;

fn read_to_string(filename: &str) -> String {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, retuResult<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains: {} lines\n", display, s.lines().count()),
    }
    s
}

fn parse_input_as_bresenham_lines(input: &str, ignore_diag: bool) -> Vec<Bresenham<i32>> {
    let mut lines = Vec::new();
    for line in input.lines() {
        let mut line_parts = line.split(" -> ");
        let (x1, y1) = parse_coord_pairs(line_parts.next().unwrap());
        let (x2, y2) = parse_coord_pairs(line_parts.next().unwrap());
        if ignore_diag {
            if x1 == x2 || y1 == y2 {
                lines.push(Bresenham::new((x1, y1), (x2, y2)));
            } else {
                continue;
            }
        } else {
            lines.push(Bresenham::new((x1, y1), (x2, y2)));
        }
    }
    lines
}

fn parse_coord_pairs(line: &str) -> (i32, i32) {
    let mut line_parts = line.split(',');
    let x = line_parts.next().unwrap().parse::<i32>().unwrap();
    let y = line_parts.next().unwrap().parse::<i32>().unwrap();
    (x, y)
}

fn count_overlaps(lines: Vec<Vec<(i32, i32)>>) -> i32 {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines.clone() {
        line.iter().for_each(|coord| {
            if grid.contains_key(coord) {
                *grid.get_mut(coord).unwrap() += 1;
            } else {
                grid.insert(*coord, 1);
            }
        });
    }

    grid.values().filter(|&x| *x > 1).count() as i32
}

fn convert_to_list_of_pointlines(lines: Vec<Bresenham<i32>>) -> Vec<Vec<(i32, i32)>> {
    let mut pointlines = Vec::new();
    for line in lines {
        let pointline = line.collect();
        pointlines.push(pointline);
    }
    pointlines
}

fn main() {
    println!("Day 5 Test Input");
    let test_input = read_to_string("test_input.txt");
    let test_lines = parse_input_as_bresenham_lines(&test_input, true);
    let test_plines = convert_to_list_of_pointlines(test_lines);
    let test_overlaps = count_overlaps(test_plines);
    println!("Overlaps: {}", test_overlaps);
    println!("part 1");
    let input = read_to_string("input.txt");
    let lines = parse_input_as_bresenham_lines(&input, true);
    let plines = convert_to_list_of_pointlines(lines);
    let overlaps = count_overlaps(plines);
    println!("Overlaps: {}", overlaps);
    
    println!("part2");
    let input = read_to_string("input.txt");
    let lines = parse_input_as_bresenham_lines(&input, false);
    let plines = convert_to_list_of_pointlines(lines);
    let overlaps = count_overlaps(plines);
    println!("Overlaps: {}", overlaps);
}
