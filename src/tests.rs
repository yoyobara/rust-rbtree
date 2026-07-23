#![cfg(test)]

use rand::seq::SliceRandom;

use crate::set::RBTreeSet;

type IntegerSet = RBTreeSet<i32>;

fn get_set_example() -> IntegerSet {
    let mut set = IntegerSet::new();

    set.add(10);
    set.add(15);
    set.add(11);
    set.add(100);
    set.add(5);
    set.add(2);
    set.add(0);
    set.add(4);

    set
}

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

#[test]
fn iterating() {
    let set = get_set_example();

    for x in &set {
        println!("{}", x);
    }
}
