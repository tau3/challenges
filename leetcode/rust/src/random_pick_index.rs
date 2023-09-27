use std::collections::HashMap;
use rand::seq::SliceRandom;

pub struct Solution {
    counters: HashMap<i32, Vec<usize>>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut counters: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, val) in nums.iter().enumerate() {
            let value = counters.entry(*val).or_default();
            value.push(i);
        }

        Self { counters }
    }

    pub fn pick(&self, target: i32) -> i32 {
        let indexes = &self.counters[&target];
        *indexes.choose(&mut rand::thread_rng()).unwrap() as i32
    }
}
