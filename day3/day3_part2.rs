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

fn get_common_values(file_str: &String) -> Vec<[u32; 2]> {
    let line_length: usize = file_str.lines().nth(0).unwrap().chars().count();
    let mut common_nums: Vec<[u32; 2]> = vec![[0, 0]; line_length];
    file_str.lines().for_each(|line| {
        for i in 0..(line_length) {
            let character = line.chars().nth(i).unwrap();
            match character {
                '0' => common_nums[i][0] += 1,
                '1' => common_nums[i][1] += 1,
                _ => (),
            }
        }
    });

    return common_nums;
}


fn get_most_common_digit(zero: u32, one: u32) -> char {
    match zero.cmp(&one) {
        std::cmp::Ordering::Less => '1',
        std::cmp::Ordering::Greater => '0',
        std::cmp::Ordering::Equal => '1',
    }
}

fn get_least_common_digit(zero: u32, one: u32) -> char {
    match zero.cmp(&one) {
        std::cmp::Ordering::Less => '0',
        std::cmp::Ordering::Greater => '1',
        std::cmp::Ordering::Equal => '0',
    }
}

fn main() {
    let file_string = read_to_string("input.txt");

    let mut common_nums = get_common_values(&file_string);

    let row_length: usize = file_string.lines().nth(0).unwrap().len();

    let mut oxygen_generator = file_string.clone();

    for i in 0..row_length {
        let most_common = get_most_common_digit(common_nums[i][0], common_nums[i][1]);
        oxygen_generator = oxygen_generator
            .lines()
            .filter(|line| line.chars().nth(i).unwrap() == most_common)
            .collect::<Vec<&str>>()
            .join("\n");
        if oxygen_generator.lines().count() == 1 {
            break;
        }
        common_nums = get_common_values(&oxygen_generator);
    }

    println!("o2: {}", oxygen_generator);

    let mut co2_scrubber = file_string.clone();

    common_nums = get_common_values(&file_string);
    for i in 0..row_length {
        let least_common = get_least_common_digit(common_nums[i][0], common_nums[i][1]);
        co2_scrubber = co2_scrubber
            .lines()
            .filter(|line| line.chars().nth(i).unwrap() == least_common)
            .collect::<Vec<&str>>()
            .join("\n");
        if co2_scrubber.lines().count() == 1 {
            break;
        }
        common_nums = get_common_values(&co2_scrubber);
    }
    println!("co2: {}", co2_scrubber);
    println!("Oxygen Generator: {}", oxygen_generator);


    let o2_num = isize::from_str_radix(&oxygen_generator, 2).unwrap();
    let co2_num = isize::from_str_radix(&co2_scrubber, 2).unwrap();

    println!("o2_num: {}, c02_num: {}", o2_num, co2_num);
    println!("answer {}", o2_num * co2_num);
}
