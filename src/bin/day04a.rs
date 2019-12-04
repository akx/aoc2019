fn check(pw: &Vec<u8>) -> bool {
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

fn main() -> Result<(), std::io::Error> {
    let min: u32 = 145852;
    let max: u32 = 616942;
    let mut n_matching: u32 = 0;
    for pwn in min ..= max {
        let pw = pwn.to_string().into_bytes();
        if check(&pw) {
            println!("{}", pwn);
            n_matching += 1;
        }
    }
    println!("total = {}", n_matching);
    Ok(())
}
