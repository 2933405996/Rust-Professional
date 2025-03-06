/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert_recursive(root: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        //TODO
        match root {
            None => Some(Box::new(TreeNode::new(value))),
            Some(mut node) => {
                match node.value.cmp(&value) {
                    Ordering::Less => node.right = Self::insert_recursive(node.right.take(), value),
                    Ordering::Greater => {
                        node.left = Self::insert_recursive(node.left.take(), value)
                    }
                    Ordering::Equal => (),
                }
                Some(node)
            }
        }
    }

    fn search_recursive(root: Option<&Box<TreeNode<T>>>, value: T) -> bool {
        //TODO
        match root {
            None => false,
            Some(mut node) => match node.value.cmp(&value) {
                Ordering::Less => Self::search_recursive(node.right.as_ref(), value),
                Ordering::Greater => Self::search_recursive(node.left.as_ref(), value),
                Ordering::Equal => true,
            },
        }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        self.root = Self::insert_recursive(self.root.take(), value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        Self::search_recursive(self.root.as_ref(), value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
