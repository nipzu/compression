use bitvec::{vec::BitVec, order::Lsb0};

use crate::binarytree::BinaryTree;

pub fn apply_tree(data: Vec<u8>, tree: &BinaryTree<u8>) -> Vec<u8> {
    let bits: BitVec<Lsb0, u8> = BitVec::from_vec(data);
    let mut decompressed = Vec::new();

    let mut it = bits.iter().map(|r| *r);

    while let Some(byte) = tree.get_leaf(&mut it) {
        decompressed.push(*byte);
    }

    assert_eq!(None, it.next());

    decompressed
}
