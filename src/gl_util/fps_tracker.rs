use std::time::Instant;

pub struct FpsTracker {
    pub frames: usize,
    pub start: Instant
}

impl FpsTracker {
    pub fn new() -> FpsTracker {
        return FpsTracker {
            frames: 0,
            start: Instant::now()
        }
    }
    pub fn tick(&mut self) {
        if self.frames % 100 == 0 {
            self.frames = 1;
            println!("fps: {}", 1_000_000_000.0 / (self.start.elapsed().as_nanos() as f64 / 100.0));
            self.start = Instant::now();
        } else {
            self.frames += 1;
        }
    }
}