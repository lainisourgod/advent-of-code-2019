use regex::Regex;
use std::fs;

type Program = Vec<usize>;

fn main() {
    let program: Program = read_program();
    println!("The program: {:?}", program);

    let new_program = execute_program(program);
    println!("\nNew program: {:?}", new_program);
    println!("First value: {:?}", new_program.get(0));
}

fn read_program() -> Program {
    let program_text = fs::read_to_string("program.txt").unwrap();
    let program_regex = Regex::new(r"(\d+)").unwrap();

    let program: Program = program_regex
        .find_iter(&program_text)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect();

    program
}

fn execute_program(mut program: Program) -> Program {
    let mut result: usize;
    let mut index: usize = 0;
    loop {
        if index > program.len() - 4 {
            break;
        }
        let code_block: Vec<usize>;
        {
            code_block = Vec::from(&program[index..index + 4]);
            match code_block[0] {
                1 => {
                    result = program[code_block[1]] + program[code_block[2]];
                }
                2 => {
                    result = program[code_block[1]] * program[code_block[2]];
                }
                99 => break,
                _ => panic!("Incorrect first code at block {:?}", code_block),
            }
        }
        program[code_block[3]] = result;
        index += 4;
    }

    program
}

#[test]
fn test_program_execution() {
    let cases = [
        (
            Program::from([1, 0, 0, 0, 99]),
            Program::from([2, 0, 0, 0, 99]),
        ),
        (
            Program::from([2, 3, 0, 3, 99]),
            Program::from([2, 3, 0, 6, 99]),
        ),
        (
            Program::from([2, 4, 4, 5, 99, 0]),
            Program::from([2, 4, 4, 5, 99, 9801]),
        ),
        (
            Program::from([1, 1, 1, 4, 99, 5, 6, 0, 99]),
            Program::from([30, 1, 1, 4, 2, 5, 6, 0, 99]),
        ),
    ];

    for case in &cases {
        assert_eq!(execute_program(case.0.clone()), case.1)
    }
}
