use std::collections::HashMap;
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
        Ok(_) => println!("{} contains: {} lines\n", display, s.lines().count()),
    }
    s
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn get_keys(vec: &[i32]) -> Vec<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    vec.iter().for_each(|x| {
        if counts.contains_key(x) {
            *counts.get_mut(x).unwrap() += 1;
        } else {
            counts.insert(*x, 1);
        }
    });

    return counts.keys().cloned().collect();
}

fn calc_linear_fuel(vec: &[i32], most_common: i32) -> i32 {
    vec.iter().map(|x| (x - most_common).abs()).sum::<i32>()
}

fn part1(vec: &[i32]) -> (i32, i32) {
    let mut fuel_consumed: Vec<(i32, i32)> = Vec::new();
    get_keys(vec).iter().for_each(|x| {
        fuel_consumed.push((*x, calc_linear_fuel(vec, *x)));
    });

    let mut min_v = i32::MAX;
    let mut index = 0;
    fuel_consumed.iter().for_each(|(i, consumed)| {
        if min_v >= *consumed {
            index = *i;
            min_v = *consumed;
        }
    });
    (index, min_v)
}

fn calc_gauss_fuel(vec: &[i32], cn: i32) -> i32 {
    let mut fuel: i32 = 0;
    vec.iter().for_each(|x| {
        fuel += gauss(*x, cn);
    });
    fuel
}

fn gauss_calc(start: i32, end: i32) -> i32 {
    (start + end) * (end - start + 1) / 2
}

fn gauss(start: i32, end: i32) -> i32 {
    match start.cmp(&end) {
        std::cmp::Ordering::Less => gauss_calc(1, (end-start).abs()),
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => gauss_calc(1, (start-end).abs()),
    }
}

fn part2(vec: Vec<i32>) -> i32 {
    let mut fuel_consumed: Vec<i32> = Vec::new();
    get_keys(&vec).iter().for_each(|x| {
        fuel_consumed.push(calc_gauss_fuel(&vec, *x));
    });
    let mut min_v = i32::MAX;
    fuel_consumed.iter().for_each(|x| {
        if min_v >= *x {
            min_v = *x;
        }
    });
    min_v
}



fn main() {
    let t_input = read_to_string("test_input.txt");
    let t_parsed_input = parse_input(&t_input);
    println!("Test consumed fuel is: {:?}", part1(&t_parsed_input));

    let input = read_to_string("input.txt");
    let parsed_input = parse_input(&input);
    println!("p1 consumed fuel is: {:?}", part1(&parsed_input));

    let input_2 = read_to_string("input.txt");

    let parsed_input_2 = parse_input(&input_2);
    println!("p2 consumed fuel is: {:?}", part2(parsed_input_2));
}