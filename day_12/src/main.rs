use std::collections::HashMap;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_12", do_task)
}

#[derive(Default, Debug)]
struct Region {
    character: char,
    area: usize,
    perimeter: usize,
    inside_corners: usize,
}

fn find(
    x: (usize, usize),
    parents: &mut HashMap<(usize, usize), (usize, usize)>,
) -> (usize, usize) {
    let mut cur = x;
    while cur != parents[&cur] {
        cur = parents[&cur];
    }
    cur
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 100;

    let board = parse_input(input);
    let rows = board.len();
    let cols = board[0].len();

    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut ranks: HashMap<(usize, usize), usize> = HashMap::new();
    let mut regions: HashMap<(usize, usize), Region> = HashMap::new();

    for row in 0..rows {
        for col in 0..cols {
            parents.insert((row, col), (row, col));
            ranks.insert((row, col), 0);
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            let character = board[row][col];
            // Up
            if row > 0 && board[row - 1][col] == character {
                union((row, col), (row - 1, col), &mut parents, &mut ranks);
            }
            // Left
            if col > 0 && board[row][col - 1] == character {
                union((row, col), (row, col - 1), &mut parents, &mut ranks);
            }
            // Down
            if row + 1 < rows && board[row + 1][col] == character {
                union((row, col), (row + 1, col), &mut parents, &mut ranks);
            }
            // Right
            if col + 1 < cols && board[row][col + 1] == character {
                union((row, col), (row, col + 1), &mut parents, &mut ranks);
            }
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            let character = board[row][col];
            let mut perimeter = 0;
            // Up
            if row == 0 || board[row - 1][col] != character {
                perimeter += 1;
            }
            // Left
            if col == 0 || board[row][col - 1] != character {
                perimeter += 1;
            }
            // Down
            if row + 1 == rows || board[row + 1][col] != character {
                perimeter += 1;
            }
            // Right
            if col + 1 == cols || board[row][col + 1] != character {
                perimeter += 1;
            }
            ;

            let region_key = find((row, col), &mut parents);
            if let Some(region) = regions.get_mut(&region_key) {
                region.perimeter += perimeter;
                region.area += 1;

            } else {
                regions.insert(
                    region_key,
                    Region {
                        character,
                        area: 1,
                        perimeter,
                        inside_corners: 0,
                    },
                );
            }
        }
    }

    let mut result1 = 0;

    for region in regions.values() {
        if debug_print {
            println!("{:?}", region);
        }
        result1 += region.area * region.perimeter;
    }

    let mut result2 = 0;
    (result1 as i64, result2 as i64)
}

fn union(
    mut x: (usize, usize),
    mut y: (usize, usize),
    parents: &mut HashMap<(usize, usize), (usize, usize)>,
    ranks: &mut HashMap<(usize, usize), usize>,
) {
    x = find(x, parents);
    y = find(y, parents);
    if x == y {
        return;
    }
    if ranks[&x] < ranks[&y]
    // Wenn der Rang von x kleiner als der Rang von y ist, wird y zur neuen Wurzel
    {
        parents.insert(x, y);
    } else if ranks[&x] > ranks[&y]
    // Wenn der Rang von x größer als der Rang von y ist, wird x zur neuen Wurzel
    {
        parents.insert(y, x);
    } else
    // Wenn die Ränge gleich sind, wird y zur neuen Wurzel und der Rang von y inkrementiert
    {
        parents.insert(x, y);
        ranks.insert(y, ranks.get(&y).unwrap() + 1);
    }
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut board: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            board[row][col] = c;
        }
    }

    board
}
