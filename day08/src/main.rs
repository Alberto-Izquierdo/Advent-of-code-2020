use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

#[derive(Clone)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

#[derive(Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

fn read_line(line: String) -> Instruction {
    let parts = line.split(" ");
    let operation = match parts.clone().next().unwrap() {
        "nop" => Operation::NOP,
        "acc" => Operation::ACC,
        "jmp" => Operation::JMP,
        _ => panic!(),
    };
    let argument = parts.skip(1).next().unwrap().parse::<i32>().unwrap();
    Instruction {
        operation,
        argument,
    }
}

fn read_program(lines: Lines<BufReader<File>>) -> Vec<Instruction> {
    lines
        .map(|line| read_line(line.unwrap()))
        .collect::<Vec<Instruction>>()
}

fn execute_instruction(instruction: &Instruction, index: i32, accumulator: i32) -> (i32, i32) {
    match instruction.operation {
        Operation::NOP => (index + 1, accumulator),
        Operation::JMP => (index + instruction.argument, accumulator),
        Operation::ACC => (index + 1, accumulator + instruction.argument),
    }
}

fn get_accumulator_when_program_finishes(program: Vec<Instruction>) -> Result<i32, String> {
    let mut accumulator = 0;
    let mut lines_visited = HashSet::new();
    let mut index: i32 = 0;
    while index < program.len() as i32 {
        if index < 0 {
            return Err(String::from("Wrong program found"));
        }
        let instruction = program.get(index as usize).unwrap();
        let result = execute_instruction(instruction, index, accumulator);
        index = result.0;
        accumulator = result.1;
        match lines_visited.get(&index) {
            Some(_) => {
                return Err(String::from("Loop detected"));
            }
            None => {
                lines_visited.insert(index);
            }
        }
    }
    return Ok(accumulator);
}

fn get_accumulator_of_correct_program(program: Vec<Instruction>) -> i32 {
    for i in 0..program.len() - 1 {
        let instruction = program.get(i).unwrap();
        let new_instruction = match instruction.operation {
            Operation::JMP => Instruction {
                operation: Operation::NOP,
                argument: instruction.argument,
            },
            Operation::NOP => Instruction {
                operation: Operation::JMP,
                argument: instruction.argument,
            },
            _ => continue,
        };
        let slices = program.split_at(i + 1);
        let mut new_program = slices.0.split_last().unwrap().1.to_vec();
        new_program.push(new_instruction);
        let new_program = new_program.into_iter().chain(slices.1.to_vec()).collect();
        match get_accumulator_when_program_finishes(new_program) {
            Ok(accumulator) => return accumulator,
            Err(_) => {}
        }
    }
    0
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    let program = read_program(lines);
    let accumulator = get_accumulator_of_correct_program(program);
    println!("Result = {}", accumulator);
}
