use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let nums: Vec<i32> = read_numbers("./input.txt");
    let sliding: Vec<i32> = add_sliding_window(nums);
    println!("{}", count_increasing(sliding));
}

// Adds buckets of 3 numbers together in an array
fn add_sliding_window(vec: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        if i + 2 < vec.len() {
            result.push(vec[i] + vec[i + 1] + vec[i + 2]);
        }
    }
    result
}

//function that counts the number of times in a list of numbers that a value increases over the last one
fn count_increasing(list: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..list.len() {
        if i == 0 {
            continue;
        }
        if list[i] > list[i - 1] {
            count += 1;
        }
    }
    return count;
}

// function that reads a list of numbers from a file into a vector
fn read_numbers(filename: &str) -> Vec<i32> {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    let mut numbers = Vec::new();
    // Open the path in read-only mode, returns `io::Result<File>`
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

    for line in s.lines() {
        numbers.push(line.parse::<i32>().expect("Error parsing number"));
    }
    return numbers;
}
