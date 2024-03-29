use aoc2019::intcpu;

fn main() -> Result<(), std::io::Error> {
    let mut mem: Vec<u32> = intcpu::read_intcode("inputs/02.txt");
    println!("{:?}", mem);
    // "replace position 1 with the value 12 and replace position 2 with the value 2."
    mem[1] = 12;
    mem[2] = 2;
    let mut pc: usize = 0;
    loop {
        match intcpu::execute_insn(pc, &mut mem) {
            Some(new_pc) => pc = new_pc,
            None => break
        }
    }
    println!("{:?}", mem);
    Ok(())
}
