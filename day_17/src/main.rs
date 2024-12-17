use std::cmp::PartialEq;
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
    output: Vec<usize>,
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
    L_Literal0,
    L_Literal1,
    L_Literal2,
    L_Literal3,
    L_Literal4,
    L_Literal5,
    L_Literal6,
    L_Literal7,
    // Combo
    C_Literal0,
    C_Literal1,
    C_Literal2,
    C_Literal3,
    C_RegisterA,
    C_RegisterB,
    C_RegisterC,
    C_Reserved,
}

fn do_task(input: &String) -> (String, String) {
    let debug_print =
        std::env::var("DEBUG_PRINT").unwrap_or("0".to_string()) == "1" && input.len() < 10000;

    let (program, program_binary, state) = parse_input(input);

    let result1 = run(program.clone(), state.clone())
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(",");

    let mut result2 = 0;

    let program_size = program.len();
    let target_output_size = program_binary.len();
    let target_output = program_binary;

    for a_value in 100000000..200000000 {
        let mut cur_state = state.clone();
        cur_state.register_a = a_value;
        while cur_state.instruction_pointer < program_size {
            let instruction = &program[cur_state.instruction_pointer];
            execute(&mut cur_state, instruction);
            if cur_state.output.len() == target_output_size {
                break;
            }
        }
        if cur_state.output == target_output {
            println!("Reached goal for input {a_value}..");
            result2 = a_value;
            break;
        }
    }

    (format!("{}", result1), format!("{}", result2))
}

fn run(program: Vec<Instruction>, mut state: MachineState) -> Vec<usize> {
    let program_size = program.len();
    while state.instruction_pointer < program_size {
        let instruction = &program[state.instruction_pointer];
        execute(&mut state, instruction);
    }
    state.output
}
// ADV, // A = A / (1 << Combo)
// BXL, // B = B ^ Literal
// BST, // B = Combo % 8
// JNZ, // if A != 0 { goto Literal; }
// BXC, // B = B ^ C
// OUT, // print(Combo % 8)
// BDV, // B = A / (1 << Combo)
// CDV, // C = A / (1 << Combo)

fn execute(state: &mut MachineState, instruction: &Instruction) {
    let operand_value = get_value(state, &instruction.operand);
    match instruction.opcode {
        Opcode::ADV => {
            state.register_a = state.register_a / (1 << operand_value);
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
                return;
            }
        }
        Opcode::BXC => {
            state.register_b = state.register_b ^ state.register_c;
        }
        Opcode::OUT => state.output.push(operand_value % 8),
        Opcode::BDV => {
            state.register_b = state.register_a / (1 << operand_value);
        }
        Opcode::CDV => {
            state.register_c = state.register_a / (1 << operand_value);
        }
    }
    state.instruction_pointer += 1;
}

fn get_value(state: &mut MachineState, operand: &Operand) -> usize {
    match operand {
        Operand::L_Literal0 => 0,
        Operand::L_Literal1 => 1,
        Operand::L_Literal2 => 2,
        Operand::L_Literal3 => 3,
        Operand::L_Literal4 => 4,
        Operand::L_Literal5 => 5,
        Operand::L_Literal6 => 6,
        Operand::L_Literal7 => 7,
        Operand::C_Literal0 => 0,
        Operand::C_Literal1 => 1,
        Operand::C_Literal2 => 2,
        Operand::C_Literal3 => 3,
        Operand::C_RegisterA => state.register_a,
        Operand::C_RegisterB => state.register_b,
        Operand::C_RegisterC => state.register_c,
        Operand::C_Reserved => {
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
            output: Vec::new(),
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
        0 => Operand::L_Literal0,
        1 => Operand::L_Literal1,
        2 => Operand::L_Literal2,
        3 => Operand::L_Literal3,
        4 => Operand::L_Literal4,
        5 => Operand::L_Literal5,
        6 => Operand::L_Literal6,
        7 => Operand::L_Literal7,
        _ => {
            panic!("Unknown opcode: {}", operand);
        }
    }
}

fn parse_combo(operand: usize) -> Operand {
    match operand {
        0 => Operand::C_Literal0,
        1 => Operand::C_Literal1,
        2 => Operand::C_Literal2,
        3 => Operand::C_Literal3,
        4 => Operand::C_RegisterA,
        5 => Operand::C_RegisterB,
        6 => Operand::C_RegisterC,
        7 => Operand::C_Reserved,
        _ => {
            panic!("Unknown opcode: {}", operand);
        }
    }
}

fn takes_literal(opcode: Opcode) -> bool {
    opcode == Opcode::BXL || opcode == Opcode::JNZ
}
