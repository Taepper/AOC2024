use std::collections::HashMap;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_11", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let mut stones = input
        .split_whitespace()
        .map(|x| (x.parse::<usize>().unwrap(), 1))
        .collect::<HashMap<usize, usize>>();
    if debug_print {
        print!("Start stones {:?}", stones);
    }

    // let mut dp_table: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..25 {
        stones = step(stones);
        if debug_print {
            print!("Step {i}: {stones:?}");
        }
    }

    let result1 = stones.iter().map(|(_, count)| count).sum::<usize>();

    for i in 25..75 {
        stones = step(stones);
        if debug_print {
            print!("Step {i}: {stones:?}");
        }
    }

    let result2 = stones.iter().map(|(_, count)| count).sum::<usize>();
    (format!("{result1}"), format!("{result2}"))
}

fn step(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();
    for (stone, count) in &stones {
        for new_stone in transform(*stone) {
            if let Some(current) = new_stones.get_mut(&new_stone) {
                *current += *count;
            } else {
                new_stones.insert(new_stone, *count);
            }
        }
    }
    new_stones
}

fn transform(number: usize) -> Vec<usize> {
    if number == 0 {
        return vec![1];
    }
    if let Some((first_half, second_half)) = split_even(number) {
        return vec![first_half, second_half];
    }
    vec![number * 2024]
}

fn split_even(number: usize) -> Option<(usize, usize)> {
    // let mut num_digits = 2;
    // let mut lower = 10;
    // let mut upper = 100; always lower * 10
    // let mut half = 10;

    // let mut num_digits = 4;
    // let mut lower = 1000; -> lower *= 100
    // let mut upper = 10000;
    // let mut half = 100; -> half *= 10
    let mut lower = 10;
    let mut half_digits = 10;
    loop {
        let upper = lower * 10;
        if number < lower {
            return None;
        }
        assert!(lower <= number);
        if number < upper {
            return Some((number / half_digits, number % half_digits));
        }
        lower *= 100;
        half_digits *= 10;
    }
}
