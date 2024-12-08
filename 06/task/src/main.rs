use core::fmt;
use std::{collections::HashSet, path::Path};

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct Vector2<T> {
    x: T,
    y: T,
}

enum RotateDirection {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl<T> Vector2<T>
where
    T: Copy + std::ops::Neg<Output = T> + std::ops::Add<Output = T>,
{
    fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }

    fn add(&self, other: &Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn rotate(&self, direction: RotateDirection) -> Vector2<T> {
        match direction {
            RotateDirection::Right => Vector2 {
                x: -self.y,
                y: self.x,
            },
            RotateDirection::Left => Vector2 {
                x: self.y,
                y: -self.x,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_add() {
        let a = Vector2::new(1, 2);
        let b = Vector2::new(3, 4);
        let c = a.add(&b);
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);
    }

    #[test]
    fn test_vector2_rotate() {
        let a = Vector2::new(0, -1);
        let b = a.rotate(RotateDirection::Right);
        assert_eq!(b.x, 1);
        assert_eq!(b.y, 0);
        let c = b.rotate(RotateDirection::Right);
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 1);
        let d = c.rotate(RotateDirection::Right);
        assert_eq!(d.x, -1);
        assert_eq!(d.y, 0);
        let e = d.rotate(RotateDirection::Left);
        assert_eq!(e.x, 0);
        assert_eq!(e.y, 1);
        let f = e.rotate(RotateDirection::Left);
        assert_eq!(f.x, 1);
        assert_eq!(f.y, 0);
        let g = f.rotate(RotateDirection::Left);
        assert_eq!(g.x, 0);
        assert_eq!(g.y, -1);
    }
}

#[derive(Clone)]
struct Guard {
    position: Vector2<i32>,
    direction: Vector2<i32>,
}

impl Guard {
    fn step(&mut self, map: &Map) {
        let next_position = self.position.add(&self.direction);
        if map.tile_at(&next_position).is_empty() {
            self.position = next_position;
        } else {
            self.direction = self.direction.rotate(RotateDirection::Right);
            self.step(map);
        }
    }

    fn orientation(&self) -> Orientation {
        if self.direction.x == 0 {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        }
    }
}

#[derive(Clone)]
enum Tile {
    Empty,
    EmptyButVisited(Vec<Orientation>),
    Occupied,
    OccupiedInserted,
}

impl Tile {
    fn is_empty(&self) -> bool {
        match self {
            Tile::Empty => true,
            Tile::EmptyButVisited(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::EmptyButVisited(orientations) => {
                if orientations.contains(&Orientation::Horizontal)
                    && orientations.contains(&Orientation::Vertical)
                {
                    write!(f, "+")
                } else if orientations.contains(&Orientation::Horizontal) {
                    write!(f, "-")
                } else {
                    write!(f, "|")
                }
            }
            Tile::Occupied => {
                write!(f, "#")
            }
            Tile::OccupiedInserted => {
                write!(f, "O")
            }
        }
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LoopGuard {
    position: Vector2<i32>,
    direction: Vector2<i32>,
}

impl Map {
    fn tile_at(&self, position: &Vector2<i32>) -> &Tile {
        // if oob, return empty
        if position.x < 0
            || position.x >= self.width as i32
            || position.y < 0
            || position.y >= self.height as i32
        {
            return &Tile::Empty;
        }
        &self.tiles[position.y as usize][position.x as usize]
    }

    fn set_tile(&mut self, position: &Vector2<i32>, tile: Tile) {
        // if oob, do nothing
        if position.x < 0
            || position.x >= self.width as i32
            || position.y < 0
            || position.y >= self.height as i32
        {
            return;
        }
        self.tiles[position.y as usize][position.x as usize] = tile;
    }
}

#[derive(Clone)]
struct Simulation {
    map: Map,
    guard: Guard,
    guard_starting_position: Vector2<i32>,
}

impl Simulation {
    fn from_file(path: &Path) -> Simulation {
        let contents = std::fs::read_to_string(path).unwrap();
        let mut tiles = vec![];
        let mut guard = Guard {
            position: Vector2::new(0, 0),
            direction: Vector2::new(0, 1),
        };
        for line in contents.lines() {
            let mut row = vec![];
            for c in line.chars() {
                match c {
                    '.' => row.push(Tile::Empty),
                    '#' => row.push(Tile::Occupied),
                    '^' => {
                        row.push(Tile::Empty);
                        guard.position = Vector2::new(row.len() as i32, tiles.len() as i32);
                        guard.direction = Vector2::new(0, -1);
                    }
                    'v' => {
                        row.push(Tile::Empty);
                        guard.position = Vector2::new(row.len() as i32, tiles.len() as i32);
                        guard.direction = Vector2::new(0, 1);
                    }
                    '<' => {
                        row.push(Tile::Empty);
                        guard.position = Vector2::new(row.len() as i32, tiles.len() as i32);
                        guard.direction = Vector2::new(-1, 0);
                    }
                    '>' => {
                        row.push(Tile::Empty);
                        guard.position = Vector2::new(row.len() as i32, tiles.len() as i32);
                        guard.direction = Vector2::new(1, 0);
                    }
                    _ => panic!("Invalid character in input file"),
                }
            }
            tiles.push(row);
        }
        guard.position = guard.position.add(&Vector2 { x: -1, y: 0 });
        Simulation {
            map: Map {
                width: tiles[0].len(),
                height: tiles.len(),
                tiles,
            },
            guard_starting_position: guard.position.clone(),
            guard,
        }
    }

    fn step(&mut self) {
        let tile = self.map.tile_at(&self.guard.position);
        match tile {
            Tile::EmptyButVisited(orientations) => {
                if !orientations.contains(&self.guard.orientation()) {
                    let mut new_orientations = orientations.clone();
                    new_orientations.push(self.guard.orientation());
                    self.map.set_tile(
                        &self.guard.position,
                        Tile::EmptyButVisited(new_orientations),
                    );
                }
            }
            _ => {
                self.map.set_tile(
                    &self.guard.position,
                    Tile::EmptyButVisited(vec![self.guard.orientation()]),
                );
            }
        }
        self.guard.step(&self.map);
    }

    fn run_until_guard_oob(&mut self) -> Vec<LoopGuard> {
        let mut path :Vec<LoopGuard> = vec![];
        loop {
            if self.guard.position.x < 0
                || self.guard.position.x >= self.map.width as i32
                || self.guard.position.y < 0
                || self.guard.position.y >= self.map.height as i32
            {
                return path;
            }
            path.push(LoopGuard {
                position: self.guard.position,
                direction: self.guard.direction,
            });
            self.step();
        }
    }

    fn run_until_loop(&mut self, mut previous_positions: HashSet<LoopGuard>) -> bool {
        loop {
            previous_positions.insert(LoopGuard {
                position: self.guard.position,
                direction: self.guard.direction,
            });
            self.step();
            if self.guard.position.x < 0
                || self.guard.position.x >= self.map.width as i32
                || self.guard.position.y < 0
                || self.guard.position.y >= self.map.height as i32
            {
                return false;
            }
            if previous_positions.contains(&LoopGuard {
                position: self.guard.position,
                direction: self.guard.direction,
            }) {
                println!("Loop detected");
                return true;
            }
        }
    }

    fn count_visited(&self) -> usize {
        self.map
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| match tile {
                        Tile::EmptyButVisited(_) => true,
                        _ => false,
                    })
                    .count()
            })
            .sum()
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.guard.position.x == x as i32 && self.guard.position.y == y as i32 {
                    write!(
                        f,
                        "{}",
                        match self.guard.direction {
                            Vector2 { x: 0, y: -1 } => '^',
                            Vector2 { x: 0, y: 1 } => 'v',
                            Vector2 { x: -1, y: 0 } => '<',
                            Vector2 { x: 1, y: 0 } => '>',
                            _ => panic!("Invalid direction"),
                        }
                    )?;
                } else {
                    write!(f, "{}", self.map.tiles[y][x])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut sim = Simulation::from_file(Path::new("input.txt"));
    println!("{}", sim);
    let mut visited_sim = sim.clone();
    let path = visited_sim.run_until_guard_oob();
    println!("Visited: {}", visited_sim.count_visited());

    // At every step of the path, we insert an obstacle and try to find a loop
    let (tx, rx) = std::sync::mpsc::channel();
    for g in path {
        if g.position == sim.guard_starting_position {
            continue;
        }
        let tx = tx.clone();
        let mut test_sim = sim.clone();
        std::thread::spawn(move || {
            if g.position == sim.guard_starting_position {
                return;
            }
            test_sim.map.set_tile(&g.position, Tile::OccupiedInserted);
            if test_sim.run_until_loop(HashSet::new()) {
                tx.send(g.position).unwrap();
            }
        });
    }
    drop(tx);
    let mut positions = HashSet::new();
    for p in rx {
        positions.insert(p);
    }

    println!("Loop positions: {:?}", positions.len());
}
