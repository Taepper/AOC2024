use std::fmt::{Display, Formatter};
use tae_aoclib2025::{solve_all_inputs, Coordinate};

fn main() {
    solve_all_inputs("day_15", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;
    let mut result1 = 0;
    let mut result2 = 0;

    let (start_map, movements) = parse_input(input);
    println!("Initial state:");
    print_map(&start_map);

    let mut map = start_map.clone();
    for movement in movements {
        map = next(map, movement);
    }

    (result1, result2)
}

fn next(map: Vec<Vec<Object>>, direction: Direction) -> Vec<Vec<Object>> {
    println!("Move {direction}:");

    todo!()
}

fn print_map(map: &Vec<Vec<Object>>) {
    let string = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|o| format!("{o}"))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", string);
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::UP => '^',
                Direction::DOWN => 'v',
                Direction::LEFT => '<',
                Direction::RIGHT => '>',
            },
        )
    }
}

#[derive(Debug, Clone)]
enum Object {
    ROBOT,
    BOX,
    WALL,
    EMPTY,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::ROBOT => '@',
                Object::BOX => 'O',
                Object::WALL => '#',
                Object::EMPTY => '.',
            },
        )
    }
}

fn parse_input(input: &String) -> (Vec<Vec<Object>>, Vec<Direction>) {
    let mut lines1: Vec<&str> = Vec::new();
    let mut lines2: Vec<&str> = Vec::new();
    let mut first = true;
    for line in input.lines() {
        if line.is_empty() {
            first = false
        } else if first {
            lines1.push(line);
        } else {
            lines2.push(line);
        }
    }
    (parse_map(lines1), parse_movements(lines2))
}

fn parse_map(lines: Vec<&str>) -> Vec<Vec<Object>> {
    let rows = lines.len();
    let cols = lines.iter().next().unwrap().len();
    let mut board = vec![vec![Object::EMPTY; cols]; rows];
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                board[row][col] = Object::WALL;
            } else if char == '@' {
                board[row][col] = Object::ROBOT;
            } else if char == 'O' {
                board[row][col] = Object::BOX;
            }
        }
    }
    board
}

fn parse_movements(lines: Vec<&str>) -> Vec<Direction> {
    let mut movements: Vec<Direction> = Vec::new();
    for line in lines {
        for char in line.chars() {
            let direction = if char == '^' {
                Direction::UP
            } else if char == '>' {
                Direction::RIGHT
            } else if char == '<' {
                Direction::LEFT
            } else if char == 'v' {
                Direction::DOWN
            } else {
                unreachable!()
            };
            movements.push(direction);
        }
    }
    movements
}
