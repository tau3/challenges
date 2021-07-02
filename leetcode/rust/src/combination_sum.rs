struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let result = combination(&candidates, vec![], 0, target);
        prune(result)
    }
}

fn prune(mut combinations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for combo in combinations.iter_mut() {
        combo.sort_unstable()
    }
    let distinct: std::collections::HashSet<Vec<i32>> = combinations.drain(..).collect();
    combinations.extend(distinct.into_iter());
    combinations
}

fn combination(
    candidates: &[i32],
    combo: Vec<i32>,
    current_sum: i32,
    target: i32,
) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for num in candidates {
        let new_sum = num + current_sum;
        match new_sum.cmp(&target) {
            std::cmp::Ordering::Less => {
                let mut combo = combo.clone();
                combo.push(*num);
                let mut sub_results = combination(candidates, combo, new_sum, target);
                result.append(&mut sub_results);
            }
            std::cmp::Ordering::Equal => {
                let mut combo = combo.clone();
                combo.push(*num);
                result.push(combo);
            }
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]]
                .into_iter()
                .collect::<HashSet<Vec<i32>>>(),
            Solution::combination_sum(vec![2, 3, 6, 7], 7)
                .into_iter()
                .collect::<HashSet<Vec<i32>>>()
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
                .into_iter()
                .collect::<HashSet<Vec<i32>>>(),
            Solution::combination_sum(vec![2, 3, 5], 8)
                .into_iter()
                .collect::<HashSet<Vec<i32>>>()
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Vec::new() as Vec<Vec<i32>>,
            Solution::combination_sum(vec![2], 1)
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![vec![1]], Solution::combination_sum(vec![1], 1))
    }

    #[test]
    fn test_5() {
        assert_eq!(vec![vec![1, 1]], Solution::combination_sum(vec![1], 2))
    }
}
