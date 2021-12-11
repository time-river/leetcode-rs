// https://leetcode.com/problems/add-two-numbers/

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
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut dummy_head;
    let (mut l1, mut l2) = (l1, l2);
    let (mut stop1, mut stop2, mut overflow) = (false, false, false);

    loop {
        let val1 = match l1 {
            Some(node) => {
                l1 = node.next;
                node.val
            }
            None => {
                stop1 = true;
                0
            }
        };

        let val2 = match l2 {
            Some(node) => {
                l2 = node.next;
                node.val
            }
            None => {
                stop2 = true;
                0
            }
        };

        if stop1 && stop2 && !overflow {
            break;
        }

        let sum = val1 + val2 + if overflow { 1 } else { 0 };
        let sum = if sum >= 10 {
            overflow = true;
            sum - 10
        } else {
            overflow = false;
            sum
        };

        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    return dummy_head.unwrap().next;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec1 = vec![9, 9, 9, 9, 9, 9, 9];
        let vec2 = vec![9, 9, 9, 9];
        let result = vec![8, 9, 9, 9, 0, 0, 0, 1];
        let mut dummy_l1 = Some(Box::new(ListNode::new(0)));
        let mut dummy_l2 = Some(Box::new(ListNode::new(0)));
        let mut dummy_l3 = Some(Box::new(ListNode::new(0)));
        let mut tail_l1 = &mut dummy_l1;
        let mut tail_l2 = &mut dummy_l2;
        let mut tail_l3 = &mut dummy_l3;

        for val in vec1.iter() {
            let node = Box::new(ListNode::new(*val));
            tail_l1.as_mut().unwrap().next = Some(node);
            tail_l1 = &mut tail_l1.as_mut().unwrap().next;
        }
        
        for val in vec2.iter() {
            let node = Box::new(ListNode::new(*val));
            tail_l2.as_mut().unwrap().next = Some(node);
            tail_l2 = &mut tail_l2.as_mut().unwrap().next;
        }

        for val in result.iter() {
            let node = Box::new(ListNode::new(*val));
            tail_l3.as_mut().unwrap().next = Some(node);
            tail_l3 = &mut tail_l3.as_mut().unwrap().next;
        }

        assert_eq!(add_two_numbers(dummy_l1.unwrap().next, dummy_l2.unwrap().next),
                    dummy_l3.unwrap().next);
    }
}
