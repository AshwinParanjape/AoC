use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let mut sum = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines(){
        //println!("You typed: {}", line.unwrap().trim());
        let i = i64::from_str_radix(line.unwrap().trim(), 10).unwrap();
        sum += i;
    }
    println!("{}", sum);
    Ok(())
}
