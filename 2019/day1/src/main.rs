use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

//noinspection ALL
fn main() -> io::Result<()> {
    let file = "input.txt";

    // fuel is weightless
    let res = BufReader::new(File::open(file)?).lines()
        .map(|line| {
            let mass = line.unwrap().parse::<i64>().unwrap();
            mass / 3 - 2
        })
        .sum::<i64>();

    println!("Result: {}", res);

    // fuel has weight
    let res = BufReader::new(File::open(file)?).lines()
        .map(|line| {
            let mass = line.unwrap().parse::<i64>().unwrap();
            calc_fuel(mass)
        })
        .sum::<i64>();


    println!("Result: {}", res);

    Ok(())
}

fn calc_fuel(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + calc_fuel(fuel)
}



