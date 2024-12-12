use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_11", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let mut stones = input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    for i in 0..25 {
        if debug_print {
            println!("{i}: {stones:?}");
        }
        stones = blink(stones);
    }

    let mut result1 = stones.len();
    let mut result2 = 0;
    (result1 as i64, result2)
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = Vec::new();
    for stone in stones {
        let tmp: Vec<usize> = transform(stone);
        new_stones.extend(tmp);
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
