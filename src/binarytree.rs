pub struct BinaryTree<T> {
    root_node: Box<BTreeNode<T>>,
}

enum BTreeNode<T> {
    Branch {
        left: Box<BTreeNode<T>>,
        right: Box<BTreeNode<T>>,
    },
    Leaf {
        value: T,
    },
}

impl<T> BinaryTree<T> {
    /// Creates a new tree from a single value which is made the root node of the tree,
    /// Since the tree does not contain any other nodes, the provided value is set to
    /// be a leaf node.
    pub fn new(root_value: T) -> BinaryTree<T> {
        BinaryTree {
            root_node: Box::new(BTreeNode::Leaf { value: root_value }),
        }
    }

    /// Traverses the tree using the provided iterator until a leaf node is found.
    /// Then, the next value from the iterator determines whether the provided value 
    /// for the new node is added to the left or right (`true == right`, `false == left`).
    /// The other value at the leaf node is moved to the other direction.
    /// TODO ^ that is confusing af
    /// TODO return value and iter
    pub fn add_leaf(&mut self, new_value: T, iter: &mut impl Iterator<Item = bool>) -> Option<usize> {
        let mut cur_node = &mut self.root_node;
        let mut depth = 0;
        while let BTreeNode::Branch { left, right } = cur_node.as_mut() {
            match iter.next() {
                Some(true) => cur_node = right,
                Some(false) => cur_node = left,
                None => return None,
            }
            depth += 1;
        }
        // if let BTreeNode::Leaf { value: old_value } = *cur_node {
            // let old = cur_node;//Box::new(BTreeNode::Leaf { value: old_value });
            // let new = &mut Box::new(BTreeNode::Leaf { value: new_value });
            // let (left, right) = match iter.next() {
            //     Some(true) => (cur_node, new),
            //     Some(false) => (new, cur_node),
            //     None => return None,
            // };
            // *cur_node = Box::new(BTreeNode::Branch { 
            //     left: *left, right: *right
            // });
            *cur_node = Box::new(BTreeNode::Leaf { value: new_value });
            return Some(depth);
        // } else {
            // unreachable!();
        // }
    }

    /// Returns a reference pointing to a leaf node reached by traversing the tree
    /// as dictated by the provided iterator. If the iterator returns `None` before
    /// reaching a leaf node, `None` is returned.
    pub fn get_leaf(&self, iter: &mut impl Iterator<Item = bool>) -> Option<&T> {
        let mut cur_node = self.root_node.as_ref();
        while let BTreeNode::Branch { left, right } = cur_node {
            match iter.next() {
                Some(true) => cur_node = right,
                Some(false) => cur_node = left,
                None => return None,
            }
        }
        if let BTreeNode::Leaf { value } = cur_node {
            return Some(value);
        } else {
            unreachable!();
        }
    }
    /// Returns an iterator iterating over the tree from left to right. Note that
    /// only leaf nodes are returned.
    pub fn leaves(&self) -> BTreeLeafIter<T> {
        let mut current_branch = vec![(self.root_node.as_ref(), false)];
        while let Some((BTreeNode::Branch { left, .. }, _)) = current_branch.last() {
            current_branch.push((left.as_ref(), false));
        }
        BTreeLeafIter { current_branch }
    }
}

/// Iterator over a binary tree. Iterates over the leaf node from left to right.
pub struct BTreeLeafIter<'a, T> {
    current_branch: Vec<(&'a BTreeNode<T>, bool)>,
}

impl<'a, T> Iterator for BTreeLeafIter<'a, T> {
    type Item = (&'a T, Vec<bool>);
    fn next(&mut self) -> Option<Self::Item> {
        while self.current_branch.last_mut()?.1 {
            self.current_branch.pop();
        }

        match self.current_branch.last_mut() {
            Some((BTreeNode::Branch { right, .. }, visited)) => {
                *visited = true;
                self.current_branch.push((right, false))
            }
            // this case *should* only be hit on the first call to .next()
            Some((BTreeNode::Leaf { .. }, _)) => (),
            None => unreachable!(),
        }

        while let Some((BTreeNode::Branch { left, .. }, _)) = self.current_branch.last() {
            self.current_branch.push((left.as_ref(), false));
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
            root_node: Box::new(BTreeNode::Branch {
                left: Box::new(BTreeNode::Branch {
                    left: Box::new(BTreeNode::Leaf { value: 1 }),
                    right: Box::new(BTreeNode::Branch {
                        left: Box::new(BTreeNode::Leaf { value: 2 }),
                        right: Box::new(BTreeNode::Leaf { value: 3 }),
                    }),
                }),
                right: Box::new(BTreeNode::Branch {
                    left: Box::new(BTreeNode::Leaf { value: 4 }),
                    right: Box::new(BTreeNode::Leaf { value: 5 }),
                }),
            }),
        }
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
}
