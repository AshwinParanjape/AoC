use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut twos = 0;
    let mut threes = 0;
    for line in stdin.lock().lines(){
        let mut char_counts = HashMap::new();
        for ch in line.unwrap().trim().chars() {
            let counter = char_counts.entry(ch).or_insert(0);
            *counter += 1;
        }
        twos += (char_counts.values().filter(|v| **v == 2).next().is_some()) as i64;
        threes += (char_counts.values().filter(|v| **v == 3).next().is_some()) as i64;
    }
    println!("{}", twos*threes);
    Ok(())
}
