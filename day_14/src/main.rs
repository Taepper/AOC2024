use std::fmt::Display;
use std::ops::{Add, AddAssign, Rem, RemAssign};
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_14", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 1000;

    let (cols, rows, robots) = parse_input(input);

    let step_limit = 50000;
    let mut result1 = 0;
    let mut result2 = 0;

    let mut positions: Vec<Coordinate> = robots.iter().map(|robot| robot.position).collect();
    for step in 0..step_limit {
        for (position, robot) in positions.iter_mut().zip(&robots) {
            *position += robot.vel;
            *position %= Coordinate {
                col: cols,
                row: rows,
            };
        }
        if step == 99 {
            result1 = calculate_safety_number(&positions, cols, rows, debug_print);
        }
        if is_christmas_tree(&positions, cols, rows, step, debug_print) {
            result2 = step + 1;
            break;
        }
    }

    if debug_print {
        print_positions(&positions, cols, rows);
    }

    (result1 as i64, result2 as i64)
}

fn print_positions(positions: &Vec<Coordinate>, cols: usize, rows: usize) {
    let mut robots_at_position = vec![vec![0; cols]; rows];
    for pos in positions {
        robots_at_position[pos.row][pos.col] += 1;
    }
    println!(
        "All robots:\n{}",
        robots_at_position
            .iter()
            .map(|row| row
                .iter()
                .map(|c| format!("{c}"))
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn is_christmas_tree(
    positions: &Vec<Coordinate>,
    cols: usize,
    rows: usize,
    step: usize,
    debug_print: bool,
) -> bool {
    let mut row_cols: Vec<Vec<usize>> = vec![Vec::new(); rows];
    positions
        .iter()
        .for_each(|pos| row_cols[pos.row].push(pos.col));
    let mut horizontal_rows = 0;
    for mut col_ids in row_cols {
        col_ids.sort_unstable();
        for i in 3..col_ids.len() {
            if col_ids[i - 3] + 1 == col_ids[i - 2]
                && col_ids[i - 2] + 1 == col_ids[i - 1]
                && col_ids[i - 1] + 1 == col_ids[i]
            {
                horizontal_rows += 1;
            }
        }
    }
    if horizontal_rows >= 20 {
        if debug_print {
            println!("Step {}", step + 1);
            print_positions(positions, cols, rows);
        }
        true
    } else {
        false
    }
}

fn calculate_safety_number(
    positions: &Vec<Coordinate>,
    cols: usize,
    rows: usize,
    debug_print: bool,
) -> i64 {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bot_left = 0;
    let mut bot_right = 0;
    for pos in positions {
        if debug_print {
            println!("A robot will be at position {pos:?}");
        }
        if pos.col < cols / 2 && pos.row < rows / 2 {
            if debug_print {
                println!("This is in quadrant top_left");
            }
            top_left += 1;
        } else if pos.col > cols / 2 && pos.row < rows / 2 {
            if debug_print {
                println!("This is in quadrant top_right");
            }
            top_right += 1;
        } else if pos.col < cols / 2 && pos.row > rows / 2 {
            if debug_print {
                println!("This is in quadrant bot_left");
            }
            bot_left += 1;
        } else if pos.col > cols / 2 && pos.row > rows / 2 {
            if debug_print {
                println!("This is in quadrant bot_right");
            }
            bot_right += 1;
        }
    }
    top_left * top_right * bot_left * bot_right
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    col: usize,
    row: usize,
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

#[derive(Debug)]
struct Robot {
    position: Coordinate,
    vel: Coordinate,
}

fn parse_input(input: &String) -> (usize, usize, Vec<Robot>) {
    let cols = if input.lines().count() > 30 { 101 } else { 11 };
    let rows = if input.lines().count() > 30 { 103 } else { 7 };
    let mut result = Vec::new();
    for line in input.lines() {
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].starts_with("p="));
        parts[0] = parts[0].strip_prefix("p=").unwrap();
        assert!(parts[1].starts_with("v="));
        parts[1] = parts[1].strip_prefix("v=").unwrap();
        let position = parts[0]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(position.len(), 2);
        let velocity = parts[1]
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        assert_eq!(velocity.len(), 2);
        result.push(Robot {
            position: Coordinate {
                col: position[0],
                row: position[1],
            },
            vel: Coordinate {
                col: if velocity[0] >= 0 {
                    velocity[0] as usize
                } else {
                    (cols as i64 + velocity[0]) as usize
                },
                row: if velocity[1] >= 0 {
                    velocity[1] as usize
                } else {
                    (rows as i64 + velocity[1]) as usize
                },
            },
        });
    }
    (cols, rows, result)
}
