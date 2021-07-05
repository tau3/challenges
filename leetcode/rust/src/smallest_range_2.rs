struct Solution {}

impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut max = nums[nums.len() - 1];
        let mut min = nums[0];
        let mut diff = max - min;

        for i in 0..nums.len() - 1 {
            max = std::cmp::max(nums[nums.len() - 1] - k, nums[i] + k);
            min = std::cmp::min(nums[0] + k, nums[i + 1] - k);
            diff = std::cmp::min(diff, max - min);
        }
        diff
    }
}

#[cfg(test)]
mod test {
    use crate::smallest_range_2::Solution;

    #[test]
    fn test_singleton() {
        assert_eq!(0, Solution::smallest_range_ii(vec![1], 0));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(6, Solution::smallest_range_ii(vec![0, 10], 2));
    }

    #[test]
    fn test_example_3() {
        assert_eq!(3, Solution::smallest_range_ii(vec![1, 3, 6], 3));
    }
}