use crate::savebits::SaveBits;

pub struct BinaryTree<T: Clone> {
    nodes: Vec<BTreeNode<T>>,
    root_node: usize,
}

#[derive(Clone)]
enum BTreeNode<T: Clone> {
    Branch { left: usize, right: usize },
    Leaf { value: T },
}

impl<T: Clone> BinaryTree<T> {
    /// Creates a new tree from a single value which is made the root node of the tree,
    /// Since the tree does not contain any other nodes, the provided value is set to
    /// be a leaf node.
    pub fn new(root_value: T) -> BinaryTree<T> {
        BinaryTree {
            nodes: vec![BTreeNode::Leaf { value: root_value }],
            root_node: 0,
        }
    }

    /// Returns a immutable reference to the root node.
    fn root_node(&self) -> &BTreeNode<T> {
        assert!(!self.nodes.is_empty());
        &self.nodes[self.root_node]
    }

    /// Maps the values in leaf nodes using the provided function.
    pub fn map_values<U: Clone>(self, f: &dyn Fn(T) -> U) -> BinaryTree<U> {
        BinaryTree {
            nodes: self
                .nodes
                .into_iter()
                .map(|node| match node {
                    BTreeNode::Branch { left, right } => BTreeNode::Branch { left, right },
                    BTreeNode::Leaf { value } => BTreeNode::Leaf { value: f(value) },
                })
                .collect(),
            root_node: self.root_node,
        }
    }

    /// Traverses the tree using the provided iterator until a leaf node is found.
    /// Then, the next value from the iterator determines whether the provided value
    /// for the new node is added to the left or right (`true == right`, `false == left`).
    /// The other value at the leaf node is moved to the other direction.
    /// TODO ^ that is confusing af
    pub fn add_leaf(
        &mut self,
        new_value: T,
        iter: &mut impl Iterator<Item = bool>,
    ) -> Option<usize> {
        let mut cur_node = 0;
        let mut depth = 0;
        while let BTreeNode::Branch { left, right } = self.nodes[cur_node] {
            match iter.next() {
                Some(true) => cur_node = right,
                Some(false) => cur_node = left,
                None => return None,
            }
            depth += 1;
        }

        if let BTreeNode::Leaf { value: old_value } = self.nodes[cur_node].clone() {
            self.nodes.push(BTreeNode::Leaf { value: old_value });
            self.nodes.push(BTreeNode::Leaf { value: new_value });
            let idx_old = self.nodes.len() - 2;
            let idx_new = self.nodes.len() - 1;
            self.nodes[cur_node] = match iter.next() {
                Some(true) => BTreeNode::Branch {
                    left: idx_old,
                    right: idx_new,
                },
                Some(false) => BTreeNode::Branch {
                    left: idx_new,
                    right: idx_old,
                },
                None => return None,
            };
            return Some(depth);
        }
        unreachable!();
    }

    /// Returns a reference pointing to a leaf node reached by traversing the tree
    /// as dictated by the provided iterator. If the iterator returns `None` before
    /// reaching a leaf node, `None` is returned.
    pub fn get_leaf(&self, iter: &mut impl Iterator<Item = bool>) -> Option<&T> {
        let mut cur_node = self.root_node();
        while let BTreeNode::Branch { left, right } = cur_node {
            match iter.next() {
                Some(true) => cur_node = &self.nodes[*right],
                Some(false) => cur_node = &self.nodes[*left],
                None => return None,
            }
        }
        if let BTreeNode::Leaf { value } = cur_node {
            return Some(value);
        }
        unreachable!();
    }
    /// Returns an iterator iterating over the tree from left to right. Note that
    /// only leaf nodes are returned.
    pub fn leaves(&self) -> BTreeLeafIter<T> {
        let mut current_branch = vec![(self.root_node(), false)];
        while let Some((BTreeNode::Branch { left, .. }, _)) = current_branch.last() {
            current_branch.push((&self.nodes[*left], false));
        }
        BTreeLeafIter {
            nodes: &self.nodes,
            current_branch,
        }
    }
}

impl<T: Clone + SaveBits> SaveBits for BinaryTree<T> {
    fn save_bits(&self) -> Box<dyn Iterator<Item = bool>> {
        self.save_from_node(self.root_node())
    }

    fn from_bits(iter: &mut dyn Iterator<Item = bool>) -> Self {
        let mut s = BinaryTree {
            nodes: Vec::new(),
            root_node: 0,
        };

        let root_node = s.load_node(iter);

        s.root_node = root_node;

        s
    }
}

impl<T: Clone + SaveBits> BinaryTree<T> {
    fn save_from_node(&self, node: &BTreeNode<T>) -> Box<dyn Iterator<Item = bool>> {
        match node {
            BTreeNode::Branch { left, right } => Box::new(
                std::iter::once(false)
                    .chain(self.save_from_node(&self.nodes[*left]))
                    .chain(self.save_from_node(&self.nodes[*right])),
            ),
            BTreeNode::Leaf { value } => Box::new(std::iter::once(true).chain(value.save_bits())),
        }
    }

