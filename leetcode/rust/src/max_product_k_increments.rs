use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = nums.iter().map(|num| -num).collect();
        for _ in 0..k {
            let smallest = heap.pop().unwrap();
            heap.push(smallest - 1);
        }

        let modulo = 10_i64.pow(9) + 7;
        let mut result = 1_i64;
        for num in heap {
            result *= num as i64;
            result %= modulo;
        }

        (result % modulo).abs() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(20, Solution::maximum_product(vec![0, 4], 5));
        assert_eq!(216, Solution::maximum_product(vec![6, 3, 3, 2], 2));
    }

    #[test]
    fn test_large_result() {
        assert_eq!(
            180820950,
            Solution::maximum_product(vec![24, 5, 64, 53, 26, 38], 54)
        );
    }
}
