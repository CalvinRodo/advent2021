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

fn build_strings_from_common_chars(a: Vec<[u32;2]>) -> (String, String) {
    let mut s1 = String::new();
    let mut s2 = String::new();
    for i in 0..a.len() {
      if a[i][0] > a[i][1] {
        s1.push('0');
        s2.push('1');
      } else {
        s1.push('1');
        s2.push('0');
      }
    }
    return (s1, s2);
}

fn main() {
    let file_string = read_to_string("input.txt");
    let line_length: usize = file_string.lines().nth(0).unwrap().chars().count();
    let mut common_nums: Vec<[u32; 2]> = vec![[0, 0]; line_length];

    file_string.lines().for_each(|line| {
        for i in 0..(line_length) {
            let character = line.chars().nth(i).unwrap();
            match character {
                '0' => common_nums[i][0] += 1,
                '1' => common_nums[i][1] += 1,
                _ => (),
            }
        }
    });

    let (s1, s2) = build_strings_from_common_chars(common_nums);
    println!("s1: {}, s2: {}", s1, s2);

    let i1 = isize::from_str_radix(&s1, 2).unwrap();
    let i2 = isize::from_str_radix(&s2, 2).unwrap();

    println!("i1: {}, i2: {}", i1, i2);
    println!("answer {}", i1 * i2);

}
