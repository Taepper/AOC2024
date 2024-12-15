use std::fmt::{Display, Formatter};
use tae_aoclib2025::{solve_all_inputs, Coordinate};

fn main() {
    solve_all_inputs("day_15", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;
    let mut result2 = 0;

    let (start_map, movements) = parse_input(input);
    if debug_print {
        println!("Initial state:");
        print_map(&start_map.map);
    }

    let mut map = start_map.clone();
    for movement in movements {
        map = next(map, movement);
        if debug_print {
            println!("Move {movement}:");
            print_map(&map.map);
        }
    }


    let mut result1 = 0;
    for (row, line) in map.map.iter().enumerate() {
        for (col, object) in line.iter().enumerate() {
            if *object == Object::BOX {
                result1 += row * 100 + col;
            }
        }
    }

    (result1 as i64, result2 as i64)
}

fn next(mut state: MapState, direction: Direction) -> MapState {
    let row = state.robot_position.row;
    let col = state.robot_position.col;
    assert_eq!(state.map[row][col], Object::ROBOT);

    let new_robot_pos = step(state.robot_position, direction, 1);

    let mut push_length = 1;
    loop {
        let push_pos = step(state.robot_position, direction, push_length);
        match state.map[push_pos.row][push_pos.col] {
            Object::ROBOT => {panic!("This should not happen? Multiple robots on map?2")}
            Object::BOX => {// Nothing..
            }
            Object::WALL => {
                return state
            }
            Object::EMPTY => {
                state.robot_position = new_robot_pos;
                state.map[row][col] = Object::EMPTY;
                state.map[new_robot_pos.row][new_robot_pos.col] = Object::ROBOT;
                if push_length > 1 {
                    state.map[push_pos.row][push_pos.col] = Object::BOX;
                }
                return state
            }
        }
        push_length += 1;
    }
}

fn step(coord: Coordinate, direction: Direction, steps: usize) -> Coordinate {
    let col = coord.col;
    let row = coord.row;
    match direction {
        Direction::UP => {Coordinate{col, row: row - steps}}
        Direction::DOWN => {Coordinate{col, row: row + steps}}
        Direction::LEFT => {Coordinate{col: col - steps, row}}
        Direction::RIGHT => {Coordinate{col: col + steps, row}}
    }
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
struct MapState {
    map: Vec<Vec<Object>>,
    robot_position: Coordinate,
}


#[derive(Debug, Clone, Eq, PartialEq)]
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

fn parse_input(input: &String) -> (MapState, Vec<Direction>) {
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

fn parse_map(lines: Vec<&str>) -> MapState {
    let rows = lines.len();
    let cols = lines.iter().next().unwrap().len();
    let mut map = vec![vec![Object::EMPTY; cols]; rows];
    let mut robot_position = Coordinate{col: 0, row: 0};
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                map[row][col] = Object::WALL;
            } else if char == '@' {
                map[row][col] = Object::ROBOT;
                robot_position = Coordinate{col, row};
            } else if char == 'O' {
                map[row][col] = Object::BOX;
            }
        }
    }
    MapState{map, robot_position}

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
