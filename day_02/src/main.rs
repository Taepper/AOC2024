use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    let input_dir: &Path = if Path::new("input").exists() {
        Path::new("input")
    } else {
        Path::new("day_02/input")
    };
    let mut inputs: Vec<(String, PathBuf)> = Vec::new();
    fs::read_dir(input_dir)
        .unwrap()
        .map(|x| x.unwrap().path())
        .map(|x| (fs::read_to_string(&x).unwrap(), x))
        .filter(|(file_content, _name)| !file_content.is_empty())
        .for_each(|x| inputs.push(x));
    for (file_contents, test_name) in inputs.iter() {
        let start = Instant::now();
        let result = do_task(file_contents);
        let duration = start.elapsed();
        let formatted_result = format!("{:?}", result);
        let formatted_duration = format_duration(duration);
        println!(
            "{}: {:>16} in {}",
            test_name.to_str().unwrap(),
            formatted_result,
            formatted_duration
        );
    }
}

fn format_duration(duration: Duration) -> String {
    let duration_secs = duration.as_secs();
    let duration_millis = duration.as_millis() % 1000;
    if duration_secs > 0 {
        return format!("{}.{:0>3} s", duration_secs, duration_millis);
    }
    let duration_micros = duration.as_micros() % 1000;
    if duration_millis > 0 {
        return format!("{}.{:0>3} ms", duration_millis, duration_micros);
    }
    let duration_nanos = duration.as_nanos() % 1000;
    format!("{}.{:0>3} micros", duration_micros, duration_nanos)
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
