use std::collections::HashSet;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_06", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";

    let mut board = parse_input(input);
    let mut history: HashSet<Position> = HashSet::new();

    let mut all_visited_squares: HashSet<(usize, usize)> = HashSet::new();
    let mut potential_new_obstacles = HashSet::new();

    let mut position = board.start_position;
    history.insert(position);
    all_visited_squares.insert((position.x, position.y));

    loop {
        if debug_print {
            print_state(&board, &history);
        }
        match step(&board, &position, debug_print) {
            None => {
                break;
            }
            Some(new_position) => {
                position = new_position;
            }
        }
        if history.contains(&position) {
            panic!("Found a loop, when we should run off the board.")
        }
        history.insert(position);
        all_visited_squares.insert((position.x, position.y));

        if let Some(potential_obstacle) = get_next_square(&board, &position, debug_print){
            if !board.obstacles.contains(&potential_obstacle)
                && !potential_new_obstacles.contains(&potential_obstacle)
                && !all_visited_squares.contains(&potential_obstacle) {
                board.obstacles.insert(potential_obstacle);
                if is_looping(&board, position, &history, debug_print) {
                    potential_new_obstacles.insert(potential_obstacle);
                    if debug_print {
                        print_state(&board, &history);
                    }
                }
                board.obstacles.remove(&potential_obstacle);
            }
        }
    }
    let result1 = all_visited_squares.len();
    let result2 = potential_new_obstacles.len();

    (result1 as i64, result2 as i64)
}

fn is_looping(board: &Board, mut position: Position, original_history: &HashSet<Position>, debug_print: bool) -> bool {
    let mut local_history: HashSet<Position> = HashSet::new();
    loop{
        match step(board, &position, debug_print) {
            None => {
                return false;
            }
            Some(new_position) => {
                position = new_position;
                if original_history.contains(&position) {
                    if debug_print {
                        println!("Loop!");
                    }
                    return true;
                }
                if local_history.contains(&position) {
                    if debug_print {
                        println!("Loop2!");
                    }
                    return true;
                }
                local_history.insert(position);
            }
        }
    }
}

fn step(board: &Board, position: &Position, debug_print: bool) -> Option<Position> {
    let next_square = get_next_square(board, position, debug_print);
    if let Some(next_square) = next_square {
        if board.obstacles.contains(&next_square) {
            Some(Position {
                x: position.x,
                y: position.y,
                direction: turn(position.direction),
            })
        } else {
            Some(Position {
                x: next_square.0,
                y: next_square.1,
                direction: position.direction,
            })
        }
    } else {
        None
    }
}

fn get_next_square(
    board: &Board,
    position: &Position,
    debug_print: bool,
) -> Option<(usize, usize)> {
    match position.direction {
        Direction::LEFT => {
            if position.y == 0 {
                if debug_print {
                    println!("Moved off the board - left.")
                }
                return None;
            }
            Some((position.x, position.y - 1))
        }
        Direction::UP => {
            if position.x == 0 {
                if debug_print {
                    println!("Moved off the board - up.")
                }
                return None;
            }
            Some((position.x - 1, position.y))
        }
        Direction::RIGHT => {
            if position.y == board.width - 1 {
                if debug_print {
                    println!("Moved off the board - right.")
                }
                return None;
            }
            Some((position.x, position.y + 1))
        }
        Direction::DOWN => {
            if position.x == board.height - 1 {
                if debug_print {
                    println!("Moved off the board - down.")
                }
                return None;
            }
            Some((position.x + 1, position.y))
        }
    }
}

fn parse_input(input: &String) -> Board {
    let width: usize = input.lines().next().unwrap().len();
    let height: usize = input.lines().count();
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut start_position: Position = Position {
        x: 0,
        y: 0,
        direction: Direction::UP,
    };
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert((x, y));
            } else if c == '^' {
                start_position = Position {
                    x,
                    y,
                    direction: Direction::UP,
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

fn print_state(board: &Board, history: &HashSet<Position>) {
    let mut result = Vec::new();
    result.resize(board.height, vec!['.'; board.width]);
    for (x, y) in board.obstacles.iter() {
        result[*x][*y] = '#';
    }
    result[board.start_position.x][board.start_position.y] = '^';

    for pos in history.iter() {
        let char = match pos.direction {
            Direction::LEFT => '<',
            Direction::UP => '^',
            Direction::RIGHT => '>',
            Direction::DOWN => 'v',
        };
        result[pos.x][pos.y] = char;
    }

    for line in result {
        println!("{}", String::from(line.iter().collect::<String>()));
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

fn turn(dir: Direction) -> Direction {
    match dir {
        Direction::LEFT => Direction::UP,
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

struct Board {
    width: usize,
    height: usize,
    obstacles: HashSet<(usize, usize)>,
    start_position: Position,
}