    /// WARNING: should probably only be called from BinaryTree::from_bits
    fn load_node(&mut self, iter: &mut dyn Iterator<Item = bool>) -> usize {
        match iter.next() {
            Some(true) => {
                self.nodes.push(BTreeNode::Leaf {
                    value: T::from_bits(iter),
                });
            }
            Some(false) => {
                let left = self.load_node(iter);
                let right = self.load_node(iter);
                self.nodes.push(BTreeNode::Branch { left, right });
            }
            None => panic!("Iterator returned None while loading binary tree"),
        }
        self.nodes.len() - 1
    }
}

/// Iterator over a binary tree. Iterates over the leaf node from left to right.
pub struct BTreeLeafIter<'a, T: Clone> {
    nodes: &'a [BTreeNode<T>],
    current_branch: Vec<(&'a BTreeNode<T>, bool)>,
}

impl<'a, T: Clone> Iterator for BTreeLeafIter<'a, T> {
    type Item = (&'a T, Vec<bool>);
    fn next(&mut self) -> Option<Self::Item> {
        while self.current_branch.last_mut()?.1 {
            self.current_branch.pop();
        }

        match self.current_branch.last_mut() {
            Some((BTreeNode::Branch { right, .. }, visited)) => {
                *visited = true;
                self.current_branch.push((&self.nodes[*right], false))
            }
            // this case *should* only be hit on the first call to .next()
            Some((BTreeNode::Leaf { .. }, _)) => (),
            None => unreachable!(),
        }

        while let Some((BTreeNode::Branch { left, .. }, _)) = self.current_branch.last() {
            self.current_branch.push((&self.nodes[*left], false));
        }

        self.current_branch.last_mut().unwrap().1 = true;
        self.current_branch.last().map(|(node, _)| {
            if let BTreeNode::Leaf { value } = node {
                (
                    value,
                    self.current_branch
                        .iter()
                        .map(|(_, b)| *b)
                        .take(self.current_branch.len() - 1)
                        .collect(),
                )
            } else {
                unreachable!();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //           O
    //         _/ \_
    //       _/     \_
    //      O         O
    //     / \       / \
    //    /   \     /   \
    //   1     O   4     5
    //        / \
    //       /   \
    //      2     3
    fn get_test_tree() -> BinaryTree<u8> {
        BinaryTree::<u8> {
            nodes: vec![
                BTreeNode::Branch { left: 1, right: 2 },
                BTreeNode::Branch { left: 3, right: 4 },
                BTreeNode::Branch { left: 5, right: 6 },
                BTreeNode::Leaf { value: 1 },
                BTreeNode::Branch { left: 7, right: 8 },
                BTreeNode::Leaf { value: 4 },
                BTreeNode::Leaf { value: 5 },
                BTreeNode::Leaf { value: 2 },
                BTreeNode::Leaf { value: 3 },
            ],
            root_node: 0,
        }
    }

    #[test]
    fn test_map_values() {
        let tree = get_test_tree();
        let tree2 = tree.map_values(&|x| x as i16 + 3i16);

        for (leaf, x) in tree2.leaves().zip(4..) {
            assert_eq!(*leaf.0, x);
        }
    }

    #[test]
    fn test_add_leaf() {
        let mut tree = BinaryTree::new(3u8);
        let seq = vec![
            vec![true],
            vec![false, false],
            vec![false, true, false],
            vec![true, true],
        ];
        let mut it = seq.iter().flatten().copied();

        tree.add_leaf(4, &mut it);
        tree.add_leaf(1, &mut it);
        tree.add_leaf(2, &mut it);
        tree.add_leaf(5, &mut it);

        let mut n_leaves = 0;
        for (x1, x2) in tree.leaves().zip(get_test_tree().leaves()) {
            assert_eq!(x1, x2);
            n_leaves += 1;
        }
        assert_eq!(n_leaves, 5);
    }

    #[test]
    fn test_btree_iterator() {
        let tree = get_test_tree();

        assert_eq!(
            tree.leaves().collect::<Vec<(&u8, Vec<bool>)>>(),
            vec![
                (&1, vec![false, false]),
                (&2, vec![false, true, false]),
                (&3, vec![false, true, true]),
                (&4, vec![true, false]),
                (&5, vec![true, true])
            ]
        );
    }

    #[test]
    fn test_get_leaf() {
        let tree = get_test_tree();

        let mut it1 = [false, true, false].iter().copied();
        assert_eq!(tree.get_leaf(&mut it1), Some(&2));
        assert_eq!(it1.next(), None);

        let mut it2 = [false, false, false].iter().copied();
        assert_eq!(tree.get_leaf(&mut it2), Some(&1));
        assert_eq!(it2.next(), Some(false));
        assert_eq!(it2.next(), None);

        let mut it3 = [false, true].iter().copied();
        assert_eq!(tree.get_leaf(&mut it3), None);
        assert_eq!(it3.next(), None);
    }

    #[test]
    fn test_save_load_tree() {
        let tree = get_test_tree();

        let mut n_leaves = 0;
        for (x1, x2) in tree
            .leaves()
            .zip(BinaryTree::<u8>::from_bits(&mut tree.save_bits()).leaves())
        {
            assert_eq!(x1, x2);
            n_leaves += 1;
        }
        assert_eq!(n_leaves, 5);
    }
}
