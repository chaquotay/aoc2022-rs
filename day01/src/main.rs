use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let mut acc = 0;
    let mut vec : Vec::<i32> = Vec::new();

    for line in text.lines() {
        if line=="" {
            vec.push(acc);
            acc = 0;
        } else {
            let val : i32 = line.parse().unwrap();
            acc += val;
        }
    }

    vec.sort_by(|a, b| b.cmp(a));

    let first = vec[0];
    println!("Top 1: {}", first);

    let top3 : i32 = vec.into_iter().take(3).sum();
    println!("Top 3: {}", top3);
}
