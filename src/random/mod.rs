#[cfg(test)]
mod tests;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn rand(val_weight_items: &Vec<(u32, u32)>) -> (u32, u32) {
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(val_weight_items.iter().map(|item| item.1))
        .expect("random create weight failed");
    let index = dist.sample(&mut rng);
    let item = val_weight_items.get(index).expect("random get item failed");

    (item.0, item.1)
}
