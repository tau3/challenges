use std::collections::{BTreeSet, HashMap};

pub struct MovieRentingSystem {
    absent: BTreeSet<(i32, i32, i32)>,           // price, shop, movie
    present: HashMap<i32, BTreeSet<(i32, i32)>>, // movie -> (price, shop)
    prices: HashMap<(i32, i32), i32>,            // (shop, movie) -> price
}

impl MovieRentingSystem {
    pub fn new(_: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut present = HashMap::new();
        let mut prices = HashMap::new();
        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            present
                .entry(movie)
                .or_insert_with(BTreeSet::new)
                .insert((price, shop));
            prices.insert((shop, movie), price);
        }
        Self {
            absent: BTreeSet::new(),
            present,
            prices,
        }
    }

    pub fn search(&self, movie: i32) -> Vec<i32> {
        let data = self.present.get(&movie).unwrap();
        data.iter().take(5).map(|(_, shop)| *shop).collect()
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(shop, movie)).unwrap();
        self.absent.insert((price, shop, movie));

        let data = self.present.get_mut(&movie).unwrap();
        data.remove(&(price, shop));
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(shop, movie)).unwrap();
        self.absent.remove(&(price, shop, movie));

        let data = self.present.get_mut(&movie).unwrap();
        data.insert((price, shop));
    }

    pub fn report(&self) -> Vec<Vec<i32>> {
        self.absent
            .iter()
            .take(5)
            .map(|(_, shop, movie)| vec![*shop, *movie])
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
