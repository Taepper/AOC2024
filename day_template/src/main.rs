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
    (result1, result2)
}
