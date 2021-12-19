// https://leetcode.com/problems/merge-two-sorted-lists/

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

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;
        let mut p1: Option<Box<ListNode>> = list1;
        let mut p2: Option<Box<ListNode>> = list2;
        
        loop {
            match p1.as_mut() {
                None => { break; }
                Some(_) => { /* do nothing */ }
            }

            match p2.as_mut() {
                None => { break; }
                Some(_) => { /* do nothing */ }
            }

            let val1 = p1.as_mut().unwrap().val;
            let val2 = p2.as_mut().unwrap().val;

            if val1 < val2 {
                let next = p1.as_mut().unwrap().next.take();

                tail.as_mut().unwrap().next = p1;
                p1 = next;
            } else {
                let next = p2.as_mut().unwrap().next.take();

                tail.as_mut().unwrap().next = p2;
                p2 = next;
            }

            tail = &mut tail.as_mut().unwrap().next;
        }

        loop {
            match p1.as_mut() {
                None => { break; }
                Some(_) => { /* do nothing */ }
            }

            let next = p1.as_mut().unwrap().next.take();

            tail.as_mut().unwrap().next = p1;
            p1 = next;

            tail = &mut tail.as_mut().unwrap().next;
        }

        loop {
            match p2.as_mut() {
                None => { break; }
                Some(_) => { /* do nothing */ }
            }

            let next = p2.as_mut().unwrap().next.take();

            tail.as_mut().unwrap().next = p2;
            p2 = next;

            tail = &mut tail.as_mut().unwrap().next;
        }

        return dummy.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec1 = vec![1, 2, 4];
        let vec2 = vec![1, 3, 4];
        let ans = vec![1, 1, 2, 3, 4, 4];
        let mut dummy_vec1 = Some(Box::new(ListNode::new(0)));
        let mut dummy_vec2 = Some(Box::new(ListNode::new(0)));
        let mut dummy_ans = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_vec1;

        for &val in vec1.iter() {
            let node = Some(Box::new(ListNode::new(val)));
            tail.as_mut().unwrap().next = node;
            tail = &mut tail.as_mut().unwrap().next;
        }

        tail = &mut dummy_vec2;
        for &val in vec2.iter() {
            let node = Some(Box::new(ListNode::new(val)));
            tail.as_mut().unwrap().next = node;
            tail = &mut tail.as_mut().unwrap().next;
        }

        tail = &mut dummy_ans;
        for &val in ans.iter() {
            let node = Some(Box::new(ListNode::new(val)));
            tail.as_mut().unwrap().next = node;
            tail = &mut tail.as_mut().unwrap().next;
        }

        assert_eq!(Solution::merge_two_lists(dummy_vec1.unwrap().next, dummy_vec2.unwrap().next),
                    dummy_ans.unwrap().next);
    }
}
