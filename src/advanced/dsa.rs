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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res : Vec<i32> = Vec::new();
        let mut deque: VecDeque<u32> = VecDeque::new();
        if let Some(curr) = root {
            
        }
        while(Some(curr) != None || !deque.is_empty()){
            while(Some(curr) != None){
                deque.push_back(curr.borrow().val);
                curr = curr.borrow().left.clone();
            }
            curr = deque.front();
            res.push(curr.borrow().val);
            curr = curr.borrow().right.clone();
        }

        res
    }
}