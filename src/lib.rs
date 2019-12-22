use std::io::Write;

use js_sys;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod wasm_util;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)] // bool is not understood
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

fn coin_flip_js() -> bool {
    js_sys::Math::random() < 0.5
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Model {
    world: Vec<Cell>,
    pub width: u32,
    pub height: u32,
    last_update_count: u32,
    rule: Rule,
}

#[wasm_bindgen]
impl Model {
    pub fn random(width: u32, height: u32) -> Self {
        let world = (0..width * height)
            .map(|_| {
                if coin_flip_js() {
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
            last_update_count: 0,
            rule: RULE,
        }
    }

    pub fn world(&mut self) -> *const Cell {
        self.world.as_ptr()
    }

    pub fn update_and_report(&mut self) -> *const i32 {
        // update itself, and reports where the status changes
        let current = self.clone();
        self.last_update_count = 0;
        let mut updates = Vec::new();
        for (i, &cell) in current.world.iter().enumerate() {
            let neighbours = current.neighbours_of(i as u32);
            if cell.is_alive() {
                if neighbours < self.rule.alive_min || self.rule.alive_max < neighbours {
                    self.world[i] = Cell::Dead;
                    updates.push(-((i + 1) as i32));
                    self.last_update_count += 1;
                }
            } else {
                // for dead cells
                if self.rule.birth_min <= neighbours && neighbours <= self.rule.birth_max {
                    self.world[i] = Cell::Alive;
                    updates.push((i + 1) as i32);
                    self.last_update_count += 1;
                }
            }
        }
        updates.as_ptr()
    }

    pub fn tell_last_update_count(&self) -> u32 {
        self.last_update_count
    }
    pub fn w(&self) -> u32 {
        self.width
    }
    pub fn h(&self) -> u32 {
        self.height
    }
}

impl Model {
    fn at(&self, x: u32, y: u32) -> Cell {
        self.world[(y * self.width + x) as usize]
    }
    pub fn neighbours_of(&self, loc: u32) -> u8 {
        let mut ns = 0;
        let (x, y) = (loc % self.width, loc / self.width);
        for &dx in &[self.width - 1, 0, 1] {
            for &dy in &[self.height - 1, 0, 1] {
                ns += self
                    .at((x + dx) % self.width, (y + dy) % self.height)
                    .as_num();
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

#[wasm_bindgen]
#[derive(Copy, Debug, Clone)]
pub struct Rule {
    /// for a Cell::Dead, if birth_min <= neighbour <= birth_max then new cell is born
    /// for a Cell::Alive, if alive_min <= neighbour <= alive_max then it stays alive
    pub birth_min: u8,
    pub birth_max: u8,
    pub alive_min: u8,
    pub alive_max: u8,
}

const RULE: Rule = Rule {
    birth_min: 3,
    birth_max: 3,
    alive_min: 2,
    alive_max: 3,
};
