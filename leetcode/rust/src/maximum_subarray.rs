use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut current = nums[0];

        for i in 1..nums.len() {
            current = max(nums[i], current + nums[i]);
            result = max(result, current)
        }

        result
    }
}

struct Solution {}
