use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::Iterator;

fn get_mod_fuel(module_weight: i32) -> i32 {
    let mut last_fuel= module_weight / 3 - 2;
    let mut total_fuel = last_fuel;
    loop {
        last_fuel = last_fuel / 3 - 2;
        if last_fuel <= 0 {
            break;
        }
        total_fuel += last_fuel;
    }
    total_fuel
}

fn main() -> Result<(), std::io::Error> {
    let f = BufReader::new(File::open("inputs/01.txt")?);
    let total_weight = f.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).fold(0, |sum, module_weight| {
        sum + get_mod_fuel(module_weight)
    });
    println!("{}", total_weight);
    Ok(())
}
