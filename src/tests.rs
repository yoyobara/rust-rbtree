#![cfg(test)]

use rand::seq::SliceRandom;

use crate::set::RBTreeSet;

type IntegerSet = RBTreeSet<i32>;

#[test]
fn empty_set() {
    let set = IntegerSet::new();
    assert_eq!(set.size(), 0);
}

#[test]
fn up_to_hundred_unique() {
    let mut set = IntegerSet::new();

    let mut values = (1..=100).collect::<Vec<i32>>();
    values.shuffle(&mut rand::rng());

    for (i, val) in values.into_iter().enumerate() {
        set.add(val);
        assert_eq!(set.size(), i as usize + 1);
    }
}
