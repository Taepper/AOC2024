use std::cmp::min;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use tae_aoclib2025::{solve_all_inputs, step, Coordinate, Direction};

fn main() {
    solve_all_inputs("day_16", do_task)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct State {
    coordinate: Coordinate,
    direction: Direction,
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let map = parse_map(input.lines().collect());

    let mut scores = HashMap::new();
    let mut predecessors = HashMap::new();

    let mut queue = BinaryHeap::new();
    let start = State {
        coordinate: map.start,
        direction: Direction::Right,
    };
    queue.push(Reverse((0usize, start.clone())));
    scores.insert(start, 0usize);
    while let Some(Reverse((cur_score, cur))) = queue.pop() {
        if debug_print {
            println!("{cur:?}: score {cur_score}");
        }

        let mut moves = Vec::new();

        // Straight
        let new_pos = step(cur.coordinate, &cur.direction, 1);
        if map.obstacles[new_pos.row][new_pos.col] == false {
            let new_state = State {
                coordinate: new_pos,
                direction: cur.direction,
            };
            let new_score = cur_score + 1;
            moves.push((new_state, new_score));
        }

        // Left
        let new_dir = cur.direction.turn_left();
        let new_state = State {
            coordinate: cur.coordinate,
            direction: new_dir,
        };
        let new_score = cur_score + 1000;
        moves.push((new_state, new_score));

        // Right
        let new_dir = cur.direction.turn_right();
        let new_state = State {
            coordinate: cur.coordinate,
            direction: new_dir,
        };
        let new_score = cur_score + 1000;
        moves.push((new_state, new_score));

        for (new_state, new_score) in moves.into_iter() {
            if let Some(&score) = scores.get(&new_state) {
                if new_score < score {
                    queue.push(Reverse((new_score, new_state.clone())));
                    scores.insert(new_state.clone(), new_score);
                    predecessors.insert(new_state, cur.clone());
                }
            } else {
                queue.push(Reverse((new_score, new_state.clone())));
                scores.insert(new_state.clone(), new_score);
                predecessors.insert(new_state, cur.clone());
            }
        }
    }

    print_predecessor_path(
        State {
            coordinate: map.end,
            direction: Direction::Up,
        },
        &map,
        &predecessors,
    );

    println!(
        "{}",
        *scores
            .get(&State {
                coordinate: map.end,
                direction: Direction::Left
            })
            .unwrap()
    );
    println!(
        "{}",
        *scores
            .get(&State {
                coordinate: map.end,
                direction: Direction::Right
            })
            .unwrap()
    );
    println!(
        "{}",
        *scores
            .get(&State {
                coordinate: map.end,
                direction: Direction::Up
            })
            .unwrap()
    );
    println!(
        "{}",
        *scores
            .get(&State {
                coordinate: map.end,
                direction: Direction::Down
            })
            .unwrap()
    );

    let result1 = min(
        min(
            *scores
                .get(&State {
                    coordinate: map.end,
                    direction: Direction::Left,
                })
                .unwrap(),
            *scores
                .get(&State {
                    coordinate: map.end,
                    direction: Direction::Right,
                })
                .unwrap(),
        ),
        min(
            *scores
                .get(&State {
                    coordinate: map.end,
                    direction: Direction::Up,
                })
                .unwrap(),
            *scores
                .get(&State {
                    coordinate: map.end,
                    direction: Direction::Down,
                })
                .unwrap(),
        ),
    );
    let mut result2 = 0;

    (result1 as i64, result2)
}

fn print_predecessor_path(mut pos: State, map: &Map, predecessors: &HashMap<State, State>) {
    let mut map_chars = map
        .obstacles
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| if *x { '#' } else { '.' })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    while let Some(new_pos) = predecessors.get(&pos) {
        pos = new_pos.clone();
        map_chars[pos.coordinate.row][pos.coordinate.col] =
            new_pos.direction.to_string().chars().next().unwrap();
    }

    println!(
        "{}",
        map_chars
            .iter()
            .map(|x| x
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
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
