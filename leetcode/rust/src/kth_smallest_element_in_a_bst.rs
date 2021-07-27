use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution {}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut values = vec![];
        Solution::in_order_traversal(&root, &mut values);
        values[(k - 1) as usize]
    }

    fn in_order_traversal(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if node.is_none() {
            return;
        }

        let node = node.as_ref().unwrap();
        Solution::in_order_traversal(&node.borrow().left, values);
        values.push(node.borrow().val);
        Solution::in_order_traversal(&node.borrow().right, values);
    }
}
