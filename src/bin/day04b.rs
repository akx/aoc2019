fn check(pwn: u32) -> bool {
    let pw: [u32; 6] = [
        (pwn / 100000) % 10,
        (pwn / 10000) % 10,
        (pwn / 1000) % 10,
        (pwn / 100) % 10,
        (pwn / 10) % 10,
        pwn % 10,
    ];
    let mut run_lengths: [usize; 6] = [0; 6];
    for i in 0 .. 6 {
        if i > 0 && pw[i] < pw[i - 1] {
            return false;
        }
    }
    let mut i = 0;
    while i < 6 {
        for j in i .. 6 {
            if pw[i] == pw[j] {
                run_lengths[i] += 1;
            }
        }
        i += run_lengths[i];
    }
    //println!("{}: rl={:?}", pwn, run_lengths);
    return run_lengths.iter().any(|&n| n == (2 as usize));
}

fn main() -> Result<(), std::io::Error> {
    let min: u32 = 145852;
    let max: u32 = 616942;
    let mut n_matching: u32 = 0;
    for pwn in min ..= max {
        if check(pwn) {
            n_matching += 1;
        }
    }
    println!("total = {}", n_matching);
    Ok(())
}
