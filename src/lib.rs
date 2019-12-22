use rand::prelude::*;
use std::io::Write;

const THREADS: usize = 4;
#[derive(Copy, Debug, Clone)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn as_num(self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Alive => 1,
        }
    }
    pub fn is_alive(self) -> bool {
        match self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub world: Vec<Cell>,
    pub width: u32,
    pub height: u32,
    pub rule: Rule,
}

impl Model {
    pub fn random(width: u32, height: u32, rule: Rule) -> Self {
        let mut rng = rand::thread_rng();
        let world = (0..width * height)
            .map(|_| {
                if rng.gen::<bool>() {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Model {
            world,
            width,
            height,
            rule,
        }
    }
    fn at(&self, x: u32, y: u32) -> Cell {
        self.world[(y * self.width + x) as usize]
    }
    pub fn neighbours_of(&self, loc: u32) -> u8 {
        // 累積和で書き直す
        let mut ns = 0;
        let (x, y) = (loc % self.width, loc / self.width);
        for &neighbour_x in &[
            x.checked_sub(1).unwrap_or(self.width - 1),
            x,
            (x + 1) % self.width,
        ] {
            for &neighbour_y in &[
                y.checked_sub(1).unwrap_or(self.height - 1),
                y,
                (y + 1) % self.height,
            ] {
                ns += self.at(neighbour_x, neighbour_y).as_num();
            }
        }
        ns -= self.world[loc as usize].as_num();
        ns
    }

    pub fn update_self(&mut self) -> () {
        let current = self.clone();
        for (i, &cell) in current.world.iter().enumerate() {
            let neighbours = current.neighbours_of(i as u32);
            if cell.is_alive() {
                if neighbours < self.rule.alive_min || self.rule.alive_max < neighbours {
                    self.world[i] = Cell::Dead;
                }
            } else {
                // for dead cells
                if self.rule.birth_min <= neighbours && neighbours <= self.rule.birth_max {
                    self.world[i] = Cell::Alive;
                }
            }
        }
    }

    pub fn print_stdout(&self) {
        let mut reader = self.world.iter();
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        for _ in 0..self.height {
            for _ in 0..self.width {
                handle
                    .write_all(match reader.next().unwrap() {
                        Cell::Alive => b"X",
                        Cell::Dead => b" ",
                    })
                    .unwrap();
            }
            handle.write_all(b"\n").unwrap();
        }
        handle.write_all(b"\n").unwrap();
        handle.flush().unwrap();
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Rule {
    /// for a Cell::Dead, if birth_min <= neighbour <= birth_max then new cell is born
    /// for a Cell::Alive, if alive_min <= neighbour <= alive_max then it stays alive
    pub birth_min: u8,
    pub birth_max: u8,
    pub alive_min: u8,
    pub alive_max: u8,
}
