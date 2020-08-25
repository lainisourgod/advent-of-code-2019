use regex::Regex;
use std::fs;

mod computer;
use computer::Computer;

fn main() {
    let program: Vec<usize> = read_program();
    println!("The program: {:?}", program);

    let mut computer = Computer::new();

    computer.set_program(&program);
    computer.execute_program();

    println!("First value: {:?}", computer.get_value_at_address(0));
}

fn read_program() -> Vec<usize> {
    let program_text = fs::read_to_string("program.txt").unwrap();
    let program_regex = Regex::new(r"(\d+)").unwrap();

    let program: Vec<usize> = program_regex
        .find_iter(&program_text)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect();

    program
}
