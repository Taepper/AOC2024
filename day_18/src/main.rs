use std::cmp::Reverse;
use std::collections::BinaryHeap;
use tae_aoclib2025::{solve_all_inputs, Coordinate};

fn main() {
    solve_all_inputs("day_18", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let bytes = parse_input(input);

    let dim = if bytes.iter().map(|x| x.col).max().unwrap() > 6 {
        71
    } else {
        7
    };

    let mut board = vec![vec![false; dim]; dim];

    let first_n_bytes = if dim > 7 { 1024 } else { 12 };

    assert!(first_n_bytes < bytes.len());

    for i in 0..first_n_bytes {
        board[bytes[i].row][bytes[i].col] = true;
    }

    if debug_print {
        print_board(&board);
    }

    let start = Coordinate { col: 0, row: 0 };
    let goal = Coordinate {
        col: dim - 1,
        row: dim - 1,
    };

    let result1 = djikstra(start, goal, &board, debug_print).unwrap();

    if debug_print {
        println!("Part 2")
    }
    let mut board = vec![vec![false; dim]; dim];
    let bytes = deduplicate(bytes, dim);

    let result2: Option<Coordinate> = bin_search(
        &mut board,
        &bytes,
        0,
        bytes.len() - 1,
        start,
        goal,
        debug_print,
    );

    (format!("{}", result1), format!("{}", result2.unwrap()))
}

fn deduplicate(bytes: Vec<Coordinate>, dim: usize) -> Vec<Coordinate> {
    let mut visited = vec![vec![false; dim]; dim];
    let mut result = Vec::new();
    for byte in bytes {
        if visited[byte.row][byte.col] {
            continue;
        }
        visited[byte.row][byte.col] = true;
        result.push(byte);
    }
    result
}

fn bin_search(
    board: &mut Vec<Vec<bool>>,
    bytes: &Vec<Coordinate>,
    lower: usize,
    upper: usize,
    start: Coordinate,
    goal: Coordinate,
    debug_print: bool,
) -> Option<Coordinate> {
    let middle = (lower + upper) / 2;
    if lower == upper {
        if debug_print {
            println!("Found result at index {middle}")
        }
        return Some(bytes[middle])
    }
    for i in lower..=middle {
        board[bytes[i].row][bytes[i].col] = true;
    }
    if debug_print {
        println!();
        println!("Board after {middle} steps. Current range is [{lower},{upper}]");
        print_board(&board);
        println!("Performing djikstra search");
    }
    let result = djikstra(start, goal, &board, debug_print);
    if result.is_none() {
        for i in lower..=middle {
            board[bytes[i].row][bytes[i].col] = false;
        }
        bin_search(board, bytes, lower, middle, start, goal, debug_print)
    } else {
        bin_search(board, bytes, middle + 1, upper, start, goal, debug_print)
    }
}

fn djikstra(
    start: Coordinate,
    goal: Coordinate,
    board: &Vec<Vec<bool>>,
    debug_print: bool,
) -> Option<usize> {
    let dim = board.len();
    let mut queue: BinaryHeap<(Reverse<usize>, Coordinate)> = BinaryHeap::new();
    let mut visited = vec![vec![false; dim]; dim];
    let mut scores = vec![vec![None; dim]; dim];
    queue.push((Reverse(0), start));
    scores[start.row][start.col] = Some(0);

    while let Some((Reverse(score), cur)) = queue.pop() {
        if visited[cur.row][cur.col] {
            continue;
        }
        assert_eq!(scores[cur.row][cur.col].unwrap(), score);
        if cur == goal {
            if debug_print {
                print_scores(&scores, &board);
            }
            return Some(score);
        }
        for neighbor in neighbors(cur, dim, dim) {
            if !board[neighbor.row][neighbor.col] && !visited[neighbor.row][neighbor.col] {
                let neighbor_score = score + 1;
                if scores[neighbor.row][neighbor.col] == None
                    || neighbor_score < scores[neighbor.row][neighbor.col].unwrap()
                {
                    scores[neighbor.row][neighbor.col] = Some(neighbor_score);
                    queue.push((Reverse(neighbor_score), neighbor));
                }
            }
        }
        visited[cur.row][cur.col] = true;
    }
    if debug_print {
        print_scores(&scores, &board);
    }
    assert_eq!(scores[goal.row][goal.col], None);
    None
}

fn neighbors(pos: Coordinate, cols: usize, rows: usize) -> Vec<Coordinate> {
    let mut result = Vec::new();
    if pos.row > 0 {
        result.push(Coordinate {
            col: pos.col,
            row: pos.row - 1,
        });
    }
    if pos.row < rows - 1 {
        result.push(Coordinate {
            col: pos.col,
            row: pos.row + 1,
        });
    }
    if pos.col > 0 {
        result.push(Coordinate {
            col: pos.col - 1,
            row: pos.row,
        });
    }
    if pos.col < cols - 1 {
        result.push(Coordinate {
            col: pos.col + 1,
            row: pos.row,
        });
    }
    result
}

fn print_board(board: &Vec<Vec<bool>>) {
    println!(
        "{}",
        board
            .iter()
            .map(|row| row
                .iter()
                .map(|x| if *x { "#" } else { "." })
                .collect::<Vec<&str>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn print_scores(scores: &Vec<Vec<Option<usize>>>, board: &Vec<Vec<bool>>) {
    let mut output = vec![Vec::new(); board.len()];
    for (row, line) in scores.iter().enumerate() {
        for (col, score) in line.iter().enumerate() {
            if let Some(score) = score {
                output[row].push(format!("{}", score % 10));
            } else if board[row][col] {
                output[row].push("#".to_string());
            } else {
                output[row].push(".".to_string());
            }
        }
    }
    println!(
        "{}",
        output
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn parse_input(input: &String) -> Vec<Coordinate> {
    input
        .lines()
        .map(|x| x.split(","))
        .map(|mut x| Coordinate {
            col: x.next().unwrap().parse::<usize>().unwrap(),
            row: x.next().unwrap().parse::<usize>().unwrap(),
        })
        .collect()
}
