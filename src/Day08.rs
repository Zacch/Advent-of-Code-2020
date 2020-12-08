use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Instruction {
    opcode: String,
    argument: i32
}

pub fn day08() {
    let contents = fs::read_to_string("Input/Day08.txt")
        .expect("Couldn't read the file");

    let mut program = parse_input(contents);

    let (accumulator, _) = run(&program);
    println!("Part 1: {}", accumulator);

    for i in 0..=program.len() - 1 {
        let instruction = program[i].clone();
        if instruction.opcode == "acc" {
            continue;
        }
        let new_instr = Instruction {
            opcode: if instruction.opcode == "nop" {"jmp".to_string()} else {"nop".to_string()},
            argument: instruction.argument
        };
        program[i] = new_instr;
        let (accumulator, terminated) = run(&program);
        if terminated {
            println!("Part 2: {}", accumulator);
            return
        }
        program[i] = instruction;
    }
}

fn parse_input(contents: String) -> Vec<Instruction> {
    let mut program = vec![];
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        program.push(Instruction {
            opcode: words[0].to_owned(),
            argument: i32::from_str(words[1]).unwrap()
        });
    }
    program
}

/// Runs the program until an instruction is about to be executed for the second time,
/// or until the program terminates.
/// # Returns
/// A tuple with the accumulator's value followed by a flag that is true iff the program terminated
/// correctly.
fn run(program: &Vec<Instruction>) -> (i32, bool) {
    let mut pc: i32 = 0;
    let mut accumulator = 0;
    let mut visited = vec![];

    while !visited.contains(&pc) && (pc as usize) < program.len() {
        visited.push(pc);
        let instr = &program[pc as usize];
        match instr.opcode.as_str() {
            "nop" => pc += 1,
            "acc" => {
                accumulator += instr.argument;
                pc += 1
            },
            "jmp" => pc += instr.argument,
            _ => println!("Unknown instruction: {:?}", instr)
        }
    }
    (accumulator, pc as usize == program.len())
}

