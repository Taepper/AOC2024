use std::fmt::Display;
use std::ops::Add;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_14", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 1000;

    let robots = parse_input(input);

    let width: i64 = if robots.iter().map(|r| r.position.col).max().unwrap() < 11 {
        11
    } else {
        101
    };
    let height: i64 = if robots.iter().map(|r| r.position.row).max().unwrap() < 7 {
        7
    } else {
        103
    };

    let mut top_left = Vec::new();
    let mut top_right = Vec::new();
    let mut bot_left = Vec::new();
    let mut bot_right = Vec::new();

    let mut robots_at_position = vec![vec![0; width as usize]; height as usize];

    let steps = 100;
    for robot in &robots {
        println!("Robot {robot:?}");
        let mut pos = robot.position;
        for _ in 0..steps {
            pos = pos + robot.vel;
        }
        println!("{:?}", pos);
        pos.col = (pos.col + 101 * width) % width;
        println!("{:?}", pos);
        pos.row = (pos.row + 101 * height) % height;
        println!("{:?}", pos);
        robots_at_position[pos.row as usize][pos.col as usize] += 1;
        if debug_print {
            println!("will be at position {pos:?} after {steps} steps");
        }
        if pos.col < width / 2 && pos.row < height / 2 {
            if debug_print {
                println!("This is in quadrant top_left");
            }
            top_left.push(pos);
        } else if pos.col > width / 2 && pos.row < height / 2 {
            if debug_print {
                println!("This is in quadrant top_right");
            }
            top_right.push(pos);
        } else if pos.col < width / 2 && pos.row > height / 2 {
            if debug_print {
                println!("This is in quadrant bot_left");
            }
            bot_left.push(pos);
        } else if pos.col > width / 2 && pos.row > height / 2 {
            if debug_print {
                println!("This is in quadrant bot_right");
            }
            bot_right.push(pos);
        } else if pos.col == width / 2 && pos.row == height / 2 {
            if debug_print {
                println!("This is in no quadrant (dead-center)");
            }
        } else if pos.col == width / 2 {
            if debug_print {
                println!("This is in no quadrant (vertical-middle)");
            }
        } else if pos.row == height / 2 {
            if debug_print {
                println!("This is in no quadrant (horizontal-middle)");
            }
        } else {
            unreachable!()
        }
    }
    if debug_print {
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

    if debug_print {
        println!("Total in top_left: {}", top_left.len());
    }
    if debug_print {
        println!("Total in top_right: {}", top_right.len());
    }
    if debug_print {
        println!("Total in bot_left: {}", bot_left.len());
    }
    if debug_print {
        println!("Total in bot_right: {}", bot_right.len());
    }

    let result1 = top_left.len() * top_right.len() * bot_left.len() * bot_right.len();
    let result2 = 0;
    println!("Dim: {}, {}", width, height);
    (result1 as i64, result2 as i64)
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    col: i64,
    row: i64,
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

#[derive(Debug)]
struct Robot {
    position: Coordinate,
    vel: Coordinate,
}

fn parse_input(input: &String) -> Vec<Robot> {
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
            .collect::<Vec<i64>>();
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
                col: velocity[0],
                row: velocity[1],
            },
        });
    }
    result
}
