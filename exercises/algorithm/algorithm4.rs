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
        match &mut self.root {
            None => {
                // 树为空，创建一个新的根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(node) => {
                // 树不为空，委托给节点的插入方法
                node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        fn search_in_node<T: Ord>(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
            match node {
                None => false, // 节点为空，值不存在
                Some(boxed_node) => {
                    // 比较当前节点的值与搜索值
                    match value.cmp(&boxed_node.value) {
                        Ordering::Equal => true, // 找到了值
                        Ordering::Less => search_in_node(&boxed_node.left, value), // 值小于当前节点，在左子树搜索
                        Ordering::Greater => search_in_node(&boxed_node.right, value), // 值大于当前节点，在右子树搜索
                    }
                }
            }
        }
        
        search_in_node(&self.root, &value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {
                // 值已存在，二叉搜索树通常不允许重复值
                // 根据测试用例，我们选择不做任何操作
                return;
            }
            Ordering::Less => {
                // 值小于当前节点，插入到左子树
                match &mut self.left {
                    None => {
                        // 如果左子树为空，创建一个新节点
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(left_node) => {
                        // 如果左子树不为空，递归插入
                        left_node.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                // 值大于当前节点，插入到右子树
                match &mut self.right {
                    None => {
                        // 如果右子树为空，创建一个新节点
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(right_node) => {
                        // 如果右子树不为空，递归插入
                        right_node.insert(value);
                    }
                }
            }
        }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}


