use std::{fs::File, io::Read, path::Path};

use regex::Regex;

fn main() {

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    let mut sum = 0;
    for cap in re.captures_iter(&contents) {
        let a: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        sum += a * b;
    }
    println!("Part 1");
    println!("Sum: {}", sum);

    // --- PART 2 ---

    let mut sum = 0;
    for cap in re.captures_iter(&contents) {

        let start = cap.get(0).unwrap().start();
        // check where this first do() is before start()
        let do_pos = contents[..start].rfind("do()").unwrap_or(0);
        let dont_pos = contents[..start].rfind("don't()").unwrap_or(0);

        // is enabled when do() is more recent than dont()
        if do_pos >= dont_pos {
            let a: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            sum += a * b;
        }
    }

    println!("Part 2");
    println!("Sum: {}", sum);

}
