use core::fmt;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::path::Path;
use std::fs::File;

fn checkNumbers(numbers: &Vec<i32>) -> bool {
    // Combine adjacent numbers with zip to iterate over pairs
    let pairs: Vec<(&i32, &i32)> = numbers.iter().zip(numbers.iter().skip(1)).collect();
    
    // Reduce the pairs to the difference between the two numbers
    let differences: Vec<i32> = pairs.iter().map(|(a, b)| *b - *a).collect();

    // Check if all differences have the same sign
    let sameSign = differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0);

    // Conver to absolute values
    let absDifferences: Vec<i32> = differences.iter().map(|&x| x.abs()).collect();

    // Check if all value are between 1 and 3
    let inRange = absDifferences.iter().all(|&x| x >= 1 && x <= 3);

    return sameSign && inRange;
}

fn checkWithUpToOneRemoved(numbers: Vec<i32>) -> bool {
    if checkNumbers(&numbers) {
        return true;
    }
    let mut numbers = numbers.clone();
    for i in 0..numbers.len() {
        let mut numbers = numbers.clone();
        numbers.remove(i);
        if checkNumbers(&numbers) {
            return true;
        }
    }
    return false;
}

fn main() {
    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut saveCount = 0;
    io::BufReader::new(file.try_clone().unwrap()).lines().for_each(|line| {
        let line = line.unwrap();
        println!("{}", line);

        let words: Vec<&str> = line.split(" ").collect();

        let numbers: Vec<i32> = words.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        if checkNumbers(&numbers) {
            saveCount += 1;
        }
    });
    std::println!("--- PART 1 ---");
    std::println!("{}", saveCount);

    // Skeek back to the beginning of the file
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut saveCount = 0;
    io::BufReader::new(file).lines().for_each(|line| {
        let line = line.unwrap();
        println!("{}", line);

        let words: Vec<&str> = line.split(" ").collect();

        // First parse every word in the line to i32
        let numbers: Vec<i32> = words.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        if checkWithUpToOneRemoved(numbers) {
            saveCount += 1;
        }
    });


    std::println!("--- PART 2 ---");
    std::println!("{}", saveCount);
}
