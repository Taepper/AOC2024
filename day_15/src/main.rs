use std::fmt::{Display, Formatter};
use tae_aoclib2025::{solve_all_inputs, step, Coordinate, Direction};

fn main() {
    solve_all_inputs("day_15", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let (start_map, movements) = parse_input(input);
    if debug_print {
        println!("Initial state:");
        print_map(&start_map.map);
    }

    let mut map = start_map.clone();
    for movement in &movements {
        map = next(map, movement);
        if debug_print {
            println!("Move {movement}:");
            print_map(&map.map);
        }
    }

    let mut wide_map = start_map.widen();
    if debug_print {
        println!("W Initial state:");
        print_wide_map(&wide_map.map);
    }
    for movement in &movements {
        wide_map = next_wide(wide_map, movement);
        if debug_print {
            println!("W Move {movement}:");
            print_wide_map(&wide_map.map);
        }
    }

    let mut result1 = 0;
    for (row, line) in map.map.iter().enumerate() {
        for (col, object) in line.iter().enumerate() {
            if *object == Object::Box {
                result1 += row * 100 + col;
            }
        }
    }

    let mut result2 = 0;
    for (row, line) in wide_map.map.iter().enumerate() {
        for (col, object) in line.iter().enumerate() {
            if *object == WideObject::BoxLeft {
                result2 += row * 100 + col;
            }
        }
    }

    (format!("{result1}"), format!("{result2}"))
}

fn next_wide(state: WideMapState, direction: &Direction) -> WideMapState {
    if *direction == Direction::Left || *direction == Direction::Right {
        next_wide_simple(state, direction)
    } else {
        next_wide_up_down(state, direction)
    }
}

fn next_wide_up_down(mut state: WideMapState, direction: &Direction) -> WideMapState {
    assert_eq!(
        state.map[state.robot_position.row][state.robot_position.col],
        WideObject::Robot
    );

    let new_robot_pos = step(state.robot_position, direction, 1);

    let mut push_starts = vec![state.robot_position];
    let mut all_pushes = Vec::new();
    while let Some(push_start) = push_starts.pop() {
        match get_push_target(&push_start, direction, &state.map) {
            None => {
                return state;
            }
            Some((push_length, new_push_starts)) => {
                all_pushes.push((push_start, push_length));
                push_starts.extend(new_push_starts);
            }
        }
    }
    let mut pushes_by_cols = vec![Vec::new(); state.map[0].len()];
    for (push_start, push_length) in all_pushes {
        pushes_by_cols[push_start.col].push((push_start, push_length));
    }
    for pushes in pushes_by_cols {
        for (push_start, push_length) in deduplicate(pushes, direction) {
            for i in (0..push_length).rev() {
                let copy_from = step(push_start, direction, i);
                let copy_to = step(push_start, direction, i + 1);
                state.map[copy_to.row][copy_to.col] =
                    state.map[copy_from.row][copy_from.col].clone();
            }
            state.map[push_start.row][push_start.col] = WideObject::Empty;
        }
    }
    state.robot_position = new_robot_pos;
    state
}

fn deduplicate(
    mut pushes: Vec<(Coordinate, usize)>,
    direction: &Direction,
) -> Vec<(Coordinate, usize)> {
    if *direction == Direction::Up {
        pushes.sort_by_key(|a| a.0.row);
        pushes.reverse();
        // println!("pushes: {:?}", pushes);
        for (push1, (push_2_index, push_2)) in pushes.iter().zip(pushes.iter().enumerate().skip(1))
        {
            if push1.0.row == push_2.0.row {
                pushes.remove(push_2_index);
                return deduplicate(pushes, direction);
            }
            if push1.0.row - push1.1 <= push_2.0.row {
                pushes.remove(push_2_index);
                return deduplicate(pushes, direction);
            }
        }
        pushes.reverse();
        pushes
    } else {
        // Down
        pushes.sort_by_key(|a| a.0.row);
        for (push1, (push_2_index, push_2)) in pushes.iter().zip(pushes.iter().enumerate().skip(1))
        {
            if push1.0.row == push_2.0.row {
                pushes.remove(push_2_index);
                return deduplicate(pushes, direction);
            }
            if push1.0.row + push1.1 >= push_2.0.row {
                pushes.remove(push_2_index);
                return deduplicate(pushes, direction);
            }
        }
        pushes.reverse();
        pushes
    }
}

fn next_wide_simple(mut state: WideMapState, direction: &Direction) -> WideMapState {
    let row = state.robot_position.row;
    let col = state.robot_position.col;
    assert_eq!(state.map[row][col], WideObject::Robot);

    let new_robot_pos = step(state.robot_position, direction, 1);

    if let Some((push_length, _)) = get_push_target(&state.robot_position, direction, &state.map) {
        for i in (0..push_length).rev() {
            let copy_from = step(state.robot_position, direction, i);
            let copy_to = step(state.robot_position, direction, i + 1);
            state.map[copy_to.row][copy_to.col] = state.map[copy_from.row][copy_from.col].clone();
        }
        state.map[row][col] = WideObject::Empty;
        state.robot_position = new_robot_pos;
        state
    } else {
        state
    }
}

fn get_push_target(
    push_start: &Coordinate,
    direction: &Direction,
    map: &Vec<Vec<WideObject>>,
) -> Option<(usize, Vec<Coordinate>)> {
    let mut push_length = 1;
    let mut new_push_starts = Vec::new();
    loop {
        let push_pos = step(*push_start, direction, push_length);
        match map[push_pos.row][push_pos.col] {
            WideObject::Robot => {
                panic!("This should not happen? Multiple robots on map?2")
            }
            WideObject::BoxLeft => {
                new_push_starts.push(Coordinate {
                    col: push_pos.col + 1,
                    row: push_pos.row,
                });
            }
            WideObject::BoxRight => {
                new_push_starts.push(Coordinate {
                    col: push_pos.col - 1,
                    row: push_pos.row,
                });
            }
            WideObject::Wall => {
                return None;
            }
            WideObject::Empty => {
                return Some((push_length, new_push_starts));
            }
        }
        push_length += 1;
    }
}

fn next(mut state: MapState, direction: &Direction) -> MapState {
    let row = state.robot_position.row;
    let col = state.robot_position.col;
    assert_eq!(state.map[row][col], Object::Robot);

    let new_robot_pos = step(state.robot_position, direction, 1);

    let mut push_length = 1;
    loop {
        let push_pos = step(state.robot_position, direction, push_length);
        match state.map[push_pos.row][push_pos.col] {
            Object::Robot => {
                panic!("This should not happen? Multiple robots on map?2")
            }
            Object::Box => { // Nothing..
            }
            Object::Wall => return state,
            Object::Empty => {
                state.robot_position = new_robot_pos;
                state.map[row][col] = Object::Empty;
                state.map[new_robot_pos.row][new_robot_pos.col] = Object::Robot;
                if push_length > 1 {
                    state.map[push_pos.row][push_pos.col] = Object::Box;
                }
                return state;
            }
        }
        push_length += 1;
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

fn print_wide_map(map: &Vec<Vec<WideObject>>) {
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

#[derive(Debug, Clone)]
struct MapState {
    map: Vec<Vec<Object>>,
    robot_position: Coordinate,
}

impl MapState {
    pub(crate) fn widen(&self) -> WideMapState {
        let mut wide_map = vec![Vec::new(); self.map.len()];
        for (row, line) in self.map.iter().enumerate() {
            for o in line {
                match o {
                    Object::Robot => {
                        wide_map[row].push(WideObject::Robot);
                        wide_map[row].push(WideObject::Empty);
                    }
                    Object::Box => {
                        wide_map[row].push(WideObject::BoxLeft);
                        wide_map[row].push(WideObject::BoxRight);
                    }
                    Object::Wall => {
                        wide_map[row].push(WideObject::Wall);
                        wide_map[row].push(WideObject::Wall);
                    }
                    Object::Empty => {
                        wide_map[row].push(WideObject::Empty);
                        wide_map[row].push(WideObject::Empty);
                    }
                }
            }
        }
        let new_start = Coordinate {
            row: self.robot_position.row,
            col: self.robot_position.col * 2,
        };
        WideMapState {
            map: wide_map,
            robot_position: new_start,
        }
    }
}

#[derive(Debug, Clone)]
struct WideMapState {
    map: Vec<Vec<WideObject>>,
    robot_position: Coordinate,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Object {
    Robot,
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WideObject {
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
    Empty,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Robot => '@',
                Object::Box => 'O',
                Object::Wall => '#',
                Object::Empty => '.',
            },
        )
    }
}

impl Display for WideObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WideObject::Robot => '@',
                WideObject::BoxLeft => '[',
                WideObject::BoxRight => ']',
                WideObject::Wall => '#',
                WideObject::Empty => '.',
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
    let mut map = vec![vec![Object::Empty; cols]; rows];
    let mut robot_position = Coordinate { col: 0, row: 0 };
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                map[row][col] = Object::Wall;
            } else if char == '@' {
                map[row][col] = Object::Robot;
                robot_position = Coordinate { col, row };
            } else if char == 'O' {
                map[row][col] = Object::Box;
            }
        }
    }
    MapState {
        map,
        robot_position,
    }
}

fn parse_movements(lines: Vec<&str>) -> Vec<Direction> {
    let mut movements: Vec<Direction> = Vec::new();
    for line in lines {
        for char in line.chars() {
            let direction = if char == '^' {
                Direction::Up
            } else if char == '>' {
                Direction::Right
            } else if char == '<' {
                Direction::Left
            } else if char == 'v' {
                Direction::Down
            } else {
                unreachable!()
            };
            movements.push(direction);
        }
    }
    movements
}
