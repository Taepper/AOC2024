use std::iter::{Rev, Zip};
use std::str::Chars;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_04", do_task);
    // println!("{:?}", do_task(&fs::read_to_string("day_04/input/demo.txt").unwrap()));
}

const SEARCH_WORD: &str = "XMAS";
const SEARCH_WORD2: &str = "MAS";

fn do_task(input: &String) -> (i64, i64) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";
    let dim = input.lines().count();
    let lines: Vec<&str> = input.lines().collect();
    let mut result1 = 0;
    let targets_iter1 = SEARCH_WORD
        .chars()
        .zip(SEARCH_WORD.chars().rev())
        .into_iter();

    for x in 0..dim - (SEARCH_WORD.len() - 1) {
        for y in 0..dim - (SEARCH_WORD.len() - 1) {
            if matches_search_word(
                &lines,
                std::iter::repeat_n(x, SEARCH_WORD.len()),
                y..(y + SEARCH_WORD.len()),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found horizontal at {},{}", x, y);
                }
                result1 += 1;
            }
            if matches_search_word(
                &lines,
                x..(x + SEARCH_WORD.len()),
                std::iter::repeat_n(y, SEARCH_WORD.len()),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found vertical at {},{}", x, y);
                }
                result1 += 1;
            }
            // Diagonal
            // Direction /
            if matches_search_word(
                &lines,
                x..(x + SEARCH_WORD.len()),
                (y..(y + SEARCH_WORD.len())).rev().into_iter(),
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
                x..(x + SEARCH_WORD.len()),
                y..(y + SEARCH_WORD.len()),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found diag \\ at {},{}", x, y);
                }
                result1 += 1;
            }
        }
    }
    // Horizontal follow-up
    for x in dim - (SEARCH_WORD.len() - 1)..dim {
        for y in 0..dim - (SEARCH_WORD.len() - 1) {
            if matches_search_word(
                &lines,
                std::iter::repeat_n(x, SEARCH_WORD.len()),
                y..(y + SEARCH_WORD.len()),
                targets_iter1.clone(),
            ) {
                if debug_print {
                    println!("Found horizontal at {},{}", x, y);
                }
                result1 += 1;
            }
        }
    }
    // Vertical follow-up
    for x in 0..dim - (SEARCH_WORD.len() - 1) {
        for y in dim - (SEARCH_WORD.len() - 1)..dim {
            if matches_search_word(
                &lines,
                x..(x + SEARCH_WORD.len()),
                std::iter::repeat_n(y, SEARCH_WORD.len()),
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
    for x in 0..dim - (SEARCH_WORD2.len() - 1) {
        for y in 0..dim - (SEARCH_WORD2.len() - 1) {
            if
            // Direction /
            matches_search_word(
                &lines,
                x..(x + SEARCH_WORD2.len()),
                (y..(y + SEARCH_WORD2.len())).rev().into_iter(),
                targets_iter2.clone(),
            ) &&
                // Direction \
                matches_search_word(
                    &lines,
                    x..(x + SEARCH_WORD2.len()),
                    y..(y + SEARCH_WORD2.len()),
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

fn matches_search_word<I, J>(lines: &Vec<&str>, x_iter: I, y_iter: J, targets_iter: Zip<Chars, Rev<Chars>>) -> bool
where
    I: Iterator<Item = usize>,
    J: Iterator<Item = usize>,
{
    let mut found_backwards: bool = true;
    let mut found_forwards: bool = true;
    for ((x, y), (char_forwards, char_backwards)) in x_iter.zip(y_iter).zip(targets_iter) {
        if lines[x].chars().nth(y).unwrap() != char_forwards {
            found_forwards = false;
        }
        if lines[x].chars().nth(y).unwrap() != char_backwards {
            found_backwards = false;
        }
    }
    found_backwards || found_forwards
}
