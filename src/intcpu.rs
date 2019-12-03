macro_rules! imem {
    ($mem: ident, $e: expr) => (&($mem[$mem[$e] as usize]));
    ($mem: ident, $e: expr, $v: expr) => (let p = $mem[$e] as usize; $mem[p] = {$v});
}

pub fn execute_insn(pc: usize, mem: &mut Vec<u32>) -> Option<usize> {
    match mem[pc] {
        1 => {
            imem!(mem, pc + 3, imem!(mem, pc + 1) + imem!(mem, pc + 2));
            return Some(pc + 4);
        }
        2 => {
            imem!(mem, pc + 3, imem!(mem, pc + 1) * imem!(mem, pc + 2));
            return Some(pc + 4);
        }
        99 => {
            return None;
        }
        _ => panic!("what is {:#?}", mem[pc])
    }
}