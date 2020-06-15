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
struct Solution {}
impl Solution {
  pub fn search_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let rootOpt = Solution::get_tree_node(root);
    if rootOpt.map(|n| n.val == val).unwrap_or(true) {
      return root;
    }

    let leftOpt = rootOpt.and_then(|r| Solution::get_tree_node(r.left));
    if leftOpt.map(|left| left.val >= val).unwrap_or(false) {
      return Solution::search_bst(rootOpt.map(|r| r.left).unwrap(), val);
    }

    let rightOpt = rootOpt.and_then(|r| Solution::get_tree_node(r.right));
    if root.map(|right| right.val <= val).unwrap_or(false) {
      return Solution::search_bst(rootOpt.map(|r| r.right).unwrap(), val);
    }

    return None;
  }

  pub fn get_tree_node(node: Option<Rc<RefCell<TreeNode>>>) -> Option<TreeNode> {
    return node.map(|r| Rc::try_unwrap(r).map(|rc| rc.into_inner()).unwrap());
  }
}

fn main() {
  println!("Hello, world!");
}
