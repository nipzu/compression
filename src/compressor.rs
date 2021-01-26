use crate::binarytree::BinaryTree;
use std::collections::{BinaryHeap, HashMap};

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut uses = count_uses(data);

    for (c, b) in uses.drain_sorted() {
        println!("{}: {:?}", c, b as char);
    }

    vec![]
}

fn count_uses(data: &[u8]) -> BinaryHeap<(usize, u8)> {
    let mut num_uses = HashMap::new();
    for byte in data {
        *num_uses.entry(byte).or_insert(0) += 1;
    }

    // TODO build iterator straigth from hashmap?

    let mut heap = BinaryHeap::new();
    for (byte, count) in num_uses {
        heap.push((count, *byte));
    }

    heap
}

fn build_tree(uses: BinaryHeap<(usize, u8)>) -> BinaryTree<u8> {
    unimplemented!()
}
