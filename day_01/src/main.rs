use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let input_dir: &Path = if Path::new("input").exists() {
        Path::new("input")
    } else {
        Path::new("day_01/input")
    };
    let mut inputs: Vec<(String, PathBuf)> = Vec::new();
    fs::read_dir(input_dir)
        .unwrap()
        .map(|x| x.unwrap().path())
        .map(|x| (fs::read_to_string(&x).unwrap(), x))
        .filter(|(file_content, name)| !file_content.is_empty())
        .for_each(|x| inputs.push(x));
    for (file_contents, test_name) in inputs.iter() {
        println!("{}: {:?}", test_name.to_str().unwrap(), do_task(file_contents));
    }
}

fn do_task(input: &String) -> (i64, i64) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    let mut right_numbers_count: HashMap<i64, i64> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let left = split.next().unwrap().parse::<i64>().unwrap();
        let right = split.next().unwrap().parse::<i64>().unwrap();
        left_numbers.push(left);
        right_numbers.push(right);

        match right_numbers_count.get_mut(&right) {
            None => { right_numbers_count.insert(right, 1); }
            Some(count) => { *count += 1 }
        }
    }
    left_numbers.sort();
    right_numbers.sort();
    let mut result1 = 0;
    let mut result2 = 0;
    for (left, right) in left_numbers.iter().zip(right_numbers.iter()) {
        result1 += if left > right { left - right } else { right - left };
        if let Some (right_count) = right_numbers_count.get(left) {
            result2 += left * right_count;
        }
    }

    (result1, result2)
}

