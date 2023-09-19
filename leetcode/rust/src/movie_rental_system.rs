use std::collections::HashMap;

pub struct MovieRentingSystem {
    shops: HashMap<i32, HashMap<i32, (i32, bool)>>,
}

impl MovieRentingSystem {
    pub fn new(_: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut shops = HashMap::new();
        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            shops
                .entry(shop)
                .or_insert_with(HashMap::new)
                .insert(movie, (price, true));
        }
        Self { shops }
    }

    pub fn search(&self, movie: i32) -> Vec<i32> {
        let mut result = vec![];
        for (shop, shop_data) in self.shops.iter() {
            if let Some((price, is_present)) = shop_data.get(&movie) {
                if *is_present {
                    result.push((shop, price));
                }
            }
        }
        result
            .sort_by(|(shop1, price1), (shop2, price2)| price1.cmp(price2).then(shop1.cmp(shop2)));
        result.iter().take(5).map(|(shop, _)| **shop).collect()
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let shop_data = &mut self.shops.get_mut(&shop).unwrap();
        if let Some(entry) = shop_data.get_mut(&movie) {
            let price = entry.0;
            *entry = (price, false);
        } else {
            panic!("shop {} doesn't have movie {}", shop, movie);
        }
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let shop_data = &mut self.shops.get_mut(&shop).unwrap();
        if let Some(entry) = shop_data.get_mut(&movie) {
            let price = entry.0;
            *entry = (price, true);
        } else {
            panic!("shop {} doesn't have movie {}", shop, movie);
        }
    }

    pub fn report(&self) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for (shop, movie_to_stats) in self.shops.iter() {
            for (movie, (price, is_present)) in movie_to_stats {
                if *is_present {
                    continue;
                }
                result.push((shop, movie, price));
            }
        }

        result.sort_by(|(shop1, movie1, price1), (shop2, movie2, price2)| {
            price1
                .cmp(price2)
                .then(shop1.cmp(shop2).then(movie1.cmp(movie2)))
        });
        result
            .iter()
            .take(5)
            .map(|(shop, movie, _)| vec![**shop, **movie])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut system = MovieRentingSystem::new(
            3,
            vec![
                vec![0, 1, 5],
                vec![0, 2, 6],
                vec![0, 3, 7],
                vec![1, 1, 4],
                vec![1, 2, 7],
                vec![2, 1, 5],
            ],
        );
        assert_eq!(vec![1, 0, 2], system.search(1)); // return [1, 0, 2], Movies of ID 1 are unrented at shops 1, 0, and 2. Shop 1 is cheapest; shop 0 and 2 are the same price, so order by shop number.
        system.rent(0, 1); // Rent movie 1 from shop 0. Unrented movies at shop 0 are now [2,3].
        system.rent(1, 2); // Rent movie 2 from shop 1. Unrented movies at shop 1 are now [1].
        assert_eq!(vec![vec![0, 1], vec![1, 2]], system.report()); // return [[0, 1], [1, 2]]. Movie 1 from shop 0 is cheapest, followed by movie 2 from shop 1.
        system.drop(1, 2); // Drop off movie 2 at shop 1. Unrented movies at shop 1 are now [1,2].
        assert_eq!(vec![0, 1], system.search(2)); // return [0, 1]. Movies of ID 2 are unrented at shops 0 and 1. Shop 0 is cheapest, followed by shop 1.
    }
}
