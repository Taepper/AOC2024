use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_07", do_task)
}

fn do_task(input: &String) -> (String, String) {
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
        if any_matches(&numbers, goal, debug_print) {
            result2 += goal as u128;
        }
    }
    (format!("{result1}"), format!("{result2}"))
}

fn any_matches(numbers: &Vec<(u64, u32)>, goal: u64, debug_print: bool) -> bool {
    if debug_print {
        println!("Checking numbers: {:?}", numbers);
    }
    let mut possible_result_after_tries = vec![goal];
    for (number, strlen) in numbers.iter().skip(1).rev() {
        let mut new_possible_result_after_tries = Vec::new();
        for result in possible_result_after_tries {
            // Was it an add?
            if result >= *number {
                if debug_print {
                    println!(
                        "{result} could have been formed by _{}_ + {number}",
                        result - *number
                    );
                }
                new_possible_result_after_tries.push(result - *number);
            }

            // Was it an mul?
            if result % *number == 0 {
                if debug_print {
                    println!(
                        "{result} could have been formed by _{}_ * {number}",
                        result / *number
                    );
                }
                new_possible_result_after_tries.push(result / *number);
            }

            // Was it a concat?
            let end_digit_exponent = 10_u64.pow(*strlen);
            if result % end_digit_exponent == *number {
                if debug_print {
                    println!(
                        "{result} could have been formed by _{}_ || {number}",
                        result / end_digit_exponent
                    );
                }
                new_possible_result_after_tries.push(result / end_digit_exponent);
            }
        }
        possible_result_after_tries = new_possible_result_after_tries;
    }
    for result in possible_result_after_tries {
        if result == numbers.first().unwrap().0 {
            if debug_print {
                println!("Found number {result} in final candidates!");
            }
            return true;
        }
    }
    false
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
