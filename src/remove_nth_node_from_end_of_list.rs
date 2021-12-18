// https://leetcode.com/probems/remove-nth-node-from-end-of-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[allow(dead_code)]
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut length: i32 = 0;
        
        {
            // `&foo` vs `foo.as_ref()`:
            //   https://www.reddit.com/r/rust/comments/e03piv/why_do_you_have_to_use_as_ref_rather_than_just/
            let mut p: Option<&Box<ListNode>> = head.as_ref();

            loop {
                match p {
                    Some(next) => {
                        length += 1;
                        // p : Option<&Box<ListNode>>
                        // next: &Box<ListNode>
                        // next.next: Option<Box<ListNode>>
                        // next.next.as_ref() : Option<&Box<ListNode>>
                        p = next.next.as_ref();
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        length -= n;

        {
            // &dummy : &Option<Box<ListNode>>
            let mut p: Option<&mut Box<ListNode>> = dummy.as_mut();

            for _ in 0..length {
                // p : Option<&mut Box<ListNode>>
                // p.unwrap() : &mut Box<ListNode>
                // p.unwrap().next : Option<Box<ListNode>>
                // p.unwrap().next.as_mut() : Option<&mut Box<ListNode>>
                p = p.unwrap().next.as_mut();
            }

            // Error example 1:
            //   code:
            //     let cur: Option<&mut Box<ListNode>> = p;
            //     let next: Option<&mut Box<ListNode>> = cur.unwrap().next.as_mut();
            //     cur.unwrap().next = next.unwrap().next.take();
            //
            //   error tips:
            //     Compiling leetcode v0.1.0 (/home/me/leetcode-rs)
            //     error[E0382]: use of moved value: `cur`
            //        --> src/remove_nth_node_from_end_of_list.rs:87:13
            //         |
            //     75  |             let cur: Option<&mut Box<ListNode>> = p;
            //         |                 --- move occurs because `cur` has type `Option<&mut Box<remove_nth_node_from_end_of_list::ListNode>>`, which does not implement the `Copy` trait
            //     ...
            //     82  |             let next: Option<&mut Box<ListNode>> = cur.unwrap().next.as_mut();
            //         |                                                        -------- `cur` moved due to this method call
            //     ...
            //     87  |             cur.unwrap().next = next.unwrap().next.take();
            //         |             ^^^ value used here after move
            //         |
            //     note: this function takes ownership of the receiver `self`, which moves `cur`
            //     help: consider calling `.as_ref()` to borrow the type's contents
            //         |
            //     82  |             let next: Option<&mut Box<ListNode>> = cur.as_ref().unwrap().next.as_mut();
            //         |                                                        +++++++++
            //
            //     For more information about this error, try `rustc --explain E0382`.
            //     error: could not compile `leetcode` due to previous error
            //
            //   reason:
            //     `=` means the ownship tansfer, `cur.unwrap()` moves the `cur` ownership
            //     by `let next = cur.unwrap().next.as_mut();`, so `cur.unwrap().next` is
            //     forbidden.

            // p : Option<&mut Box<ListNode>>
            // p.unwrap() : &mut Box::<ListNode>
            let mut cur: &mut Box<ListNode> = p.unwrap();
            
            // cur : &mut Box<ListNode>
            // cur.next : Option<Box<ListNode>>
            // cur.next.as_mut() : Option<&mut Box<ListNode>>
            // cur.next.as_mut().unwrap() : &mut Box<ListNode>
            let next: &mut Box<ListNode> = cur.next.as_mut().unwrap();
            
            // next.next : Option<Box<ListNode>>
            // next.next.take() : Option<Box<ListNode>>
            // why do we need call `take()` ?
            //   https://stackoverflow.com/questions/57193489/why-do-we-need-to-call-take-for-optiont-variable
            cur.next = next.next.take();

            // `next` will be free automically after the above line, so
            // `println!("{:?}", next);` will result building error.
       }

        println!("ans: {:?}", dummy);
        return dummy.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut dummy_ans = Some(Box::new(ListNode::new(0)));
        let vec1 = vec![1, 2, 3, 4, 5];
        let vec2 = vec![1, 2, 3, 5];

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

        println!("head {:?}", dummy_head);
        println!("ans {:?}", dummy_ans);
        assert_eq!(Solution::remove_nth_from_end(dummy_head.unwrap().next, 2),
                    dummy_ans.unwrap().next);
    }
}
