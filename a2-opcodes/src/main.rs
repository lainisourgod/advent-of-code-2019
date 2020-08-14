use regex::Regex;
use std::fs;

type Program = Vec<usize>;

fn read_program() -> Program {
    let program_text = fs::read_to_string("program.txt").unwrap();
    let program_regex = Regex::new(r"(\d+)").unwrap();

    let program: Program = program_regex
        .find_iter(&program_text)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect();

    program
}

fn execute_program(program: Program) -> Program {
    program
}

fn main() {
    let program: Program = read_program();
    println!("The program: {:?}", program);

    let new_program = execute_program(program);
    println!("First value: {:?}", new_program.get(0));
}
