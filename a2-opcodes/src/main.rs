use regex::Regex;
use std::fs;

mod computer;
use computer::Computer;

fn main() {
    let mut computer = Computer::new();

    let mut program: Vec<usize> = read_program();

    // First star challenge
    program[1] = 12;
    program[2] = 2;

    computer.set_program(&program);
    computer.execute_program();

    println!("First star: {:?}", computer.get_value_at_address(0));

    // Second star challenge
    let required_output = 19690720;
    let mut result = 0;
    computer.reset();

    for noun in 0..=99 {
        for verb in 0..=99 {
            program[1] = noun;
            program[2] = verb;
            computer.set_program(&program);
            computer.execute_program();

            let output = computer.get_value_at_address(0);
            if output == required_output {
                result = 100 * noun + verb;
                break;
            }
        }
    }

    println!("Second star: {}", result);
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
