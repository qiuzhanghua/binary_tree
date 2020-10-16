// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

///
/// get "Pointer" of a Tree Node
fn to_rc(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => Some(Rc::clone(node)),
        None => None,
    }
}

///
/// get value of a Tree Node
fn val_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            Some(node.val)
        }
        None => None,
    }
}

///
/// get left of a Tree Node
fn left_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            match &node.left {
                Some(l) => Some(Rc::clone(l)),
                None => None,
            }
        }
        None => None,
    }
}

///
/// get right of a Tree Node
fn right_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => {
            let node = node.borrow();
            match &node.right {
                Some(r) => Some(Rc::clone(r)),
                None => None,
            }
        }
        None => None,
    }
}

/// root is not null
/// left is none or left has no child
fn append_to_left(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            ans = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.left = to_rc(&ans);
        }
        None => {}
    }
    ans
}

/// root is not null
/// right is none or right has no child
fn append_to_right(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            ans = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.right = to_rc(&ans);
        }
        None => {}
    }
    ans
}

/// Insert a value to Binary Search Tree
fn insert_bst(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    // insert int binary search tree
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            if node.val > val {
                insert_bst(&mut node.left, val)
            } else if node.val < val {
                insert_bst(&mut node.right, val);
            }
        }
        None => {
            root.replace(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut root = None;
        insert_bst(&mut root, 3);
        assert_eq!(root, Some(Rc::new(RefCell::new(TreeNode::new(3)))));
    }

    #[test]
    fn test_insert_02() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        insert_bst(&mut root, 2);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None
            })))
        );
    }

    #[test]
    fn test_insert_03() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        insert_bst(&mut root, 3);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            })))
        );
    }

    #[test]
    fn test_insert_04() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        insert_bst(&mut root, 2);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            })))
        );
    }

    #[test]
    fn test_to_rc() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut copied = to_rc(&root); // you don't have two instances of TreeNode
        assert_eq!(root, copied);
        insert_bst(&mut copied, 8); // you can think copied as a pointer of origin tree
        assert_eq!(root, copied);
    }
}
