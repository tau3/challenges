pub struct Solution {}

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut state = [0; 1000];
        for trip in trips {
            let (count, from, to) = (trip[0], trip[1] as usize, trip[2] as usize);
            for i in from..=to {
                state[i] += count;
                if state[i] > capacity {
                    return false;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(!Solution::car_pooling(
            vec![vec![2, 1, 5], vec![3, 3, 7]],
            4
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
    }

    #[test]
    fn test3() {
        assert!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3));
    }
}
