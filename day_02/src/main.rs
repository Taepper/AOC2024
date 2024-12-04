use tae_aoclib2025::{solve_all_inputs};

fn main() {
    solve_all_inputs("day_02", do_task);
}

fn do_task(input: &String) -> (i64, i64) {
    let mut result1 = 0;
    let mut result2 = 0;
    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if line_is_safe(&levels) {
            result1 += 1;
        }
        if has_at_most_one_unsafe(&levels) {
            result2 += 1;
        }
    }
    (result1, result2)
}

fn line_is_safe(levels: &Vec<i64>) -> bool {
    let first = levels[0];
    let second = levels[1];
    let decreasing = if first < second { false } else { true };
    for (level, next_level) in levels.iter().zip(levels.iter().skip(1)) {
        if is_level_unsafe(level, next_level, decreasing) {
            return false;
        }
    }
    true
}

fn is_level_unsafe(level: &i64, next_level: &i64, decreasing: bool) -> bool {
    if level == next_level {
        return true;
    }
    if decreasing {
        if level < next_level {
            return true;
        }
        if level - next_level > 3 {
            return true;
        }
        false
    } else {
        if level > next_level {
            return true;
        }
        if next_level - level > 3 {
            return true;
        }
        false
    }
}

fn has_at_most_one_unsafe(levels: &Vec<i64>) -> bool {
    for x in 0..levels.len() {
        let mut levels_removed = levels.clone();
        levels_removed.remove(x);
        if line_is_safe(&levels_removed) {
            return true;
        }
    }
    false
}
