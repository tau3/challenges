use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        for i in 0..transactions.len() {
            let current = &transactions[i];
            let mut current_tokens = current.split(',');
            let current_name = current_tokens.next().unwrap();
            let current_time: i16 = current_tokens.next().unwrap().parse().unwrap();
            let current_amount: u16 = current_tokens.next().unwrap().parse().unwrap();
            if current_amount > 1000 {
                result.insert(i);
            }
            let current_city = current_tokens.next().unwrap();
            for j in i + 1..transactions.len() {
                let other = &transactions[j];
                let mut other_tokens = other.split(',');
                let other_name = other_tokens.next().unwrap();
                if other_name == current_name {
                    let other_time: i16 = other_tokens.next().unwrap().parse().unwrap();
                    if (other_time - current_time).abs() <= 60 {
                        let _ = other_tokens.next();
                        let other_city = other_tokens.next().unwrap();
                        if other_city != current_city {
                            result.insert(i);
                            result.insert(j);
                        }
                    }
                }
            }
        }

        result
            .into_iter()
            .map(|i| &transactions[i])
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq_any_order(
            vec!["alice,50,1200,mtv".to_string()],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string(),
            ]),
        );
    }

    #[test]
    fn test2() {
        assert_eq_any_order(
            vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,beijing".to_string(),
            ],
            Solution::invalid_transactions(vec![
                "alice,50,100,beijing".to_string(),
                "alice,20,800,mtv".to_string(),
            ]),
        );
    }

    #[test]
    fn test3() {
        assert_eq_any_order(
            vec!["alice,50,1200,mtv".to_string()],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string(),
            ]),
        );
    }

    #[test]
    fn test4() {
        assert_eq_any_order(
            vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,mtv".to_string(),
                "alice,51,100,frankfurt".to_string(),
            ],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,mtv".to_string(),
                "alice,51,100,frankfurt".to_string(),
            ]),
        );
    }

    #[test]
    fn test5() {
        assert_eq_any_order(
            vec![
                "alice,20,1220,mtv".to_string(),
                "alice,20,1220,mtv".to_string(),
            ],
            Solution::invalid_transactions(vec![
                "alice,20,1220,mtv".to_string(),
                "alice,20,1220,mtv".to_string(),
            ]),
        );
    }

    fn assert_eq_any_order(mut left: Vec<String>, mut right: Vec<String>) {
        if left.len() != right.len() {
            panic!("different len: {:?} and {:?}", left, right);
        }

        left.sort();
        right.sort();

        let mut result = true;
        for i in 0..left.len() {
            if left[i] != right[i] {
                result = false;
                break;
            }
        }

        if !result {
            panic!("{:?} != {:?}", left, right);
        }
    }
}
