#![allow(unused)]
mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
mod d21;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

use std::cell::Cell;
use std::time;

#[derive(Clone)]
pub struct Perf {
    start: time::Instant,
    last_ms: Cell<f64>,
}

impl Default for Perf {
    fn default() -> Self {
        Self {
            start: time::Instant::now(),
            last_ms: Cell::new(0.0),
        }
    }
}

impl Perf {
    pub fn print(&self, ident: &str) {
        let time_in_ms = self.start.elapsed().as_secs_f64() * 1_000.0;
        let since_last_ms = time_in_ms - self.last_ms.get();
        self.last_ms.set(time_in_ms);

        println!(
            "{} took: {:.4}ms | total {:.4}ms",
            ident, since_last_ms, time_in_ms,
        )
    }
}

fn main() {
    d21::main();
}
