use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
        Ok(_) => print!("{} contains: {} lines\n", display, s.lines().count()),
    }
    return s;
}

fn split_lines(s: &str) -> Vec<(&str, i32)> {
    return s
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let word = words.next().unwrap();
            let number = words.next().unwrap().parse::<i32>().unwrap();
            return (word, number);
        })
        .collect();
}

fn add_direction(direction: &str, val: i32, depth: i32, horiz: i32) -> (i32, i32) {
    match direction {
        "up" => (horiz, depth + val),
        "down" => (horiz, depth - val),
        "forward" => (horiz + val, depth),
        _ => panic!("unknown direction"),
    }
}

fn main() {
    let s = read_to_string("./input.txt");
    let d = split_lines(&s);

    let mut horiz = 0;
    let mut depth = 0;

    d.iter().for_each(|(direction, val)| {
        let (h, d) = add_direction(direction, *val, depth, horiz);
        horiz = h;
        depth = d;
    });

    println!("{}", horiz.abs() * depth.abs());
}
