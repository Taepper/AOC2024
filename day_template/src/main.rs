use tae_aoclib2025::{solve_all_inputs};

fn main() {
    solve_all_inputs("day_XX", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;
    let mut result1 = 0;
    let mut result2 = 0;
    (format!("{}", result1), format!("{}", result2))
}
