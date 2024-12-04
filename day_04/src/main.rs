use std::iter::{RepeatN, Rev, Zip};
use std::ops::Range;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    // println!("{:?}", do_task(&fs::read_to_string("day_04/input/demo.txt").unwrap()));
    solve_all_inputs("day_04", do_task);
}

const SEARCH_WORD: &str = "XMAS";
const SEARCH_WORD2: &str = "MAS";

fn do_task(input: &String) -> (i64, i64) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";
    let dim = input.lines().count();
    let lines: Vec<&str> = input.lines().collect();
    let mut result1 = 0;
    let target_len1 = SEARCH_WORD.len();
    let targets_iter1 = SEARCH_WORD
        .chars()
        .zip(SEARCH_WORD.chars().rev())
        .into_iter();
    for x in 0..dim - (SEARCH_WORD.len() - 1) {
        for y in 0..dim - (SEARCH_WORD.len() - 1) {
            // Diagonal
            // Direction /
            if matches_search_word(
                &lines,
                diagonal_ranges1(x,y,target_len1),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found diag / at {},{}", x, y);
                }
                result1 += 1;
            }
            // Direction \
            if matches_search_word(
                &lines,
                diagonal_ranges2(x,y,target_len1),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found diag \\ at {},{}", x, y);
                }
                result1 += 1;
            }
        }
    }
    // Horizontal
    for x in 0..dim {
        for y in 0..dim - (SEARCH_WORD.len() - 1) {
            if matches_search_word(
                &lines,
                horizontal_ranges(x, y, target_len1),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found horizontal at {},{}", x, y);
                }
                result1 += 1;
            }
        }
    }
    // Vertical
    for x in 0..dim - (SEARCH_WORD.len() - 1) {
        for y in 0..dim {
            if matches_search_word(
                &lines,
                vertical_ranges(x, y, target_len1),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found vertical at {},{}", x, y);
                }
                result1 += 1;
            }
        }
    }
    let mut result2 = 0;
    let targets_iter2 = SEARCH_WORD2
        .chars()
        .zip(SEARCH_WORD2.chars().rev())
        .into_iter();
    let target_len2 = SEARCH_WORD2.len();
    for x in 0..dim - (SEARCH_WORD2.len() - 1) {
        for y in 0..dim - (SEARCH_WORD2.len() - 1) {
            if
            // Direction /
            matches_search_word(
                &lines,
                diagonal_ranges1(x, y, target_len2),
                targets_iter2.clone(),
            ) &&
                matches_search_word(
                    &lines,
                    diagonal_ranges2(x,y, target_len2),
                    targets_iter2.clone(),
                )
            {
                if debug_print {
                    println!("Found cross at {},{}", x, y);
                }
                result2 += 1;
            }
        }
    }
    (result1, result2)
}

fn horizontal_ranges(x: usize, y: usize, target_len: usize) -> Zip<RepeatN<usize>, Range<usize>> {
    std::iter::repeat_n(x, target_len).zip(y..(y + target_len))
}

fn vertical_ranges(x: usize, y: usize, target_len: usize) -> Zip<Range<usize>, RepeatN<usize>> {
    (x..(x + target_len)).zip(std::iter::repeat_n(y, target_len))
}


// Direction \
fn diagonal_ranges1(x: usize, y: usize, target_len: usize) -> Zip<Range<usize>, Range<usize>> {
        (x..(x + target_len)).zip(
            y..(y + target_len))
    }

// Direction /
fn diagonal_ranges2(x: usize, y: usize, target_len: usize) -> Zip<Range<usize>, Rev<Range<usize>>> {
    (x..(x + target_len)).zip(
        (y..(y + target_len)).rev())
}

fn matches_search_word<I, T>(lines: &Vec<&str>, index_ranges: I, targets_iter: T) -> bool
where
    I: Iterator<Item = (usize, usize)>,
    T: Iterator<Item = (char, char)>,
{
    let mut found_backwards: bool = true;
    let mut found_forwards: bool = true;
    for ((x, y), (char_forwards, char_backwards)) in index_ranges.zip(targets_iter) {
        if lines[x].chars().nth(y).unwrap() != char_forwards {
            found_forwards = false;
        }
        if lines[x].chars().nth(y).unwrap() != char_backwards {
            found_backwards = false;
        }
    }
    found_backwards || found_forwards
}
