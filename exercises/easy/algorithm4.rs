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
        let res_node = TreeNode {
            value,
            left: None,
            right: None,
        };
        if self.root.is_none() {
            self.root = Some(Box::new(res_node));
        }
        let mut temp_ptr = Box::<TreeNode<T>>::as_ptr(&self.root.unwrap());
        while temp_ptr.is_some() {
            if value <= (*temp_ptr.unwrap()).value {
                if (*temp_ptr.unwrap()).left.is_some() {
                    temp_ptr = (*temp_ptr.unwrap()).left;
                } else {
                    (*temp_ptr.unwrap()).left = Some(Box::new(res_node));
                }
            } else {
                if (*temp_ptr.unwrap()).right.is_some() {
                    temp_ptr = (*temp_ptr.unwrap()).right;
                } else {
                    (*temp_ptr.unwrap()).right = Some(Box::new(res_node));
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut res = false;
        let mut temp_ptr = Box::<TreeNode<T>>::as_ptr(&self.root.unwrap());
        while temp_ptr.is_some() {
            if value == (*temp_ptr.unwrap()).value {
                res == true;
                break;
            } else if value < (*temp_ptr.unwrap()).value {
                temp_ptr = (*temp_ptr.unwrap()).left;
            } else {
                temp_ptr = (*temp_ptr.unwrap()).right;
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
