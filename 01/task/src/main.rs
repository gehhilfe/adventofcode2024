use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("input.txt");

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };


    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    io::BufReader::new(file).lines().for_each(|line| {
        let line = line.unwrap();
        println!("{}", line);

        let words: Vec<&str> = line.split("   ").collect();
        let av = words[0].parse::<i32>().unwrap();
        let bv = words[1].parse::<i32>().unwrap();

        a.push(av);
        b.push(bv);
    });

    a.sort();
    b.sort();

    let mut sum = 0;
    for i in 0..a.len() {
        sum += (a[i] - b[i]).abs();
    }
    println!("{}", sum);

    // --- Part Two ---

    let mut sum = 0;
    for i in 0..a.len() {
        let mut found = 0;
        for j in 0..b.len() {
            if a[i] == b[j] {
                found += 1;
            }
        }
        sum += a[i] * found;
    }
    println!("{}", sum);
}
