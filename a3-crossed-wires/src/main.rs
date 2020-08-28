use std::fs;

fn main() {
    let wires = read_wires();
    println!("Wires: {:?}", wires);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(chr: char) -> Self {
        match chr {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: usize,
}

impl From<&str> for Move {
    fn from(string: &str) -> Self {
        Move {
            direction: string.chars().nth(0).unwrap().into(),
            distance: string.get(1..).unwrap().parse().unwrap(),
        }
    }
}

type Wire = Vec<Move>;

fn read_wires() -> Vec<Wire> {
    let text = fs::read_to_string("input.txt").unwrap();

    let wires: Vec<Wire> = text
        .trim()
        .split("\n") // First split -- on two wires
        .map(|wire_text| {
            wire_text
                .trim()
                .split(",")
                .map(|string| string.into())
                .collect()
        })
        .collect();

    wires
}
