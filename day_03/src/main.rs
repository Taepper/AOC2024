use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    let input_dir: &Path = if Path::new("input").exists() {
        Path::new("input")
    } else {
        Path::new("day_03/input")
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
            "{:25} {:>25} in {}",
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

static DEBUG: bool = false;

enum Value {
    None,
    Expr(i64),
    Do,
    Dont,
}

fn do_task(input: &String) -> (i64, i64) {
    let mut result1 = 0;
    let mut result2 = 0;
    let mut cursor = 0;
    let mut enabled = true;
    while cursor < input.len() {
        let (advanced_cursor, result) = try_parse(cursor, &input);
        match result {
            Value::None => {
                cursor += 1;
            }
            Value::Expr(num) => {
                cursor = advanced_cursor;
                result1 += num;
                if enabled {
                    result2 += num;
                }
            }
            Value::Do => {
                cursor = advanced_cursor;
                enabled = true;
            }
            Value::Dont => {
                cursor = advanced_cursor;
                enabled = false;
            }
        }
    }

    (result1, result2)
}

fn print(text: &str, cursor: usize, start_cursor: usize, extra: &str) {
    if DEBUG {
        println!(
            "{0}\n{1:>2$}{3:->4$} {5}",
            text,
            ":",
            start_cursor,
            ":",
            cursor - start_cursor,
            extra
        );
    }
}

fn try_parse(mut cursor: usize, text: &String) -> (usize, Value) {
    if text[cursor..].starts_with("don't()") {
        print(text, cursor + 7, cursor, "Success don't()");
        return (cursor + 7, Value::Dont);
    }
    if text[cursor..].starts_with("do()") {
        print(text, cursor + 4, cursor, "Success do()");
        return (cursor + 4, Value::Do);
    }

    let start_cursor = cursor;
    if !text[cursor..].starts_with("mul(") {
        print(text, cursor, start_cursor, "Err mul(");
        return (start_cursor, Value::None);
    }
    cursor += 4;
    let (mut cursor, num1) = parse_number_up_to_3(cursor, &text);
    if num1.is_none() {
        print(text, cursor, start_cursor, "Err num1");
        return (start_cursor, Value::None);
    }
    if !text[cursor..].starts_with(",") {
        print(text, cursor, start_cursor, "Err ,");
        return (start_cursor, Value::None);
    }
    cursor += 1;
    let (mut cursor, num2) = parse_number_up_to_3(cursor, &text);
    if num2.is_none() {
        print(text, cursor, start_cursor, "Err num2");
        return (start_cursor, Value::None);
    }
    if !text[cursor..].starts_with(")") {
        print(text, cursor, start_cursor, "Err )");
        return (start_cursor, Value::None);
    }
    cursor += 1;
    print(text, cursor, start_cursor, "Success");
    (cursor, Value::Expr(num1.unwrap() * num2.unwrap()))
}

fn parse_number_up_to_3(cursor: usize, text: &str) -> (usize, Option<i64>) {
    if let Ok(x) = text[cursor..(cursor + 3)].parse::<i64>() {
        return (cursor + 3, Some(x));
    }
    if let Ok(x) = text[cursor..(cursor + 2)].parse::<i64>() {
        return (cursor + 2, Some(x));
    }
    if let Ok(x) = text[cursor..(cursor + 1)].parse::<i64>() {
        return (cursor + 1, Some(x));
    }
    (cursor + 3, None)
}
