use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_07", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";
    let mut result1 = 0;
    let mut result2: u128 = 0;

    for line in input.lines() {
        let (goal, numbers) = line
            .split_once(": ")
            .map(|(goal_str, numbers_str)| {
                (
                    goal_str.parse::<u64>().unwrap(),
                    numbers_str
                        .split_whitespace()
                        .map(|number_str| {
                            (number_str.parse::<u64>().unwrap(), number_str.len() as u32)
                        })
                        .collect::<Vec<(u64, u32)>>(),
                )
            })
            .unwrap();
        let n = numbers.len();
        assert!(n < 16);
        for seed in 0..(1 << (n - 1)) {
            if execute(&numbers, seed) == goal {
                result1 += goal;
                break;
            }
        }
        if any_matches(&numbers, goal, execute_three_backwards) {
            result2 += goal as u128;
        }
    }
    (result1 as i64, result2 as i64)
}

fn any_matches(numbers: &Vec<(u64, u32)>, goal: u64, f: fn(&Vec<(u64, u32)>, u64, u64) -> bool) -> bool {
    let n = numbers.len();
    let mut seed = 0;
    while seed < (1 << (2 * n - 1)) {
        if f(&numbers, seed, goal)
            || f(&numbers, seed + 1, goal)
            || f(&numbers, seed + 2, goal)
        {
            return true;
        }
        seed += 4;
    }
    false
}

fn valid_ternary_seed(mut seed: u64) -> bool {
    while seed > 0 {
        if seed & 0b11 == 0b11 {
            return false;
        }
        seed >>= 2;
    }
    true
}

fn execute_three(numbers: &Vec<(u64, u32)>, mut seed: u64, goal: u64) -> bool {
    if !valid_ternary_seed(seed) {
        return false;
    }
    let mut numbers = numbers.iter();
    let (mut result, _strlen) = *numbers.next().unwrap();
    for (n, strlen) in numbers {
        assert_ne!(seed & 0b11, 0b11);
        if seed & 0b11 == 0b00 {
            result += *n;
        } else if seed & 0b11 == 0b01 {
            result *= *n;
        } else if seed & 0b11 == 0b10 {
            result *= 10_u64.pow(*strlen);
            result += n;
        }
        if result > goal {
            return false;
        }
        seed >>= 2;
    }
    result == goal
}

fn execute_three_backwards(numbers: &Vec<(u64, u32)>, mut seed: u64, goal: u64) -> bool {
    if !valid_ternary_seed(seed) {
        return false;
    }
    let numbers_iter = numbers.iter().skip(1).rev();
    let mut result = goal;
    for (n, strlen) in numbers_iter {
        assert_ne!(seed & 0b11, 0b11);
        if seed & 0b11 == 0b00 {
            if result < *n {
                return false
            }
            result -= *n;
        } else if seed & 0b11 == 0b01 {
            if result % n > 0 {
                return false
            }
            result /= *n;
        } else if seed & 0b11 == 0b10 {
            let end_digit_exponent =  10_u64.pow(*strlen);
            if result % end_digit_exponent != *n {
                return false
            }
            result /= end_digit_exponent;
        }
        seed >>= 2;
    }
    result == numbers.first().unwrap().0
}

fn execute(numbers: &Vec<(u64, u32)>, mut seed: u64) -> u64 {
    let mut numbers = numbers.iter();
    let (mut result, _strlen) = *numbers.next().unwrap();
    for (n, _strlen) in numbers {
        if seed & 1 == 0 {
            result += *n;
        } else {
            result *= *n;
        }
        seed >>= 1;
    }
    result
}
