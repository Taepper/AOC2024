use std::collections::HashSet;
use tae_aoclib2025::{solve_all_inputs, Coordinate, Direction};

fn main() {
    solve_all_inputs("day_06", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";

    let mut board = parse_input(input);
    let mut history: Vec<Position> = Vec::new();

    let mut all_visited_squares: HashSet<Coordinate> = HashSet::new();
    let mut potential_new_obstacles = HashSet::new();

    let mut cur_position = board.start_position;
    all_visited_squares.insert(cur_position.coordinate);

    loop {
        if debug_print {
            print_state(&board, &history);
        }
        match step(&board, &cur_position, debug_print) {
            None => {
                break;
            }
            Some(new_position) => {
                cur_position = new_position;
            }
        }
        history.push(cur_position);
        all_visited_squares.insert(cur_position.coordinate);

        if let Some(potential_obstacle) = get_next_square(&board, &cur_position, debug_print) {
            if !board.obstacles[potential_obstacle.row][potential_obstacle.col]
                && !potential_new_obstacles.contains(&potential_obstacle)
                && !all_visited_squares.contains(&potential_obstacle)
            {
                board.obstacles[potential_obstacle.row][potential_obstacle.col] = true;
                if is_looping(&board, cur_position, debug_print) {
                    potential_new_obstacles.insert(potential_obstacle);
                    if debug_print {
                        print_state(&board, &history);
                    }
                }
                board.obstacles[potential_obstacle.row][potential_obstacle.col] = false;
            }
        }
    }
    let result1 = all_visited_squares.len();
    let result2 = potential_new_obstacles.len();

    (result1 as i64, result2 as i64)
}

fn is_looping(board: &Board, mut position: Position, debug_print: bool) -> bool {
    let mut loop_position = position.clone();
    let mut steps_for_new_position = 100;
    let mut last_steps_for_new_position = 100;
    loop {
        match step(board, &position, debug_print) {
            None => {
                return false;
            }
            Some(new_position) => {
                position = new_position;
                if position == loop_position {
                    if debug_print {
                        println!("Loop!2");
                    }
                    return true;
                }
                steps_for_new_position -= 1;
                if steps_for_new_position == 0 {
                    loop_position = position.clone();
                    steps_for_new_position = last_steps_for_new_position * 2;
                    last_steps_for_new_position = steps_for_new_position;
                }
            }
        }
    }
}

fn step(board: &Board, position: &Position, debug_print: bool) -> Option<Position> {
    let next_square = get_next_square(board, position, debug_print);
    if let Some(next_coord) = next_square {
        if board.obstacles[next_coord.row][next_coord.col] {
            Some(Position {
                coordinate: position.coordinate,
                direction: position.direction.turn_right(),
            })
        } else {
            Some(Position {
                coordinate: next_coord,
                direction: position.direction,
            })
        }
    } else {
        None
    }
}

fn get_next_square(board: &Board, position: &Position, debug_print: bool) -> Option<Coordinate> {
    match position.direction {
        Direction::Left => {
            if position.coordinate.col == 0 {
                if debug_print {
                    println!("Moved off the board - left.")
                }
                return None;
            }
            Some(Coordinate {
                row: position.coordinate.row,
                col: position.coordinate.col - 1,
            })
        }
        Direction::Up => {
            if position.coordinate.row == 0 {
                if debug_print {
                    println!("Moved off the board - up.")
                }
                return None;
            }
            Some(Coordinate {
                row: position.coordinate.row - 1,
                col: position.coordinate.col,
            })
        }
        Direction::Right => {
            if position.coordinate.col == board.width - 1 {
                if debug_print {
                    println!("Moved off the board - right.")
                }
                return None;
            }
            Some(Coordinate {
                row: position.coordinate.row,
                col: position.coordinate.col + 1,
            })
        }
        Direction::Down => {
            if position.coordinate.row == board.height - 1 {
                if debug_print {
                    println!("Moved off the board - down.")
                }
                return None;
            }
            Some(Coordinate {
                row: position.coordinate.row + 1,
                col: position.coordinate.col,
            })
        }
    }
}

fn parse_input(input: &String) -> Board {
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.lines().count();
    let mut obstacles: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut start_position: Position = Position {
        coordinate: Coordinate { row: 0, col: 0 },
        direction: Direction::Up,
    };
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles[row][col] = true;
            } else if c == '^' {
                start_position = Position {
                    coordinate: Coordinate { row, col },
                    direction: Direction::Up,
                };
            }
        }
    }
    Board {
        width,
        height,
        obstacles,
        start_position,
    }
}

fn print_state(board: &Board, history: &Vec<Position>) {
    let mut result = board
        .obstacles
        .iter()
        .map(|row| {
            row.iter()
                .map(|occupied| if *occupied { '#' } else { '.' })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    result[board.start_position.coordinate.row][board.start_position.coordinate.col] = '^';

    for pos in history.iter() {
        result[pos.coordinate.row][pos.coordinate.col] = pos.direction.into();
    }

    for line in result {
        println!("{}", String::from(line.iter().collect::<String>()));
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    coordinate: Coordinate,
    direction: Direction,
}

struct Board {
    width: usize,
    height: usize,
    obstacles: Vec<Vec<bool>>,
    start_position: Position,
}
