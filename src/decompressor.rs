use bitvec::{order::Lsb0, vec::BitVec};

use crate::binarytree::BinaryTree;
use crate::savebits::SaveBits;

pub fn decompress(data: Vec<u8>) -> Vec<u8> {
    let bits: BitVec<Lsb0, u8> = BitVec::from_vec(data);
    let mut it = bits.iter().map(|r| *r);

    let tree = BinaryTree::from_bits(&mut it);
    let lenght = usize::from_bits(&mut it);

    apply_tree(&mut it.take(lenght), &tree)
}

fn apply_tree(data: &mut impl Iterator<Item = bool>, tree: &BinaryTree<u8>) -> Vec<u8> {
    let mut decompressed = Vec::new();

    while let Some(byte) = tree.get_leaf(data) {
        decompressed.push(*byte);
    }

    assert_eq!(None, data.next());

    decompressed
}
