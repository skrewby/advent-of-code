use std::time::{Duration, Instant};

pub struct Solution {
    pub part1: String,
    pub part2: String,
    start: Instant,
    pub duration: Duration 
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            part1: "Not complete".to_string(),
            part2: "Not complete".to_string(),
            start: Instant::now(),
            duration: Duration::new(0, 0)
        }
    }

    pub fn complete(&mut self) {
        self.duration = self.start.elapsed();
    }
}
