use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};

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
        result.sort_by(|(shop_left, price_left), (shop_right, price_right)| {
            match price_left.cmp(price_right) {
                Less => Less,
                Greater => Greater,
                Equal => shop_left.cmp(shop_right),
            }
        });
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
                if !is_present {
                    continue;
                }
                result.push((shop, movie, price));
            }
        }

        result.sort_by(|(shop1, movie1, price1), (shop2, movie2, price2)| {
            match price1.cmp(&price2) {
                Less => Less,
                Greater => Greater,
                Equal => match shop1.cmp(shop2) {
                    Less => Less,
                    Greater => Greater,
                    Equal => movie1.cmp(movie2),
                },
            }
        });
        result
            .iter()
            .take(5)
            .map(|(shop, movie, _)| vec![**shop, **movie])
            .collect()
    }
}
