use std::collections::{HashMap, HashSet};
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_08", do_task)
}

fn do_task(input: &String) -> (String, String) {
    let debug_print = std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1";

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                if let Some(x) = antennas.get_mut(&char) {
                    x.push((row, col));
                } else {
                    antennas.insert(char, vec![(row, col)]);
                }
            }
        }
    }

    let mut antinodes = HashSet::new();
    let mut antinodes_strict = HashSet::new();
    for (char, antennas) in &antennas {
        for antenna1 in antennas {
            for antenna2 in antennas {
                if antenna1 == antenna2 {
                    continue;
                }
                let mut first: bool = true;
                let mut antinode_row = antenna1.0;
                let mut antinode_col = antenna1.1;
                antinodes.insert((antinode_row, antinode_col));
                loop {
                    // Calculate antinode + (antenna1 - antenna2)
                    if antenna2.0 > antinode_row + antenna1.0 {
                        // Out-of-bounds up
                        break;
                    }
                    if antenna2.1 > antinode_col + antenna1.1 {
                        // Out-of-bounds left
                        break;
                    }
                    // println!("{} {} {}", antinode_row, antenna1.0, antenna2.0);
                    antinode_row = antinode_row + antenna1.0 - antenna2.0;
                    antinode_col = antinode_col + antenna1.1 - antenna2.1;
                    if antinode_row >= rows {
                        // Out-of-bounds down
                        break;
                    }
                    if antinode_col >= cols {
                        // Out-of-bounds right
                        break;
                    }
                    if debug_print {
                        println!(
                            "{} antennas {:?} and {:?} map to {},{}",
                            char, antenna1, antenna2, antinode_row, antinode_col
                        );
                    }
                    if first {
                        antinodes_strict.insert((antinode_row, antinode_col));
                    }
                    antinodes.insert((antinode_row, antinode_col));
                    first = false;
                }
            }
        }
    }

    if debug_print {
        let mut debug_output: Vec<Vec<String>> = vec![Vec::new(); rows];
        for row in 0..rows {
            for _col in 0..cols {
                debug_output[row].push(".".to_string());
            }
        }
        for (a_row, a_col) in &antinodes {
            debug_output[*a_row][*a_col] = "#".to_string();
        }
        println!(
            "{:}",
            debug_output
                .iter()
                .map(|x| x.join("") + "\n")
                .collect::<Vec<String>>()
                .join("")
        );
    }

    let result1 = antinodes_strict.len() as i64;
    let result2 = antinodes.len() as i64;
    (format!("{result1}"), format!("{result2}"))
}
