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

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO

        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }

        let mut temp_ptr = self.root.as_mut().unwrap();
        loop {
            if value < temp_ptr.value {
                match temp_ptr.left {
                    Some(_) => temp_ptr = temp_ptr.left.as_mut().unwrap(),
                    None => {
                        temp_ptr.left = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                }
            } else if value > temp_ptr.value {
                match temp_ptr.right {
                    Some(_) => temp_ptr = temp_ptr.right.as_mut().unwrap(),
                    None => {
                        temp_ptr.right = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if self.root.is_none() {
            return false;
        }
        let mut res = false;
        let mut temp_ptr = self.root.as_ref().unwrap();
        loop {
            if value == temp_ptr.value {
                res = true;
                break;
            } else if value < temp_ptr.value {
                if temp_ptr.left.as_ref().is_none() {
                    break;
                }
                temp_ptr = temp_ptr.left.as_ref().unwrap();
            } else {
                if temp_ptr.right.as_ref().is_none() {
                    break;
                }
                temp_ptr = temp_ptr.right.as_ref().unwrap();
            }
        }
        res
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        self.value = value;
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
