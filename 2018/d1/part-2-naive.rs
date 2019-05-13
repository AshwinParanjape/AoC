use std::io::{self, BufRead};
use std::collections::HashSet;


fn main() -> io::Result<()> {
    let mut sums = HashSet::new();
    let mut sum = 0;
    let stdin = io::stdin();
    let mut elems = vec![];
    for line in stdin.lock().lines(){
        let i = i64::from_str_radix(line.unwrap().trim(), 10).unwrap();
        elems.push(i);
    }
    let mut done = false;
    loop {
        for elem in elems.iter(){
            sums.insert(sum);
            sum += elem;
            if sums.contains(&sum){
                println!("{}", sum);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    Ok(())
}
