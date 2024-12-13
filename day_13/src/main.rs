use tae_aoclib2025::{solve_all_inputs};

fn main() {
    solve_all_inputs("day_13", do_task)
}

#[derive(Debug)]
struct Game {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn do_task(input: &String) -> (i64, i64) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let input : Vec<Game> = parse_input(input);

    let mut result1 = 0;

    for game in input {
        if let Some(cost) = min_cost(&game){
            if debug_print {
                println!("{game:?} cost is {cost}");
            }
            result1 += cost;
        }
    }

    let mut result2 = 0;
    (result1 as i64, result2)
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

    for a_presses in 0..(budget/3)+1 {
        let b_presses = budget - a_presses * 3;

        // println!("trying with {a_presses:?} and {b_presses:?}");

        if try_game(game, a_presses, b_presses) {
           return true;
        }
    }

    false
}

fn try_game(game: &Game, a_presses: usize, b_presses: usize) -> bool {
    let mut current = (0, 0);
    for _ in 0..a_presses {
        current.0 += game.a.0;
        current.1 += game.a.1;
    }
    for _ in 0..b_presses {
        current.0 += game.b.0;
        current.1 += game.b.1;
    }
    // println!("current is {current:?}");
    current == game.prize
}

fn parse_input(input: &String) -> Vec<Game> {
    let mut result: Vec<Game> = Vec::new();

    let mut input_lines = input.lines();

    while let Some(mut a_line) = input_lines.next() {
        if let Some (mut b_line) = input_lines.next() {
            if let Some (mut prize_line) = input_lines.next() {
                input_lines.next();
                assert!(a_line.starts_with("Button A: "));
                a_line = a_line[10..].as_ref();
                assert!(b_line.starts_with("Button B: "));
                b_line = b_line[10..].as_ref();
                assert!(prize_line.starts_with("Prize: "));
                prize_line = prize_line[7..].as_ref();

                let a = a_line.split(", ").map(|x| x[2..].parse::<usize>().unwrap()).collect::<Vec<usize>>();
                assert_eq!(a.len(), 2);
                let b = b_line.split(", ").map(|x| x[2..].parse::<usize>().unwrap()).collect::<Vec<usize>>();
                assert_eq!(b.len(), 2);
                let prize = prize_line.split(", ").map(|x| x[2..].parse::<usize>().unwrap()).collect::<Vec<usize>>();
                assert_eq!(prize.len(), 2);

                result.push(Game{a: (a[0], a[1]), b: (b[0], b[1]), prize: (prize[0], prize[1]) });
            }
        }
    }

    result
}
