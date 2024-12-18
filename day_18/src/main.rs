use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap};
use tae_aoclib2025::{solve_all_inputs, Coordinate};

fn main() {
    solve_all_inputs("day_18", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let bytes = parse_input(input);

    let dim =  if bytes.iter().map(|x| x.col).max().unwrap() > 6 {
        71
    } else { 7 };

    let mut board = vec![vec![false; dim]; dim];

    let first_n_bytes = if dim > 7 { 1024 } else { 12 };

    for i in 0..min(bytes.len(), first_n_bytes) {
        board[bytes[i].row][bytes[i].col] = true;
    }

    if debug_print {
        print_board(&board);
    }

    let start = Coordinate{col: 0, row: 0 };
    let goal = Coordinate{col: dim-1, row: dim-1 };

    let result1 = djikstra(start, goal, &board, debug_print);

    let mut result2 = 0;
    (format!("{}", result1), format!("{}", result2))
}

fn djikstra(start: Coordinate, goal: Coordinate, board: &Vec<Vec<bool>>, debug_print: bool) -> usize {
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
        for neighbor in neighbors(cur, dim, dim) {
            if !board[neighbor.row][neighbor.col] && !visited[neighbor.row][neighbor.col] {
                let neighbor_score = score + 1;
                if scores[neighbor.row][neighbor.col] == None || neighbor_score <  scores[neighbor.row][neighbor.col].unwrap() {
                    scores[neighbor.row][neighbor.col] = Some(neighbor_score);
                    queue.push((Reverse(neighbor_score), neighbor));
                }
            }
        }
        visited[cur.row][cur.col] = true;
    }

    if debug_print{
        print_scores(&scores);
    }

    scores[goal.row][goal.col].unwrap()
}

fn neighbors(pos: Coordinate, cols: usize, rows: usize) -> Vec<Coordinate> {
    let mut result = Vec::new();
    if pos.row > 0 {
        result.push(Coordinate{col: pos.col, row: pos.row - 1});
    }
    if pos.row < rows - 1 {
        result.push(Coordinate{col: pos.col, row: pos.row + 1});
    }
    if pos.col > 0 {
        result.push(Coordinate{col: pos.col - 1, row: pos.row});
    }
    if pos.col < cols - 1 {
        result.push(Coordinate{col: pos.col + 1, row: pos.row});
    }
    result
}

fn print_board(board: &Vec<Vec<bool>>) {
    println!("{}",
             board.iter().map(|row| row.iter().map(|x| if *x { "#" } else { "." }).collect::<Vec<&str>>().join("")).collect::<Vec<String>>().join("\n"))
}


fn print_scores(board: &Vec<Vec<Option<usize>>>) {
    println!("{}",
             board.iter().map(|row| row.iter().map(|x| if let Some(score) = *x { format!("{}", score % 10) } else {  "#".to_string() } ).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"))
}

fn parse_input(input: &String) -> Vec<Coordinate> {
    input.lines().map(|x| x.split(",")).map(|mut x| Coordinate{
        col: x.next().unwrap().parse::<usize>().unwrap(),
    row: x.next().unwrap().parse::<usize>().unwrap()}).collect()
}
