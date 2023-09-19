use std::{cmp::Ordering, collections::HashMap};

type MovieId = i32;
type ShopId = i32;
type Price = i32;

struct MovieRentingSystem {
    state: Vec<Vec<i32>>,
    shops: HashMap<ShopId, HashMap<MovieId, (i32, bool)>>,
    s: Vec<(i32, i32)>,
}

impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        Self {
            state: entries,
            shops: n,
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let mut result = vec![];
        for (k, v) in self.shops {
            if let Some((price, is_present)) = v.get(&movie) {
                if *is_present {
                    result.push((k, price));
                }
            }
        }
        result.sort_by(|a, b| match a.1.cmp(b.1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => a.0.cmp(b.0),
        });
        result.iter().take(5).map(|(a, b)| a).collect()
    }

    fn rent(&self, shop: i32, movie: i32) {
        let mut shop_data = self.shops.get(&shop).unwrap();
        if let Some(x) = shop_data.get_mut(&movie) {
            *x = (x.0, false);
        } else {
            panic!("shop {} doesn't have movie {}", shop, movie);
        }
    }

    fn drop(&self, shop: i32, movie: i32) {
        let mut shop_data = self.shops.get(&shop).unwrap();
        if let Some(x) = shop_data.get_mut(&movie) {
            *x = (x.0, true);
        } else {
            panic!("shop {} doesn't have movie {}", shop, movie);
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for (shop_id, movie_to_stats) in self.shops {
            for (movie_id, stats) in movie_to_stats {
                if stats.1 == false {
                    continue;
                }
                result.push(vec![shop_id, movie_id, stats.0]);
            }
        }

        result.sort_by(|a, b| match a.2.cmp(b.2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match a.0.cmp(b.0) {
                Less => Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.1.cmp(b.1),
            },
        });
        result.iter().take(5).map(|(a, b, c)| b).collect()
    }
}
