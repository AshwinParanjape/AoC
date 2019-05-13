use std::io::{self, BufRead};
use std::cmp;
use std::cmp::Ordering;

struct kv {
    key: i64,
    value: i64
}

impl Ord for kv {
    fn cmp(&self, other: &kv) -> Ordering {
        self.key.cmp(&other.key)
    }
}
impl PartialOrd for kv {
    fn partial_cmp(&self, other: &kv) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for kv {
    fn eq(&self, other: &kv) -> bool {
        self.key == other.key
    }
}
impl Eq for kv {}
fn main() -> io::Result<()> {
    let mut sum = 0;
    let stdin = io::stdin();
    let mut elems = vec![];
    let mut sums = vec![];
    let mut min_kv = kv {key: i64::max_value(), value: -1};

    for (i, line) in stdin.lock().lines().enumerate(){
        //To read input
        let elem = i64::from_str_radix(line.unwrap().trim(), 10).unwrap();

        //To store in array
        elems.push(elem);

        //Store sums in array
        sum += elem;
        sums.push(sum);
        if sum == 0{
            min_kv = cmp::min(min_kv, kv{key: i as i64, value: 0});
        }
    }
    let n = elems.len() as i64;
    for (i, si) in sums.iter().enumerate(){
        for (j, sj) in sums.iter().enumerate(){
            let i = i as i64;
            let j = j as i64;
            if i == j {continue;}
            if si == sj && sum == 0{
                min_kv = cmp::min(min_kv, kv{key: cmp::min(i, j) + n, value: *si});
            } else {
                if i64::signum(sj - si) == i64::signum(sum) && (sj - si)%sum == 0{
                    min_kv = cmp::min(min_kv, kv{key: (sj - si)/sum*n + i, value: *sj});
                }
            }
            
        }
    }
    println!("{}", min_kv.value);
    Ok(())
}
