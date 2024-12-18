use std::collections::HashSet;
use std::str::Lines;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_05", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";
    let mut result1 = 0;
    let mut result2 = 0;

    let (rules, updates) = get_rules_and_updates(input.lines());

    for update in updates {
        let current_rules = rules
            .iter()
            .filter(|(x, y)| update.contains(x) && update.contains(y))
            .map(|(x, y)| (*x, *y))
            .collect::<Vec<(u64, u64)>>();
        if satisfies_rules(&update, &current_rules, debug_print) {
            result1 += update[update.len() / 2];
        } else {
            let correct_order = order_correctly(update, current_rules, debug_print);
            result2 += correct_order[correct_order.len() / 2];
        }
    }

    (format!("{result1}"), format!("{result2}"))
}

fn get_rules_and_updates(lines: Lines) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut rules_section = true;
    for line in lines {
        if line.is_empty() {
            rules_section = false;
            continue;
        }
        if rules_section {
            let (x, y) = line
                .split_once("|")
                .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                .unwrap();
            assert!(x < 100);
            assert!(y < 100);
            rules.push((x, y));
        } else {
            let ls: Vec<u64> = line.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
            for x in &ls {
                assert!(*x < 100);
            }

            let mut tmp: Vec<u64> = ls.clone();
            tmp.dedup();
            assert_eq!(tmp.len(), ls.len());

            updates.push(ls);
        }
    }

    (rules, updates)
}

fn satisfies_rules(
    number_order: &Vec<u64>,
    edge_list: &Vec<(u64, u64)>,
    _debug_print: bool,
) -> bool {
    let mut precondition_edges = Vec::new();
    precondition_edges.resize(100, HashSet::new());
    let mut edges = Vec::new();
    edges.resize(100, HashSet::new());

    for (from, to) in edge_list {
        edges[*from as usize].insert(*to);
        precondition_edges[*to as usize].insert(*from);
    }

    for x in number_order {
        if !precondition_edges[*x as usize].is_empty() {
            return false;
        }
        for to in edges[*x as usize].iter() {
            precondition_edges[*to as usize].remove(x);
        }
    }

    true
}

fn order_correctly(vertices: Vec<u64>, edge_list: Vec<(u64, u64)>, debug_print: bool) -> Vec<u64> {
    let n = vertices.len();

    let mut precondition_edges = Vec::new();
    precondition_edges.resize(100, HashSet::new());
    let mut edges = Vec::new();
    edges.resize(100, HashSet::new());

    let mut initially_empty: HashSet<&u64> = HashSet::from_iter(vertices.iter());

    for (from, to) in edge_list {
        edges[from as usize].insert(to);
        precondition_edges[to as usize].insert(from);
        initially_empty.remove(&to);
    }

    let mut solution = Vec::new();
    assert_eq!(initially_empty.len(), 1);
    solution.push(**initially_empty.iter().next().unwrap());

    for pos in 0..n - 1 {
        let x = solution[pos];
        if debug_print {
            println!("solution: {:?}", solution);
        }

        let mut now_possible = Vec::new();
        for to in edges[x as usize].clone() {
            precondition_edges[to as usize].remove(&x);
            edges[x as usize].remove(&to);
            if precondition_edges[to as usize].is_empty() {
                now_possible.push(to);
            }
        }
        assert_eq!(now_possible.len(), 1);
        for n in now_possible {
            solution.push(n);
        }
    }

    solution
}
