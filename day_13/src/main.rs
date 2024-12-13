use std::cmp::{max, min};
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_13", do_task)
}

#[derive(Debug, Clone)]
struct Game {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let (games, real_games): (Vec<Game>, Vec<Game>) = parse_input(input);

    let mut result1 = 0;

    for game in games {
        if let Some(cost) = min_cost(&game) {
            if debug_print {
                println!("{game:?} cost is {cost}");
            }
            result1 += cost;
        }
    }

    let mut result2 = 0;
    for game in real_games {
        if let Some((a_presses, b_presses)) = solve_large(&game, debug_print) {
            let cost = a_presses * 3 + b_presses;
            if debug_print {
                println!("{game:?} cost is {cost} by pressing a {a_presses} times and b {b_presses} times.");
            }
            assert_eq!(a_presses * game.a.0 + b_presses * game.b.0, game.prize.0);
            assert_eq!(a_presses * game.a.1 + b_presses * game.b.1, game.prize.1);
            result2 += cost;
        } else {
            println!("Unsolvable {game:?}. For reasons, see above.");
        }
    }

    (result1 as i64, result2 as i64)
}

// Assumes very big and almost equal prize positions
fn solve_large(game: &Game, debug_print: bool) -> Option<(usize, usize)> {
    assert_ne!(game.a.0, game.a.1);
    assert_ne!(game.b.0, game.b.1);

    // Unsolvable, one of the prize coordinates would inevitably grow way beyond the other
    if game.a.0 < game.a.1 && game.b.0 < game.b.1 {
        println!("Unsolvable, because both buttons move in the same relative direction (x-axis). We totally lose balance!");
        return None;
    }

    // Unsolvable, one of the prize coordinates would inevitably grow way beyond the other
    if game.a.0 > game.a.1 && game.b.0 > game.b.1 {
        println!("Unsolvable, because both buttons move in the same relative direction (y-axis). We totally lose balance!");
        return None;
    }

    let a_diff = game.a.0 as i64 - game.a.1 as i64;
    let b_diff = game.b.0 as i64 - game.b.1 as i64;

    let prize_diff = game.prize.0 as i64 - game.prize.1 as i64;

    if let Some((a_presses, b_presses)) = reach_diff(a_diff, b_diff, prize_diff) {
        let coords_after_diff_reached = (
            a_presses * game.a.0 + b_presses * game.b.0,
            a_presses * game.a.1 + b_presses * game.b.1,
        );

        let x_distance_left = game.prize.0 - coords_after_diff_reached.0;
        let y_distance_left = game.prize.1 - coords_after_diff_reached.1;

        let (a_presses_cycle, b_presses_cycle) =
            get_neutral_presses(a_diff.abs() as u64, b_diff.abs() as u64);
        let x_distance_covered_per_cycle = a_presses_cycle * game.a.0 + b_presses_cycle * game.b.0;
        let y_distance_covered_per_cycle = a_presses_cycle * game.a.1 + b_presses_cycle * game.b.1;

        if x_distance_left % x_distance_covered_per_cycle != 0 {
            println!("Unsolvable, the remaining distance ({x_distance_left}, {y_distance_left}) after pressing a {a_presses} times and b {b_presses} times cannot be covered by cycling our neutral distance ({x_distance_covered_per_cycle}, {y_distance_covered_per_cycle}), which requires {a_presses_cycle} presses of button a and {b_presses_cycle} presses of button b.");
            return None;
        }

        let cycles_for_x_distance = x_distance_left / x_distance_covered_per_cycle;
        let cycles_for_y_distance = y_distance_left / y_distance_covered_per_cycle;

        assert_eq!(cycles_for_x_distance, cycles_for_y_distance);
        let cycles = cycles_for_x_distance;

        let total_a_presses = cycles * a_presses_cycle + a_presses;
        let total_b_presses = cycles * b_presses_cycle + b_presses;

        return Some((total_a_presses, total_b_presses));
    }
    println!("Did not converge to target diff ({prize_diff}) in time! It should (!!) be impossible with the numbers {a_diff} and {b_diff}.");
    None
}

fn reach_diff(a_diff: i64, b_diff: i64, prize_diff: i64) -> Option<(usize, usize)> {
    // Solve a_diff * a_presses + b_diff * b_presses = prize_diff
    let positive_diff = max(a_diff, b_diff);
    let mut positive_presses = 0;
    let negative_diff = min(a_diff, b_diff);
    let mut negative_presses = 0;

    let mut cur = 0;
    for _ in 0..(prize_diff.abs() * 2 + 1)*1000 {
        if cur == prize_diff {
            return if a_diff < 0 {
                Some((negative_presses, positive_presses))
            } else {
                Some((positive_presses, negative_presses))
            };
        }
        if cur < prize_diff {
            positive_presses += 1;
            cur += positive_diff;
        }
        if cur > prize_diff {
            negative_presses += 1;
            cur += negative_diff;
        }
    }
    None
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn get_neutral_presses(mut a_diff_abs: u64, mut b_diff_abs: u64) -> (usize, usize) {
    let lcm = lcm(a_diff_abs, b_diff_abs);
    ((lcm / a_diff_abs) as usize, (lcm / b_diff_abs) as usize)
}

fn min_cost(game: &Game) -> Option<usize> {
    for budget in 1..401 {
        if try_budget(game, budget) {
            return Some(budget);
        }
    }
    None
}

fn try_budget(game: &Game, budget: usize) -> bool {
    for a_presses in 0..(budget / 3) + 1 {
        let b_presses = budget - a_presses * 3;
        if try_game(game, a_presses, b_presses) {
            return true;
        }
    }
    false
}

fn try_game(game: &Game, a_presses: usize, b_presses: usize) -> bool {
    let mut current = (0, 0);
    current.0 = a_presses * game.a.0 + b_presses * game.b.0;
    current.1 = a_presses * game.a.1 + b_presses * game.b.1;
    current == game.prize
}

fn parse_input(input: &String) -> (Vec<Game>, Vec<Game>) {
    let mut games: Vec<Game> = Vec::new();
    let mut real_games: Vec<Game> = Vec::new();

    let mut input_lines = input.lines();

    while let Some(mut a_line) = input_lines.next() {
        if let Some(mut b_line) = input_lines.next() {
            if let Some(mut prize_line) = input_lines.next() {
                input_lines.next();
                assert!(a_line.starts_with("Button A: "));
                a_line = a_line[10..].as_ref();
                assert!(b_line.starts_with("Button B: "));
                b_line = b_line[10..].as_ref();
                assert!(prize_line.starts_with("Prize: "));
                prize_line = prize_line[7..].as_ref();

                let a = a_line
                    .split(", ")
                    .map(|x| x[2..].parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                assert_eq!(a.len(), 2);
                let b = b_line
                    .split(", ")
                    .map(|x| x[2..].parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                assert_eq!(b.len(), 2);
                let prize = prize_line
                    .split(", ")
                    .map(|x| x[2..].parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                assert_eq!(prize.len(), 2);

                games.push(Game {
                    a: (a[0], a[1]),
                    b: (b[0], b[1]),
                    prize: (prize[0], prize[1]),
                });
                let real_prize = (prize[0] + 10000000000000, prize[1] + 10000000000000);
                real_games.push(Game {
                    a: (a[0], a[1]),
                    b: (b[0], b[1]),
                    prize: real_prize,
                });
            }
        }
    }

    (games, real_games)
}
