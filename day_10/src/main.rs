use tae_aoclib2025::{solve_all_inputs};

fn main() {
    solve_all_inputs("day_10", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;
    let mut result1 = 0;
    let mut result2 = 0;
    (result1, result2)
}
