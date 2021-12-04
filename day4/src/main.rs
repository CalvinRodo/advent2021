use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use nalgebra::Matrix5;

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

type Board = Matrix5<(u32, bool)>;

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.split("\n\n");
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let boards = lines
        .map(|board| {
            Matrix5::from_iterator(board.lines().flat_map(|line| {
                line.split_whitespace()
                    .filter_map(|x| x.parse::<u32>().ok())
                    .map(|x| (x, false))
            }))
        })
        .collect::<Vec<Board>>();
    return (numbers, boards);
}

fn update_board(board: &mut Board, num: u32) {
    board.iter_mut().for_each(|x| {
        if x.0 == num {
            x.1 = true;
        }
    });
}

fn check_win(board: &Board) -> bool {
    let column_won = board
        .column_iter()
        .any(|col| col.iter().all(|(_, marked)| *marked));

    let row_won = board
        .row_iter()
        .any(|row| row.iter().all(|(_, marked)| *marked));

    row_won || column_won
}

fn calc_unmarked_sum(board: &Board) -> u32 {
    board
        .iter()
        .map(|(x, marked)| if !marked { *x } else { 0 })
        .sum()
}

fn part1(input: (Vec<u32>, Vec<Board>)) -> u32 {

    let (numbers, mut boards) = input;

    for num in numbers {
        for board in boards.iter_mut() {
            update_board(board, num);
            if check_win(&board) {
                return num * calc_unmarked_sum(&board);
            }
        }
    }

    unreachable!()
}

fn part2(input: (Vec<u32>, Vec<Board>)) -> u32 {

    let (numbers, mut boards) = input;

    let mut boards_won = vec![false; boards.len()];

    for num in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if boards_won[i] {
                continue;
            }
            update_board(board, num);

            if check_win(&board) {
                boards_won[i] = true;
                if boards_won.iter().all(|x| *x) {
                    return num * calc_unmarked_sum(&board);
                }
            }
        }
    }

    unreachable!()
}

fn main() {

    let test_input = &read_to_string("test_input.txt")[..];
    let parsed_test_input = parse_input(test_input);
    println!("test part1: {}", part1(parsed_test_input.clone()));
    println!("test part2: {}", part2(parsed_test_input.clone()));

    let input = &read_to_string("input.txt")[..];
    let parsed_input = parse_input(input);
    println!("part1_1: {}", part1(parsed_input.clone()));
    println!("part2_1: {}", part2(parsed_input.clone()));
}