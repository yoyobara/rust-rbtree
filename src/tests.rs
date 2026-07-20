#![cfg(test)]

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

    for i in 1..=100 {
        set.add(i);

        assert_eq!(set.size(), i as usize);
    }
}
