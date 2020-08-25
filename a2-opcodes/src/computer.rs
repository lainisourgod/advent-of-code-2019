/// A basic unit of program.
enum Instruction {
    Add { a: usize, b: usize, to: usize },
    Mul { a: usize, b: usize, to: usize },
    Halt,
}

impl From<&[usize]> for Instruction {
    /// A mapping from vector of values to Instruction for Computer.
    fn from(params: &[usize]) -> Self {
        use Instruction::*;
        match params[0] {
            1 => Add {
                a: params[1],
                b: params[2],
                to: params[3],
            },
            2 => Mul {
                a: params[1],
                b: params[2],
                to: params[3],
            },
            99 => Halt {},
            _ => unreachable!(),
        }
    }
}

/// Computer stores a program in a memory as raw values. At each step computer reads some values
/// from the memory at pointer, interprets it as an Instruction and executes mutating the memory.
/// When an Instruction::Halt is fed, execution stops.
pub struct Computer {
    memory: Vec<usize>,
    pointer: usize,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: Vec::new(),
            pointer: 0,
        }
    }

    /// Clean up internal state.
    pub fn reset(&mut self) {
        *self = Computer::new();
    }

    /// Load new program to a memory.
    pub fn set_program(&mut self, program: &[usize]) {
        self.memory = Vec::from(program);
        self.pointer = 0;
    }

    /// Get value located in memory at a given address.
    pub fn get_value_at_address(&self, address: usize) -> usize {
        self.memory[address]
    }

    /// Step through the program and execute every instruction until Halt.
    pub fn execute_program(&mut self) {
        loop {
            if self.pointer > self.memory.len() - 4 {
                break;
            }

            let instruction = Instruction::from(&self.memory[self.pointer..self.pointer + 4]);
            self.execute_instruction(&instruction);
        }
    }

    /// Take an Instruction and mutate memory and pointer according to the rules of an instruction
    /// execution.
    fn execute_instruction(&mut self, instruction: &Instruction) {
        use Instruction::*;

        match instruction {
            Halt => self.pointer = self.memory.len(),
            Add { a, b, to } => {
                self.memory[*to] = self.memory[*a] + self.memory[*b];
                self.pointer += 4;
            }
            Mul { a, b, to } => {
                self.memory[*to] = self.memory[*a] * self.memory[*b];
                self.pointer += 4;
            }
        }
    }
}

#[test]
fn test_program_execution() {
    let mut computer = Computer::new();

    let cases: &[(&[usize], &[usize])] = &[
        (&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]),
        (&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]),
        (&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]),
        (
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];

    for case in cases {
        computer.set_program(case.0);
        computer.execute_program();
        assert_eq!(computer.memory, case.1);
    }
}
