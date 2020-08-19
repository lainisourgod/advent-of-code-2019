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

fn fuel_for_module(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_for_fuels(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + fuel_for_fuels(fuel)
    } else {
        0
    }
}

fn main() {
    let masses = read_masses().unwrap();
    let fuel_for_modules: i32 = masses.iter().map(|mass| fuel_for_module(*mass)).sum();
    println!("Fuel for models: {:?}", fuel_for_modules);

    let fuel_for_fuel: i32 = masses.iter().map(|mass| fuel_for_fuels(*mass)).sum();
    println!("Fuel TOTAL: {:?}", fuel_for_fuel);
}
