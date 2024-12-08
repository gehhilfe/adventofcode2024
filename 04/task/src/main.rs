use std::path::Path;

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        Map {
            width,
            height,
            map: vec![vec!['.'; width]; height],
        }
    }

    fn from_file(path: &Path) -> Map {
        let contents = std::fs::read_to_string(path).unwrap();
        let mut lines = contents.lines();
        let width = lines.next().unwrap().len();
        let height = contents.lines().count();
        let mut map = Map::new(width, height);
        for (y, line) in contents.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.map[y][x] = c;
            }
        }
        map
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y >= self.height as i32 {
            return None;
        }
        if x < 0 || x >= self.width as i32 {
            return None;
        }
        return Some(self.map[y as usize][x as usize]);
    }

    fn test_star(&self, x: i32, y: i32) -> i32 {
        return self.test(x, y, 1, 0)
        + self.test(x, y, -1, 0)
        + self.test(x, y, 0, 1)
        + self.test(x, y, 0, -1) 
        + self.test(x, y, 1, 1) 
        + self.test(x, y, 1, -1)
        + self.test(x, y, -1, -1) 
        + self.test(x, y, -1, 1)
    }

    fn test(&self, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
        let candidate = [self.get(x, y), self.get(x + dx, y + dy), self.get(x + 2 * dx, y + 2 * dy), self.get(x + 3 * dx, y + 3 * dy)];
        if candidate.iter().any(|&c| c == None) {
            return 0;
        }

        if candidate.iter().map(|f| f.unwrap()).collect::<String>() == "XMAS" {
            1
        } else {
            0
        }
    }
    
    fn count_xmas(&self) -> i32 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y][x] == 'X' {
                    count += self.test_star(x as i32, y as i32);
                }
            }
        }
        count
    }

    fn test_cross_xmas(&self, x: i32, y: i32) -> bool {
        let topr = self.get(x+1, y-1).unwrap_or('_');
        let topl = self.get(x-1, y-1).unwrap_or('_');
        let botr = self.get(x+1, y+1).unwrap_or('_');
        let botl = self.get(x-1, y+1).unwrap_or('_');

        if topr == 'M' && botl == 'S' && (topl == 'M' && botr == 'S' || topl == 'S' && botr == 'M') {
            return true;
        }

        if topr == 'S' && botl == 'M' && (topl == 'M' && botr == 'S' || topl == 'S' && botr == 'M') {
            return true;
        }

        false
    }

    fn count_cross_xmas(&self) -> i32 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y][x] == 'A' {
                    if self.test_cross_xmas(x as i32, y as i32) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let path = Path::new("input.txt");

    let m = Map::from_file(&path);

    // --- Part One ---
    let c = m.count_xmas();
    println!("Part One");
    println!("XMAS count: {}", c);

    // --- Part Two ---
    println!("Part Two");
    println!("Cross XMAS count: {}", m.count_cross_xmas());
}
