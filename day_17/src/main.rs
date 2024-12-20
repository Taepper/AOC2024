use std::cmp::PartialEq;
use std::fmt::Display;
use tae_aoclib2025::solve_all_inputs;

fn main() {
    solve_all_inputs("day_17", do_task)
}

#[derive(Debug, Clone)]
struct MachineState {
    instruction_pointer: usize,
    register_a: usize,
    register_b: usize,
    register_c: usize,
}
impl MachineState {
    pub fn with_a_value(a_register: usize) -> Self {
        Self {
            instruction_pointer: 0,
            register_a: a_register,
            register_b: 0,
            register_c: 0,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Opcode {
    ADV, // A = A / (1 << Combo)
    BXL, // B = B ^ Literal
    BST, // B = Combo % 8
    JNZ, // if A != 0 { goto Literal; }
    BXC, // B = B ^ C
    OUT, // print(Combo % 8)
    BDV, // B = A / (1 << Combo)
    CDV, // C = A / (1 << Combo)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Operand {
    //Literal
    Literal0,
    Literal1,
    Literal2,
    Literal3,
    Literal4,
    Literal5,
    Literal6,
    Literal7,
    // Combo
    RegisterA,
    RegisterB,
    RegisterC,
    Reserved,
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let (program, program_binary, state) = parse_input(input);

    let result1 = run(&program, state.clone(), false)
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(",");

    if debug_print {
        println!(
            "Full program:\n{}",
            program
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    let result2 = search_a_register(&program, program_binary, debug_print).unwrap();

    (format!("{}", result1), format!("{}", result2))
}

fn search_a_register(
    program: &Vec<Instruction>,
    target_output: Vec<usize>,
    debug_print: bool,
) -> Option<usize> {
    let mut partial_target = vec![target_output[0]];
    let mut filtered_values = Vec::new();
    let mut lowest_n_bits = 10;
    for a_value in 0..(1 << lowest_n_bits) {
        let state = MachineState::with_a_value(a_value);
        if simulate_against_partial_target(&program, &partial_target, state) {
            filtered_values.push(a_value);
        }
    }
    while partial_target.len() < target_output.len() {
        partial_target.push(target_output[partial_target.len()]);
        let mut next_filtered_values = Vec::new();
        for a_value_0_n in filtered_values {
            for a_value_n_n_plus_3 in 0..(1 << 3) {
                let a_value = (a_value_n_n_plus_3 << lowest_n_bits) + a_value_0_n;
                let state = MachineState::with_a_value(a_value);
                if simulate_against_partial_target(&program, &partial_target, state) {
                    next_filtered_values.push(a_value);
                }
            }
        }
        lowest_n_bits += 3;
        filtered_values = next_filtered_values;
        if debug_print {
            println!("Possibilities for the lowest {lowest_n_bits} bits to hit the first {} targets (count: {})", partial_target.len(), filtered_values.len());
            if filtered_values.len() < 10 {
                println!("{:?}", filtered_values);
            } else {
                println!(
                    "{}, {}, {}, ...",
                    filtered_values[0], filtered_values[1], filtered_values[2]
                );
            }
        }
    }
    filtered_values
        .iter()
        .filter(|candidate_a_value| {
            run(
                program,
                MachineState::with_a_value(**candidate_a_value),
                false,
            ) == target_output
        })
        .min()
        .copied()
}

fn simulate_against_partial_target(
    program: &Vec<Instruction>,
    target_output: &Vec<usize>,
    mut state: MachineState,
) -> bool {
    let program_size = program.len();
    let mut target_iter = target_output.iter();
    while state.instruction_pointer < program_size {
        let instruction = &program[state.instruction_pointer];
        if let Some(out) = execute(&mut state, instruction) {
            if let Some(target) = target_iter.next() {
                if out != *target {
                    return false;
                }
            }
        }
    }
    target_iter.next() == None
}

fn run(program: &Vec<Instruction>, mut state: MachineState, debug_print: bool) -> Vec<usize> {
    let program_size = program.len();
    let mut output = Vec::new();
    while state.instruction_pointer < program_size {
        let instruction = &program[state.instruction_pointer];
        if debug_print {
            println!("{state}");
            println!("{instruction}");
        }
        if let Some(out) = execute(&mut state, instruction) {
            output.push(out);
        }
    }
    output
}

// ADV, // A = A / (1 << Combo)
// BXL, // B = B ^ Literal
// BST, // B = Combo % 8
// JNZ, // if A != 0 { goto Literal; }
// BXC, // B = B ^ C
// OUT, // print(Combo % 8)
// BDV, // B = A / (1 << Combo)
// CDV, // C = A / (1 << Combo)

fn execute(state: &mut MachineState, instruction: &Instruction) -> Option<usize> {
    state.instruction_pointer += 1;
    let operand_value = get_value(state, &instruction.operand);
    match instruction.opcode {
        Opcode::ADV => {
            state.register_a = state.register_a >> operand_value;
        }
        Opcode::BXL => {
            state.register_b = state.register_b ^ operand_value;
        }
        Opcode::BST => {
            state.register_b = operand_value % 8;
        }
        Opcode::JNZ => {
            if state.register_a != 0 {
                state.instruction_pointer = operand_value;
            }
        }
        Opcode::BXC => {
            state.register_b = state.register_b ^ state.register_c;
        }
        Opcode::OUT => return Some(operand_value % 8),
        Opcode::BDV => {
            state.register_b = state.register_a >> operand_value;
        }
        Opcode::CDV => {
            state.register_c = state.register_a >> operand_value;
        }
    }
    None
}

fn get_value(state: &MachineState, operand: &Operand) -> usize {
    match operand {
        Operand::Literal0 => 0,
        Operand::Literal1 => 1,
        Operand::Literal2 => 2,
        Operand::Literal3 => 3,
        Operand::Literal4 => 4,
        Operand::Literal5 => 5,
        Operand::Literal6 => 6,
        Operand::Literal7 => 7,
        Operand::RegisterA => state.register_a,
        Operand::RegisterB => state.register_b,
        Operand::RegisterC => state.register_c,
        Operand::Reserved => {
            panic!("Should not appear in programs..")
        }
    }
}

fn parse_input(input: &String) -> (Vec<Instruction>, Vec<usize>, MachineState) {
    let mut lines = input.lines();
    let register_a = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    assert!(lines.next().unwrap().is_empty());

    let binary_program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap());

    let mut program = Vec::new();
    for (opcode, operand) in binary_program
        .clone()
        .step_by(2)
        .zip(binary_program.clone().skip(1).step_by(2))
    {
        let opcode = parse_opcode(opcode);
        let operand = if takes_literal(opcode) {
            parse_literal(operand)
        } else {
            parse_combo(operand)
        };
        let instruction = Instruction { opcode, operand };
        program.push(instruction);
    }

    (
        program,
        binary_program.collect(),
        MachineState {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
        },
    )
}

fn parse_opcode(opcode: usize) -> Opcode {
    match opcode {
        0 => Opcode::ADV,
        1 => Opcode::BXL,
        2 => Opcode::BST,
        3 => Opcode::JNZ,
        4 => Opcode::BXC,
        5 => Opcode::OUT,
        6 => Opcode::BDV,
        7 => Opcode::CDV,
        _ => {
            panic!("Unknown opcode: {}", opcode);
        }
    }
}

fn parse_literal(operand: usize) -> Operand {
    match operand {
        0 => Operand::Literal0,
        1 => Operand::Literal1,
        2 => Operand::Literal2,
        3 => Operand::Literal3,
        4 => Operand::Literal4,
        5 => Operand::Literal5,
        6 => Operand::Literal6,
        7 => Operand::Literal7,
        _ => {
            panic!("Unknown literal operand: {}", operand);
        }
    }
}

fn parse_combo(operand: usize) -> Operand {
    match operand {
        0 => Operand::Literal0,
        1 => Operand::Literal1,
        2 => Operand::Literal2,
        3 => Operand::Literal3,
        4 => Operand::RegisterA,
        5 => Operand::RegisterB,
        6 => Operand::RegisterC,
        7 => Operand::Reserved,
        _ => {
            panic!("Unknown combo operand: {}", operand);
        }
    }
}

fn takes_literal(opcode: Opcode) -> bool {
    opcode == Opcode::BXL || opcode == Opcode::JNZ
}

impl Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {:4} B: {:4} C: {:4}",
            self.register_a, self.register_b, self.register_c
        )
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode {
            Opcode::ADV => write!(f, "A = A >> {}", self.operand),
            Opcode::BXL => write!(f, "B = B ^ {}", self.operand),
            Opcode::BST => write!(f, "B = {} % 8", self.operand),
            Opcode::JNZ => write!(f, "if A != 0 {{ goto {}; }}", self.operand),
            Opcode::BXC => write!(f, "B = B ^ C"),
            Opcode::OUT => write!(f, "print({} % 8)", self.operand),
            Opcode::BDV => write!(f, "B = A >> {}", self.operand),
            Opcode::CDV => write!(f, "C = A >> {}", self.operand),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal0 => write!(f, "0"),
            Operand::Literal1 => write!(f, "1"),
            Operand::Literal2 => write!(f, "2"),
            Operand::Literal3 => write!(f, "3"),
            Operand::Literal4 => write!(f, "4"),
            Operand::Literal5 => write!(f, "5"),
            Operand::Literal6 => write!(f, "6"),
            Operand::Literal7 => write!(f, "7"),
            Operand::RegisterA => write!(f, "A"),
            Operand::RegisterB => write!(f, "B"),
            Operand::RegisterC => write!(f, "C"),
            Operand::Reserved => write!(f, "XXX"),
        }
    }
}
