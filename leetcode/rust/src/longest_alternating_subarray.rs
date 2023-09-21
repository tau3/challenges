pub struct Solution {}

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut in_sequence = false;
        let mut current = 0;
        let mut next_expected = false;
        let mut max = 0;
        let mut i = 1;
        loop {
            if i >= nums.len() {
                break;
            }
            if !in_sequence {
                if nums[i] - nums[i - 1] == 1 {
                    in_sequence = true;
                    current = 2;
                }
            } else if next_expected {
                if nums[i] - nums[i - 1] == 1 {
                    current += 1;
                    next_expected = false;
                } else {
                    in_sequence = false;
                    max = std::cmp::max(max, current);
                }
            } else if nums[i - 1] - nums[i] == 1 {
                current += 1;
                next_expected = true;
            } else {
                in_sequence = false;
                max = std::cmp::max(max, current);
                i -= 1;
            }
            i += 1;
        }
        max = std::cmp::max(max, current);
        if max < 2 {
            max = -1
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::alternating_subarray(vec![2, 3, 4, 3, 4]));
        assert_eq!(2, Solution::alternating_subarray(vec![4, 5, 6]));
        assert_eq!(-1, Solution::alternating_subarray(vec![21, 9, 5]));
        assert_eq!(
            -1,
            Solution::alternating_subarray(vec![14, 30, 29, 49, 3, 23, 44, 21, 26, 52])
        );
    }
}
