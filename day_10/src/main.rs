use std::collections::HashSet;
use tae_aoclib2025::{solve_all_inputs};

fn main() {
    solve_all_inputs("day_10", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 500;

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let board = parse_input(&input);

    let mut reachable_targets: Vec<Vec<HashSet<(usize, usize)>>> = vec![vec![HashSet::new(); cols]; rows];
    let mut hiking_trails: Vec<Vec<usize>> = vec![vec![0; cols]; rows];
    for row in 0..rows{
        for col in 0..cols{
            if board[row][col] == 9 {
                reachable_targets[row][col].insert((row, col));
                hiking_trails[row][col] = 1;
            }
        }
    }
    for i in (0..9).rev() {
        for row in 0..rows{
            for col in 0..cols{
                if board[row][col] == i {
                    if row + 1 < rows && board[row+1][col] == i + 1 {
                        let reachable_from_neighbor: HashSet<(usize, usize)> = reachable_targets[row+1][col].iter().cloned().collect();
                        reachable_targets[row][col].extend(reachable_from_neighbor);
                        hiking_trails[row][col] += hiking_trails[row+1][col];
                    }
                    if col + 1 < rows && board[row][col+1] == i + 1 {
                        let reachable_from_neighbor: HashSet<(usize, usize)> = reachable_targets[row][col+1].iter().cloned().collect();
                        reachable_targets[row][col].extend(reachable_from_neighbor);
                        hiking_trails[row][col] += hiking_trails[row][col+1];
                    }
                    if row >= 1 && board[row-1][col] == i + 1 {
                        let reachable_from_neighbor: HashSet<(usize, usize)> = reachable_targets[row-1][col].iter().cloned().collect();
                        reachable_targets[row][col].extend(reachable_from_neighbor);
                        hiking_trails[row][col] += hiking_trails[row-1][col];
                    }
                    if col >= 1 && board[row][col-1] == i + 1 {
                        let reachable_from_neighbor: HashSet<(usize, usize)> = reachable_targets[row][col-1].iter().cloned().collect();
                        reachable_targets[row][col].extend(reachable_from_neighbor);
                        hiking_trails[row][col] += hiking_trails[row][col-1];
                    }
                }
            }
        }
    }

    if debug_print {
        println!("{:}", board.iter().map(|x| x.iter().map(|c| format!("{c}")).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
        println!();
        println!("{:}", reachable_targets.iter().map(|x| x.iter().map(|c| format!("{}", c.len())).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
    }

    let mut result1 = 0;
    let mut result2 = 0;
    for row in 0..rows{
        for col in 0..cols{
            if board[row][col] == 0 {
                result1 += reachable_targets[row][col].len();
                result2 += hiking_trails[row][col];
            }
        }
    }

    (result1 as i64, result2 as i64)
}

fn parse_input(input: &String) -> Vec<Vec<usize>> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut board = vec![vec![0; cols]; rows];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            board[row][col] = c as usize - '0' as usize;
        }
    }
    board
}
