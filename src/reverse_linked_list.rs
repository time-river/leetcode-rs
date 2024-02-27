// https://leetcode.com/problems/reverse-linked-list

use std::ops::Deref;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        
        let mut curr = head;
        if curr.as_ref().unwrap().next.is_none() {
            return curr;
        }

        let mut prev = None;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            curr = next;
            prev = Some(node);
        }

        prev     
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let vec1 = vec![1,2,3];
        let vec2 = vec![3,2,1];
        let mut input_arr = Some(Box::new(ListNode::new(0)));
        let mut output_arr = Some(Box::new(ListNode::new(0)));

        {
            let mut tail = input_arr.as_mut();
            for &val in vec1.iter() {
                let node = Box::new(ListNode::new(val));
                tail.as_mut().unwrap().next = Some(node);
                tail = tail.unwrap().next.as_mut();
            }
        }
        {
            let mut tail = output_arr.as_mut();
            for &val in vec2.iter() {
                let node = Box::new(ListNode::new(val));
                tail.as_mut().unwrap().next = Some(node);
                tail = tail.unwrap().next.as_mut();
            }
        }

        assert_eq!(Solution::reverse_list(input_arr.unwrap().next), output_arr.unwrap().next);
    }
}
