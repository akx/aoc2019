use aoc2019::intcpu;

fn main() -> Result<(), std::io::Error> {
    let orig_mem: Vec<u32> = intcpu::read_intcode("inputs/02.txt");
    for a in 0 .. 99 {
        for b in 0 .. 99 {
            let mut mem = orig_mem.clone();
            mem[1] = a;
            mem[2] = b;
            intcpu::execute_to_end(&mut mem);
            if mem[0] == 19690720 {
                println!("{}, {}, {}", a, b, 100 * a + b);
                break;
            }
        }
    }
    Ok(())
}
