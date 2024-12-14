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

    let width = if robots.iter().map(|r| r.position.x).max().unwrap() < 11 {
        11
    } else {
        101
    };
    let height = if robots.iter().map(|r| r.position.y).max().unwrap() < 7 {
        7
    } else {
        103
    };

    let mut top_left = Vec::new();
    let mut top_right = Vec::new();
    let mut bot_left = Vec::new();
    let mut bot_right = Vec::new();

    for robot in &robots {
        let mut pos = robot.position;
        for _ in 0..100 {
            pos = pos + robot.vel;
        }
        pos.x %= width;
        pos.y %= height;
        if debug_print {
            println!("Robot {robot:?} will be at position {pos:?} after 100 steps");
        }
        if pos.x < width / 2 && pos.y < height / 2 {
            if debug_print {
                println!("This is in quadrant top_left");
            }
            top_left.push(pos);
        } else if pos.x > width / 2 && pos.y < height / 2 {
            if debug_print {
                println!("This is in quadrant top_right");
            }
            top_right.push(pos);
        } else if pos.x < width / 2 && pos.y > height / 2 {
            if debug_print {
                println!("This is in quadrant bot_left");
            }
            bot_left.push(pos);
        } else if pos.x > width / 2 && pos.y > height / 2 {
            if debug_print {
                println!("This is in quadrant bot_right");
            }
            bot_right.push(pos);
        } else {
            if debug_print {
                println!("This is in no quadrant");
            }
        }
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
    let mut result2 = 0;
    (result1 as i64, result2 as i64)
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
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
            .collect::<Vec<usize>>();
        assert_eq!(position.len(), 2);
        let velocity = parts[0]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(velocity.len(), 2);
        result.push(Robot {
            position: Coordinate {
                x: position[0],
                y: position[1],
            },
            vel: Coordinate {
                x: velocity[0],
                y: velocity[1],
            },
        });
    }
    result
}
