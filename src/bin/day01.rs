use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter::Iterator;

fn main() -> Result<(), std::io::Error> {
    let f = BufReader::new(File::open("inputs/01.txt")?);
    let total_weight = f.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).fold(0, |sum, module_weight| {
        sum + module_weight / 3 - 2
    });
    println!("{}", total_weight);
    Ok(())
}
