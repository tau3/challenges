use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut result: HashSet<String> = HashSet::new();
        for i in 0..transactions.len() {
            let current = &transactions[i];
            let mut current_tokens = current.split(',');
            let current_name = current_tokens.next().unwrap();
            let current_time: i16 = current_tokens.next().unwrap().parse().unwrap();
            let current_amount: u16 = current_tokens.next().unwrap().parse().unwrap();
            if current_amount > 1000 {
                result.insert(current.clone());
            }
            for j in i + 1..transactions.len() {
                let other = &transactions[j];
                let mut other_tokens = other.split(',');
                let other_name = other_tokens.next().unwrap();
                if other_name == current_name {
                    let other_time: i16 = other_tokens.next().unwrap().parse().unwrap();
                    if (other_time - current_time).abs() <= 60 {
                        let current_city = current_tokens.next().unwrap();
                        let _ = other_tokens.next();
                        let other_city = other_tokens.next().unwrap();
                        if other_city != current_city {
                            result.insert(current.clone());
                            result.insert(other.clone());
                        }
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec!["alice,50,1200,mtv"],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string()
            ])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec!["alice,20,800,mtv", "alice,50,100,beijing"],
            Solution::invalid_transactions(vec![
                "alice,50,100,beijing".to_string(),
                "alice,20,800,mtv".to_string()
            ])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            vec!["alice,50,1200,mtv"],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string()
            ])
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            vec!["alice,50,1200,mtv"],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,mtv".to_string(),
                "alice,51,100,frankfurt".to_string()
            ])
        );
    }
}
