use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::item::Item;

pub fn rand(items: &Vec<Item>) -> u32 {
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(items.iter().map(|item| item.weight))
        .expect("random create weight failed");
    let index = dist.sample(&mut rng);
    let item = items.get(index).expect("random get item failed");

    item.id
}
