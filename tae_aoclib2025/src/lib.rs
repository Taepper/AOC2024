use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Add, AddAssign, Rem, RemAssign};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub fn solve_all_inputs(day: &str, do_task: fn(&String) -> (i64, i64)) {
    let input_dir: String = if Path::new("input").exists() {
        "input".to_string()
    } else {
        format!("{}/input", day)
    };
    let files = get_files_from_dir(Path::new(&input_dir));
    for (file_contents, test_name) in files {
        let start = Instant::now();
        let result = do_task(&file_contents);
        let duration = start.elapsed();
        let formatted_result = format!("( {} , {} )", result.0, result.1);
        let formatted_duration = format_duration(duration);
        println!(
            "{:25} {:>25} in {}",
            test_name.to_str().unwrap(),
            formatted_result,
            formatted_duration
        );
    }
}

pub fn get_files_from_dir(input_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut inputs: Vec<(String, PathBuf)> = Vec::new();
    fs::read_dir(input_dir)
        .unwrap()
        .map(|x| x.unwrap().path())
        .map(|x| (fs::read_to_string(&x).unwrap(), x))
        .filter(|(file_content, _name)| !file_content.is_empty())
        .for_each(|x| inputs.push(x));
    inputs
}

pub fn format_duration(duration: Duration) -> String {
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub col: usize,
    pub row: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.col, self.row)
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl RemAssign for Coordinate {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl Rem<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn rem(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            col: self.col % rhs.col,
            row: self.row % rhs.row,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            },
        )
    }
}

pub fn step(coord: Coordinate, direction: &Direction, steps: usize) -> Coordinate {
    let col = coord.col;
    let row = coord.row;
    match direction {
        Direction::Up => Coordinate {
            col,
            row: row - steps,
        },
        Direction::Down => Coordinate {
            col,
            row: row + steps,
        },
        Direction::Left => Coordinate {
            col: col - steps,
            row,
        },
        Direction::Right => Coordinate {
            col: col + steps,
            row,
        },
    }
}
