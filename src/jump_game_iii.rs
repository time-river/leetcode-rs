// https://leetcode.com/problems/jump-game-iii/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut queue: Vec<i32> = Vec::new();
        let mut reached: Vec<bool> = vec![false; arr.len()];

        reached[start as usize] = true;
        queue.push(start);
        while !queue.is_empty() {
            let idx  = queue.pop().unwrap();
            let left = idx - arr[idx as usize];
            let right = idx + arr[idx as usize];

            if arr[idx as usize] == 0 {
                return true;
            }

            if left >= 0 && !reached[left as usize] {
                reached[left as usize] = true;
                queue.insert(queue.len(), left);
            }
            if (right as usize) < arr.len() && !reached[right as usize] {
                reached[right as usize] = true;
                queue.insert(queue.len(), right);
            }
        }

        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::can_reach(vec![4,2,3,0,3,1,2], 5), true);
        assert_eq!(Solution::can_reach(vec![0,3,0,6,3,3,4], 6), true);
    }
}
