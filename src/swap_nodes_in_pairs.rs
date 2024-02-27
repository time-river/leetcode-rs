// https://leetcode.com/problems/swap-nodes-in-pairs/

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut curr = dummy.as_mut();

        loop {
            let next = curr.as_mut().unwrap().next.as_ref();
            if next.is_none() {
                break;
            }
            let nnext = next.unwrap().next.as_ref();
            if nnext.as_ref().is_none() {
                break;
            }

            let mut next = curr.as_mut().unwrap().next.take();
            let mut nnext = next.as_mut().unwrap().next.take();
            let nnnext = nnext.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = nnnext;
            nnext.as_mut().unwrap().next = next;
            curr.as_mut().unwrap().next = nnext;

            curr = curr.unwrap().next.as_mut().unwrap().next.as_mut();
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let vec1 = vec![1,2,3,4];
        let vec2 = vec![2,1,4,3];
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

        assert_eq!(Solution::swap_pairs(input_arr.unwrap().next), output_arr.unwrap().next);
    }

    #[test]
    fn it_works2() {
        let vec1 = vec![1,2,3];
        let vec2 = vec![2,1,3];
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

        assert_eq!(Solution::swap_pairs(input_arr.unwrap().next), output_arr.unwrap().next);   
    }
}