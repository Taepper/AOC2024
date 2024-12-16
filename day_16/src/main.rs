use std::cmp::min;
use std::collections::HashMap;
use tae_aoclib2025::{solve_all_inputs, step, Coordinate, Direction};

fn main() {
    solve_all_inputs("day_16", do_task)
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let map = parse_map(input.lines().collect());

    let mut scores = HashMap::new();
    let mut predecessors = HashMap::new();

    let mut stack = Vec::new();
    stack.push((map.start, Direction::Left));
    scores.insert((map.start, Direction::Left), 0usize);
    while let Some((cur_pos, cur_dir)) = stack.pop() {
        let cur_score = scores[&(cur_pos, cur_dir)];
        if debug_print {
            println!("{cur_pos}: score {cur_score}");
        }

        let mut moves = Vec::new();

        // Straight
        let new_pos = step(cur_pos, &cur_dir, 1);
        if map.obstacles[new_pos.row][new_pos.col] == false {
            let new_dir = cur_dir;
            let new_score = cur_score + 1;
            moves.push((new_pos, new_dir, new_score));
        }

        // Left
        let new_pos = cur_pos;
        let new_dir = cur_dir.turn_left();
        let new_score = cur_score + 1000;
        moves.push((new_pos, new_dir, new_score));

        // Right
        let new_pos = cur_pos;
        let new_dir = cur_dir.turn_right();
        let new_score = cur_score + 1000;
        moves.push((new_pos, new_dir, new_score));

        for (new_pos, new_dir, new_score) in moves.into_iter() {
            if let Some(&score) = scores.get(&(new_pos, new_dir)) {
                if new_score < score {
                    stack.push((new_pos, new_dir));
                    scores.insert((new_pos, new_dir), new_score);
                    predecessors.insert(new_pos, (cur_pos, cur_dir));
                }
            } else {
                stack.push((new_pos, new_dir));
                scores.insert((new_pos, new_dir), new_score);
                predecessors.insert(new_pos, (cur_pos, cur_dir));
            }
        }
    }

    let result1 = min(
        min(
            *scores.get(&(map.end, Direction::Left)).unwrap(),
            *scores.get(&(map.end, Direction::Right)).unwrap(),
        ),
        min(
            *scores.get(&(map.end, Direction::Up)).unwrap(),
            *scores.get(&(map.end, Direction::Down)).unwrap(),
        ),
    );
    let mut result2 = 0;

    (result1 as i64, result2)
}

struct Map {
    start: Coordinate,
    end: Coordinate,
    obstacles: Vec<Vec<bool>>,
}

fn parse_map(lines: Vec<&str>) -> Map {
    let rows = lines.len();
    let cols = lines.iter().next().unwrap().len();
    let mut obstacles = vec![vec![false; cols]; rows];
    let mut start = Coordinate { col: 0, row: 0 };
    let mut end = Coordinate { col: 0, row: 0 };
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles[row][col] = true;
            } else if char == 'S' {
                start = Coordinate { col, row };
            } else if char == 'E' {
                end = Coordinate { col, row };
            }
        }
    }
    Map {
        obstacles,
        start,
        end,
    }
}
