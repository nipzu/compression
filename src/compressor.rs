use bitvec::{vec::BitVec, order::Lsb0};

use crate::binarytree::BinaryTree;
use std::collections::HashMap;

pub fn compress(data: &[u8]) -> Vec<u8> {
    let uses = count_uses(data);

    let tree = build_tree(&mut uses.iter().copied());

    let mut map = HashMap::new();

    for (leaf, route) in tree.leaves() {
        map.insert(leaf.1, route);
    }

    let mut compressed: BitVec<Lsb0, u8> = BitVec::new();

    compressed.extend(data.iter().map(|byte| map.get(byte).unwrap().iter()).flatten());
    
    // compressed.into_vec()
    let res = compressed.into_vec();

    let x = crate::decompressor::apply_tree(res.clone(), &tree.map_values(&|(_, byte)| byte));

    for byte in x {
        print!("{}", byte as char);
    }
    println!();

    println!();

    println!("{} => {} = {:.2} %", data.len(), res.len(), res.len() as f64 / data.len() as f64 * 100.0 );

    println!();

    res
}

fn count_uses(data: &[u8]) -> Vec<(usize, u8)> {
    let mut num_uses = HashMap::new();
    for byte in data {
        *num_uses.entry(*byte).or_insert(0) += 1;
    }

    let mut uses = Vec::new();
    for (byte, count) in num_uses.iter() {
        uses.push((*count, *byte));
    }
    // sort in descending order
    uses.sort_by(|a, b| b.cmp(&a));
    uses
}

// TODO actually invent some good algorithm
fn build_tree(uses: &mut impl Iterator<Item = (usize, u8)>) -> BinaryTree<(usize, u8)> {
    let mut tree = BinaryTree::new(uses.next().unwrap());
    while let Some((next_count, next_byte)) = uses.next() {
        let route = tree
            .leaves()
            .map(|((count, _), r)| (count + r.len() * next_count, r))
            .min()
            .unwrap()
            .1;
        tree.add_leaf(
            (next_count, next_byte),
            &mut route.iter().copied().chain(std::iter::once(false)),
        );
    }
    tree
}
