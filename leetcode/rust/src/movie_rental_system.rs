use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};

type MovieId = i32;
type ShopId = i32;
type Price = i32;

struct MovieRentingSystem {
    shops: HashMap<ShopId, HashMap<MovieId, (i32, bool)>>,
}

impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut shops = HashMap::new();
        for entry in entries {
            let shop_id = entry[0];
            let movie_id = entry[1];
            let price = entry[2];
            shops
                .entry(shop_id)
                .or_insert_with(|| HashMap::new())
                .insert(movie_id, (price, true));
        }
        Self { shops }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let mut result = vec![];
        for (shop_id, shop_data) in self.shops.iter() {
            if let Some((price, is_present)) = shop_data.get(&movie) {
                if *is_present {
                    result.push((shop_id, price));
                }
            }
        }
        result.sort_by(|a, b| match a.1.cmp(b.1) {
            Less => Less,
            Greater => Greater,
            Equal => a.0.cmp(&b.0),
        });
        result.iter().take(5).map(|(shop_id, _)| **shop_id).collect()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let mut shop_data = &mut self.shops.get(&shop).unwrap();
        if let Some(x) = shop_data.get_mut(&movie) {
            *x = (x.0, false);
        } else {
            panic!("shop {} doesn't have movie {}", shop, movie);
        }
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let shop_data = &mut self.shops.get(&shop).unwrap();
        if let Some(x) = shop_data.get_mut(&movie) {
            *x = (x.0, true);
        } else {
            panic!("shop {} doesn't have movie {}", shop, movie);
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for (shop_id, movie_to_stats) in self.shops.iter() {
            for (movie_id, stats) in movie_to_stats {
                if stats.1 == false {
                    continue;
                }
                result.push((shop_id, movie_id, stats.0));
            }
        }

        result.sort_by(|a, b| match a.2.cmp(&b.2) {
            Less => Less,
            Greater => Greater,
            Equal => match a.0.cmp(&b.0) {
                Less => Less,
                Greater => Greater,
                Equal => a.1.cmp(&b.1),
            },
        });
        result
            .iter()
            .take(5)
            .map(|(shop_id, movie_id, _)| vec![**shop_id, **movie_id])
            .collect()
    }
}
