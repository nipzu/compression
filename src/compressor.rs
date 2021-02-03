use bitvec::{order::Lsb0, vec::BitVec};

use crate::binarytree::BinaryTree;
use crate::savebits::SaveBits;
use std::collections::HashMap;

pub fn compress(data: &[u8]) -> Vec<u8> {
    let uses = count_uses(data);

    let tree = build_tree(&mut uses.iter().copied());

    let mut map = HashMap::new();

    for (leaf, route) in tree.leaves() {
        map.insert(leaf.1, route);
    }

    let mut compressed: BitVec<Lsb0, u8> = BitVec::new();
    compressed.extend(
        data.iter()
            .map(|byte| map.get(byte).unwrap().iter())
            .flatten(),
    );

    let mut compression_output: BitVec<Lsb0, u8> = BitVec::new();
    compression_output.extend(std::iter::repeat(false).take(3));
    compression_output.extend(tree.map_values(&|(_, byte)| byte).save_bits());
    // compression_output.extend(compressed.len().save_bits());
    compression_output.extend_from_bitslice(compressed.as_bitslice());

    let padding = 8 * compression_output.elements() - compression_output.len();

    *compression_output.get_mut(0).unwrap() = (padding & 1) > 0;
    *compression_output.get_mut(1).unwrap() = (padding & 2) > 0;
    *compression_output.get_mut(2).unwrap() = (padding & 4) > 0;

    compression_output.into_vec()
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
    for (next_count, next_byte) in uses {
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
