mod d1;
mod d2;

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
    d2::main();
}
