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
    input.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()

}


fn calc_fish(input: &str, gens: usize) -> i64 {
    let parsed_input = parse_input(input);
    let mut fishmap: Vec<i64> = vec![0; 9]; 
    parsed_input.iter().for_each(|x| {
        fishmap[*x as usize] += 1;
    });

    for _ in 0 .. gens { 
        let num = fishmap.remove(0);
        fishmap[6] += num;
        fishmap.push(num);
    }

    fishmap.iter().sum::<i64>()
}

fn main() {
    let test_input = read_to_string("test_input.txt");
    let test_num = calc_fish(&test_input, 80);
    println!("test_num: {}", test_num);

    let input = read_to_string("input.txt");
    let num = calc_fish(&input, 80);
    println!("part 1: {}", num);

    let num = calc_fish(&input, 256);
    println!("part 2: {}", num);

}


