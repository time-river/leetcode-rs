// https://leetcode.com/problems/remove-linked-list-elements/

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

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode{val: 0, next: None}));
        let mut prev = dummy.as_mut();
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take();

            if node.val != val {
                let pnode = prev.unwrap();
                pnode.next = Some(node);
                prev = pnode.next.as_mut();
            }
        }
      
        return dummy.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_element;

    use super::*;

    #[test]
    fn it_works1() {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut dummy_ans = Some(Box::new(ListNode::new(0)));
        let vec1 = vec![1,2,6,3,4,5,6];
        let vec2 = vec![1,2,3,4,5];

        let mut tail = &mut dummy_head;
        for &val in vec1.iter() {
            let node = Box::new(ListNode::new(val));
            tail.as_mut().unwrap().next = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        let mut tail = &mut dummy_ans;
        for &val in vec2.iter() {
            let node = Box::new(ListNode::new(val));
            tail.as_mut().unwrap().next = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        assert_eq!(Solution::remove_elements(dummy_head, 6), dummy_ans);
    }

    #[test]
    fn it_works2() {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut dummy_ans = Some(Box::new(ListNode::new(0)));
        let vec1 = vec![7,7,7,7];
        let vec2 = vec![];

        {
            let mut tail = dummy_head.as_mut().unwrap();
            for &val in vec1.iter() {
                let node = Box::new(ListNode::new(val));
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }
        {
            let mut tail = dummy_ans.as_mut().unwrap();
            for &val in vec2.iter() {
                let node = Box::new(ListNode::new(val));
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }

        assert_eq!(Solution::remove_elements(dummy_head.unwrap().next, 7), dummy_ans.unwrap().next);
    }
}