use std::time::Instant;

pub struct Solution {
    pub part1: String,
    pub part2: String,
    start: Instant,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            part1: "Not complete".to_string(),
            part2: "Not complete".to_string(),
            start: Instant::now(),
        }
    }

    pub fn get_time(&self) -> u128 {
        return self.start.elapsed().as_micros();
    }
}
