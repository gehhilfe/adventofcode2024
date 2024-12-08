use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::cmp::Ord
        + Copy,
{
    fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn div(&self, other: T) -> Self {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

struct Antenna {
    position: Vec2<i32>,
    freq: char,
}

impl Antenna {
    fn new(position: Vec2<i32>, freq: char) -> Self {
        Antenna { position, freq }
    }
}

struct Map {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<Antenna>>,
    antinodes: HashSet<Vec2<i32>>,
}

impl Map {
    // Read the map from a file
    // Every non . char is an antenna where the char is the frequency
    fn from_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        let mut antennas = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in content.lines().enumerate() {
            height = y as i32;
            for (x, c) in line.chars().enumerate() {
                width = x as i32;
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert(Vec::new())
                        .push(Antenna::new(Vec2::new(x as i32, y as i32), c));
                }
            }
        }
        Map {
            width: width + 1,
            height: height + 1,
            antennas,
            antinodes: HashSet::new(),
        }
    }

    // Calculate the antinodes
    fn calculate_antinodes(&mut self) {
        for antennas in self.antennas.values() {
            for pairs in antennas.into_iter().combinations(2) {
                let a = pairs[0];
                let b = pairs[1];
                let delta = a.position.sub(&b.position);
                let antinode = b.position.sub(&delta);
                let other_antinode = a.position.add(&delta);
                self.antinodes.insert(antinode);
                self.antinodes.insert(other_antinode);
            }
        }
    }

    // Calculate the antinodes
    fn calculate_antinodes_part_2(&mut self) {
        for antennas in self.antennas.values() {
            for pairs in antennas.into_iter().combinations(2) {
                let a = pairs[0];
                let b = pairs[1];
                let delta = a.position.sub(&b.position);
                let mut antinode = b.position.sub(&delta);
                while self.in_bounds(&antinode) {
                    self.antinodes.insert(antinode);
                    antinode = antinode.sub(&delta);
                }
                let mut other_antinode = a.position.add(&delta);
                while self.in_bounds(&other_antinode) {
                    self.antinodes.insert(other_antinode);
                    other_antinode = other_antinode.add(&delta);
                }
            }
        }
        for (_freq, antennas) in &self.antennas {
            if antennas.len() > 1 {
                for antenna in antennas {
                    self.antinodes.insert(antenna.position);
                }
            }
        }
    }

    fn in_bounds(&self, position: &Vec2<i32>) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }

    fn antinodes_within_bounds(&self) -> i32 {
        let mut sum = 0;
        for antinode in &self.antinodes {
            if self.in_bounds(antinode) {
                sum += 1;
            }
        }
        sum
    }
    
    fn antinodes_within_bounds_part_2(&self) -> i32 {
        let mut sum = self.antinodes_within_bounds();
        for (freq, antenna) in &self.antennas {
            if antenna.len() > 1 {
                sum += (antenna.len()) as i32;
            }
        }
        sum
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut found = false;
                for antennas in self.antennas.values() {
                    for antenna in antennas {
                        if antenna.position.x == x && antenna.position.y == y {
                            write!(f, "{}", antenna.freq)?;
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    continue;
                }
                for antinode in &self.antinodes {
                    if antinode.x == x && antinode.y == y {
                        write!(f, "#")?;
                        found = true;
                        break;
                    }
                }
                if !found {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut map = Map::from_file("input.txt");
    println!("Map width: {}", map.width);
    println!("Map height: {}", map.height);
    map.calculate_antinodes();

    println!("{}", map);
    println!("Antinodes within bounds: {}", map.antinodes_within_bounds());

    let mut map = Map::from_file("input.txt");
    map.calculate_antinodes_part_2();

    println!("{}", map);
    println!("Antinodes within bounds: {}", map.antinodes_within_bounds());
}
