use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

enum Operation {
    NOP,
    ACC,
    JMP,
}

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

fn get_accumulator_when_loop_starts(program: Vec<Instruction>) -> i32 {
    let mut accumulator = 0;
    let mut lines_visited = HashSet::new();
    let mut index: i32 = 0;
    loop {
        let instruction = program.get(index as usize).unwrap();
        let result = execute_instruction(instruction, index, accumulator);
        index = result.0;
        accumulator = result.1;
        match lines_visited.get(&index) {
            Some(_) => return accumulator,
            None => {
                lines_visited.insert(index);
            }
        }
    }
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    let program = read_program(lines);
    let accumulator = get_accumulator_when_loop_starts(program);
    println!("Result = {}", accumulator);
}
