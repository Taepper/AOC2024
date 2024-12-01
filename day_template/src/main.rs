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
    (42, 84)
}
