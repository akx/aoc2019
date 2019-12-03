use std::fs;

macro_rules! imem {
    ($mem: ident, $e: expr) => (&($mem[$mem[$e] as usize]));
    ($mem: ident, $e: expr, $v: expr) => (let p = $mem[$e] as usize; $mem[p] = {$v});
}

fn main() -> Result<(), std::io::Error> {
    let mut mem: Vec<u32> = fs::read_to_string("inputs/02.txt")?
        .trim()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    println!("{:?}", mem);
    // "replace position 1 with the value 12 and replace position 2 with the value 2."
    mem[1] = 12;
    mem[2] = 2;
    let mut pc = 0;
    loop {
        match mem[pc] {
            1 => {
                imem!(mem, pc + 3, imem!(mem, pc + 1) + imem!(mem, pc + 2));
                pc += 4;
            }
            2 => {
                imem!(mem, pc + 3, imem!(mem, pc + 1) * imem!(mem, pc + 2));
                pc += 4;
            }
            99 => {
                break;
            }
            _ => panic!("what is {:#?}", mem[pc])
        }
    }
    println!("{:?}", mem);
    Ok(())
}
