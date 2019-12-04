type PWA = [u32; 6];

fn pwn_to_pwa(pwn: u32) -> PWA {
    return [
        (pwn / 100000) % 10,
        (pwn / 10000) % 10,
        (pwn / 1000) % 10,
        (pwn / 100) % 10,
        (pwn / 10) % 10,
        pwn % 10,
    ];
}

fn check_a(pw: &PWA) -> bool {
    let mut has_double: u8 = 0; 
    for i in 1 .. 6 {
        if pw[i] < pw[i - 1] {
            return false;
        }
        if pw[i] == pw[i - 1] {
            has_double += 1;
        }
    }
    return has_double >= 1;
}

fn check_b(pw: &PWA) -> bool {
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
        if run_lengths[i] == 2 {
            return true;
        }
        i += run_lengths[i];
    }
    return false;
}


fn main() -> Result<(), std::io::Error> {
    let min: u32 = 145852;
    let max: u32 = 616942;
    let mut a_matching: u32 = 0;
    let mut b_matching: u32 = 0;
    for pwn in min ..= max {
        let pw = pwn_to_pwa(pwn);
        if check_a(&pw) {
            a_matching += 1;
        }
        if check_b(&pw) {
            b_matching += 1;
        }
    }
    println!("a = {}, b = {}", a_matching, b_matching);
    Ok(())
}
