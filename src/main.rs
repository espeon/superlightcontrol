use enigo::*;
use err::SCError;
use key::parse;

use std::{cmp, thread, time::Duration};

use crate::key::execute;

mod err;
mod key;

const MS_SLEEP: u64 = 1;
const ITERATIONS: u32 = 20;
const ATTEMPTS: u8 = 100;

fn main() {
    match main_probably() {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }
}

fn main_probably() -> Result<(), SCError> {
    let mut enigo = Enigo::new();
    // almost definitely a better way to do this
    let up = parse(":ctrl + :up")?;
    let down = parse(":ctrl + :down")?;
    let left = parse(":ctrl + :left")?;
    let right = parse(":ctrl + :right")?;

    let mut counter = DirCounter::default();
    
    let mut attempts = 0;

    while counter.get_largest() == MouseDirection::None && attempts < ATTEMPTS {
        counter = count(&enigo, counter);
        attempts += 1
    }

    let lg = counter.get_largest();
    println!("final: {:?}", lg);

    let ex = match lg {
        MouseDirection::Up => up,
        MouseDirection::Down => down,
        MouseDirection::Left => left,
        MouseDirection::Right => right,
        MouseDirection::None => return Ok(()),
    };
    dbg!(ex.clone());
    execute(&mut enigo, ex);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MouseDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Default, Debug, Clone, Copy)]
struct DirCounter {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
}

impl DirCounter {
    fn count(mut self, d: &MouseDirection) -> DirCounter {
        match d {
            MouseDirection::Up => self.up += 1,
            MouseDirection::Down => self.down += 1,
            MouseDirection::Left => self.left += 1,
            MouseDirection::Right => self.right += 1,
            MouseDirection::None => {}
        }
        self
    }
    fn get_largest(self) -> MouseDirection {
        if std::cmp::max(
            self.up,
            std::cmp::max(self.down, std::cmp::max(self.left, self.right)),
        ) == 0
        {
            return MouseDirection::None;
        }
        let mut lg = MouseDirection::Up;
        let count = self.up;
        if count < self.down {
            lg = MouseDirection::Down
        }
        if count < self.left {
            lg = MouseDirection::Left
        }
        if count < self.right {
            lg = MouseDirection::Right
        }
        lg
    }
}

fn count(enigo: &Enigo, mut counter: DirCounter) -> DirCounter {
    let mut last = enigo.mouse_location();
    let mut last_dir = MouseDirection::None;

    println!("gwuh");

    for i in 0..ITERATIONS {
        let pos = enigo.mouse_location();

        // change in position from previous
        let delta_y = pos.0 - last.0;
        let delta_x = pos.1 - last.1;

        let dir: MouseDirection;

        if cmp::max(delta_x.abs(), delta_y.abs()) > 8 {
            if delta_x.abs() > delta_y.abs() {
                if delta_x < 0 {
                    dir = MouseDirection::Up;
                } else {
                    dir = MouseDirection::Down;
                }
            } else if delta_y < 0 {
                dir = MouseDirection::Left;
            } else {
                dir = MouseDirection::Right;
            }
        } else {
            dir = MouseDirection::None;
        }

        if dir != MouseDirection::None {
            counter = counter.count(&dir)
        }

        if dir != MouseDirection::None && last_dir != dir {
            dbg!(format!("{:?} | {delta_x} | {delta_y} | {i}", dir));
        }

        last = pos;
        last_dir = MouseDirection::None;
        thread::sleep(Duration::from_millis(MS_SLEEP))
    }
    counter
}
