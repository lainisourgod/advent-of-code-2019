use std::fs::File;
use std::io::{self, BufRead};

fn read_masses() -> io::Result<Vec<i32>> {
    let mut masses: Vec<i32> = Vec::new();

    let file = File::open("./module-masses")?;
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(ip) = line {
            let token: String = ip.trim().to_string();
            if token.is_empty() {
                continue;
            };
            let num: i32 = token.parse().unwrap();
            masses.push(num)
        }
    }

    Ok(masses)
}

fn fuel_for_module(module_mass: i32) -> i32 {
    module_mass / 3 - 2
}

fn main() {
    let masses = read_masses().unwrap();
    let total_fuel_required: i32 = masses.iter().map(|mass| fuel_for_module(*mass)).sum();
    println!("Total fule: {:?}", total_fuel_required);
}
