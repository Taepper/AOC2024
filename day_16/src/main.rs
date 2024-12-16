use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
    let end_states = vec![
        State {
            coordinate: map.end,
            direction: Direction::Left,
        },
        State {
            coordinate: map.end,
            direction: Direction::Right,
        },
        State {
            coordinate: map.end,
            direction: Direction::Up,
        },
        State {
            coordinate: map.end,
            direction: Direction::Down,
        },
    ];
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
                    predecessors.insert(new_state, vec![cur.clone()]);
                } else if new_score == score {
                    predecessors.get_mut(&new_state).unwrap().push(cur.clone());
                }
            } else {
                queue.push(Reverse((new_score, new_state.clone())));
                scores.insert(new_state.clone(), new_score);
                predecessors.insert(new_state, vec![cur.clone()]);
            }
        }
    }

    if debug_print {
        print_one_predecessor_path(
            State {
                coordinate: map.end,
                direction: Direction::Up,
            },
            &map,
            &predecessors,
        );
    }

    let result2 = count_all_predecessor_paths(
        State {
            coordinate: map.end,
            direction: Direction::Up,
        },
        &map,
        &predecessors,
        debug_print,
    );

    let result1 = *end_states
        .iter()
        .map(|x| scores.get(x).unwrap())
        .min()
        .unwrap();

    (result1 as i64, result2 as i64)
}

fn print_one_predecessor_path(
    mut pos: State,
    map: &Map,
    predecessors: &HashMap<State, Vec<State>>,
) {
    let mut map_chars = init_char_map(map);

    while let Some(new_pos) = predecessors.get(&pos) {
        pos = new_pos.first().unwrap().clone();
        map_chars[pos.coordinate.row][pos.coordinate.col] =
            pos.direction.to_string().chars().next().unwrap();
    }

    print_char_map(&map_chars);
}

fn count_all_predecessor_paths(
    pos: State,
    map: &Map,
    predecessors: &HashMap<State, Vec<State>>,
    debug_print: bool,
) -> usize {
    let mut visited_predecessors = HashSet::new();
    let mut queue = vec![pos];
    while let Some(pos) = queue.pop() {
        if !visited_predecessors.contains(&pos) {
            if let Some(new_positions) = predecessors.get(&pos) {
                for new_pos in new_positions {
                    if !visited_predecessors.contains(new_pos) {
                        queue.push(new_pos.clone());
                    }
                }
            }
            visited_predecessors.insert(pos);
        }
    }

    if debug_print {
        let mut map_chars = init_char_map(map);
        for state in &visited_predecessors {
            map_chars[state.coordinate.row][state.coordinate.col] = 'O';
        }
        print_char_map(&map_chars);
    }

    let mut visited_coordinates = HashSet::new();
    for x in visited_predecessors {
        visited_coordinates.insert(x.coordinate);
    }
    visited_coordinates.len()
}

fn init_char_map(map: &Map) -> Vec<Vec<char>> {
    map.obstacles
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| if *x { '#' } else { '.' })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn print_char_map(char_map: &Vec<Vec<char>>) {
    println!(
        "{}",
        char_map
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
