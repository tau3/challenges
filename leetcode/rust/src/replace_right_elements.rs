pub struct Solution {}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let len = arr.len();
        let mut result = vec![-1; len];
        let mut max = arr[len - 1];
        for i in (0..len - 1).rev() {
            max = std::cmp::max(max, arr[i + 1]);
            result[i] = max;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec![18, 6, 6, 6, 1, -1],
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1])
        );
    }

    #[test]
    fn test_single_elemnt() {
        assert_eq!(vec![-1], Solution::replace_elements(vec![400]));
    }
}
